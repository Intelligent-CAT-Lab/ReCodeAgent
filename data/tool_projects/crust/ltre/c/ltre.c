#include "ltre.h"
#include <ctype.h>
#include <limits.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define METACHARS "\\.-^$*+?{}[]<>()|&~"
typedef uint8_t symset_t[256 / 8];

// an NFA state. we can assume NFA states have at most two outgoing epsilon
// transitions and at most one outgoing labeled transition without loss of
// generality. so regexes like /./ and /\w/ don't blow out NFA size, we label
// the labeled transition with a set of characters (using `symset_t`) instead
// of with a single character
struct nstate {
  symset_t label;          // set of characters labeling the labeled transition
  struct nstate *target;   // target state for the labeled transition
  struct nstate *epsilon0; // primary epsilon transition, for concatenation
  struct nstate *epsilon1; // secondary epsilon transition, for anything else
  struct nstate *next;     // linked list to keep track of all states of an NFA
  int id;                  // populated and used for various purposes throughout
};

// a DFA state, and maybe an actual DFA too, depending on context. when treated
// as a DFA, the first element of the linked list of states formed by `next` is
// the initial state and subsequent elements enumerate all remaining states
struct dstate {
  struct dstate *transitions[256]; // indexed by input characters
  bool accepting;                  // match result
  bool terminating;                // for early termination
  int id;              // populated and used for various purposes throughout
  struct dstate *next; // linked list to keep track of all states of a DFA
  uint8_t bitset[];    // powerset representation during powerset construction
};

// do not switch out these `unsigned`s for `int`s without a benchmark. on
// x86_64, `unsigned` saves us a `cdqe` instruction in the fragment `id / 8`
// and speeds up the hot loop in `dfa_step` by 10%
static bool bitset_get(uint8_t bitset[], unsigned id) {
  return bitset[id / 8] & 1 << id % 8;
}

static void bitset_set(uint8_t bitset[], unsigned id) {
  bitset[id / 8] |= 1 << id % 8;
}

static char *symset_fmt(symset_t symset) {
  // output shall be parsable by `parse_symset` and satisfy the invariant
  // `parse_symset . symset_fmt == id`. in the general case, will not satisfy
  // `symset_fmt . parse_symset == id`

  static char buf[1024], nbuf[1024];
  char *bufp = buf, *nbufp = nbuf;
  // number of symsets in `buf` and `nbuf` character classes, respectively
  int nsym = 0, nnsym = 0;

  *nbufp++ = '^';
  *bufp++ = *nbufp++ = '[';

  for (int chr = 0; chr < 256; chr++) {
  append_chr:
    bitset_get(symset, chr) ? nsym++ : nnsym++;
    char **p = bitset_get(symset, chr) ? &bufp : &nbufp;
    bool is_metachar = chr && strchr(METACHARS, chr);
    if (!isprint(chr) && !is_metachar)
      *p += sprintf(*p, "\\x%02hhx", chr);
    else {
      if (is_metachar)
        *(*p)++ = '\\';
      *(*p)++ = chr;
    }

    // make character ranges
    int start = chr;
    while (chr < 255 && bitset_get(symset, chr) == bitset_get(symset, chr + 1))
      chr++;
    if (chr - start >= 2)
      *(*p)++ = '-', bitset_get(symset, chr) ? nsym-- : nnsym--;
    if (chr - start >= 1)
      goto append_chr;
  }

  *bufp++ = *nbufp++ = ']';
  *bufp++ = *nbufp++ = '\0';

  // special casees for character classes containing zero or one symsets
  if (nnsym == 0) {
    return "<>";
  } else if (nsym == 1) {
    bufp[-2] = '\0';
    return buf + 1;
  } else if (nnsym == 1) {
    nbufp[-2] = '\0', nbuf[1] = '^';
    return nbuf + 1;
  }

  // return a complemented character class if it is shorter
  return (bufp - buf < nbufp - nbuf) ? buf : nbuf;
}

static struct nstate *nstate_alloc(void) {
  struct nstate *nstate = malloc(sizeof(struct nstate));
  *nstate = (struct nstate){.id = -1};
  return nstate;
}

void nfa_free(struct nfa nfa) {
  for (struct nstate *next, *nstate = nfa.initial; nstate; nstate = next)
    next = nstate->next, free(nstate);
}

static int nfa_get_size(struct nfa nfa) {
  // also populates `nstate.id` with unique identifiers
  int nfa_size = 0;
  for (struct nstate *nstate = nfa.initial; nstate; nstate = nstate->next)
    if ((nstate->id = nfa_size++) == INT_MAX)
      abort();
  return nfa_size;
}

static struct nfa nfa_clone(struct nfa nfa) {
  int nfa_size = nfa_get_size(nfa);

  struct nstate *nstates[nfa_size];
  for (int id = 0; id < nfa_size; id++)
    nstates[id] = nstate_alloc();

#define MAYBE_COPY(FIELD)                                                      \
  if (nstate->FIELD)                                                           \
    nstates[nstate->id]->FIELD = nstates[nstate->FIELD->id];

  for (struct nstate *nstate = nfa.initial; nstate; nstate = nstate->next) {
    memcpy(nstates[nstate->id]->label, nstate->label, sizeof(symset_t));
    MAYBE_COPY(target);
    MAYBE_COPY(epsilon0);
    MAYBE_COPY(epsilon1);
    MAYBE_COPY(next);
  }

#undef MAYBE_COPY

  return (struct nfa){
      .initial = nstates[nfa.initial->id],
      .final = nstates[nfa.final->id],
      .complemented = nfa.complemented,
  };
}

static void nfa_concat(struct nfa *nfap, struct nfa nfa) {
  // memcpys `nfa.initial` into `nfap->final` then frees `nfa.initial`; assumes
  // nothing refers to `nfa.initial` and assumes `nfap->final` refers to
  // nothing. performs a "visual" concatenation and therefore does not take into
  // account the `nfa.complemented` flag. if `nfa` or `nfap` have it set, the
  // result may not be what you expect
  if (nfap->initial == nfap->final)
    nfa_free(*nfap), *nfap = nfa;
  else if (nfa.initial != nfa.final) {
    memcpy(nfap->final, nfa.initial, sizeof(struct nstate));
    nfap->final = nfa.final;
    free(nfa.initial);
  }
}

static void nfa_pad_initial(struct nfa *nfa) {
  struct nstate *initial = nstate_alloc();
  initial->epsilon0 = nfa->initial;
  initial->next = nfa->initial;
  nfa->initial = initial;
}

static void nfa_pad_final(struct nfa *nfa) {
  struct nstate *final = nstate_alloc();
  nfa->final->epsilon0 = final;
  nfa->final->next = final;
  nfa->final = final;
}

static void nfa_uncomplement(struct nfa *nfa) {
  // ensures `!nfa->complemented`, a useful property when manipulating NFAs.
  // if `nfa->complemented`, we have to go through the whole compile pipeline
  // to uncomplement it
  if (!nfa->complemented)
    return;
  struct dstate *dfa = ltre_compile(*nfa);
  struct nfa uncomplemented = ltre_uncompile(dfa);
  dfa_free(dfa), nfa_free(*nfa);
  memcpy(nfa, &uncomplemented, sizeof(struct nfa));
}

void nfa_dump(struct nfa nfa) {
  (void)nfa_get_size(nfa);

  printf("graph LR\n");
  printf("  I( ) --> %d\n", nfa.initial->id);
  printf("  %d --> F( )\n", nfa.final->id);

  for (struct nstate *nstate = nfa.initial; nstate; nstate = nstate->next) {
    if (nstate->epsilon0)
      printf("  %d --> %d\n", nstate->id, nstate->epsilon0->id);
    if (nstate->epsilon1)
      printf("  %d --> %d\n", nstate->id, nstate->epsilon1->id);

    bool empty = true;
    for (int i = 0; i < sizeof(symset_t); i++)
      empty &= !nstate->label[i];

    if (empty)
      continue;

    // avoid breaking Mermaid
    printf("  %d --", nstate->id);
    for (char *fmt = symset_fmt(nstate->label); *fmt; fmt++)
      printf(strchr("\\\"#&{}()xo=- ", *fmt) ? "#%hhu;" : "%c", *fmt);
    printf("--> %d\n", nstate->target->id);
  }
}

static struct dstate *dstate_alloc(int bitset_size) {
  struct dstate *dstate = malloc(sizeof(struct dstate) + bitset_size);
  *dstate = (struct dstate){.id = -1};
  memset(dstate->bitset, 0x00, bitset_size);
  return dstate;
}

void dfa_free(struct dstate *dstate) {
  for (struct dstate *next; dstate; dstate = next)
    next = dstate->next, free(dstate);
}

static int dfa_get_size(struct dstate *dfa) {
  // also populates `dstate.id` with unique identifiers
  int dfa_size = 0;
  for (struct dstate *dstate = dfa; dstate; dstate = dstate->next)
    if ((dstate->id = dfa_size++) == INT_MAX)
      abort();
  return dfa_size;
}

static void leb128_put(uint8_t **p, int n) {
  while (n >> 7)
    *(*p)++ = (n & 0x7f) | 0x80, n >>= 7;
  *(*p)++ = n;
}

static int leb128_get(uint8_t **p) {
  int n = 0, c = 0;
  do
    n |= (**p & 0x7f) << c++ * 7;
  while (*(*p)++ & 0x80);

  return n;
}

uint8_t *dfa_serialize(struct dstate *dfa, size_t *size) {
  // serialize a DFA using a mix of RLE and LEB128. `size` is an out parameter

  int dfa_size = dfa_get_size(dfa);

  // len(leb128(dfa_size)) == ceil(log128(dfa_size))
  int ceil_log128 = 0;
  for (int s = dfa_size; s >>= 7;)
    ceil_log128++;
  ceil_log128++;

  uint8_t *buf = malloc(ceil_log128), *p = buf;
  leb128_put(&p, dfa_size);

  for (struct dstate *dstate = dfa; dstate; dstate = dstate->next) {
    // ensure buffer large enough for worst case. worst case is typically around
    // 500 bytes larger than best case, so this is not too wasteful.
    ptrdiff_t len = p - buf;
    // len + <accepting_terminating> + 256 * (<run_length> + <leb128(dfa_size)>)
    uint8_t *new = realloc(buf, len + 1 + 256 * (1 + ceil_log128));
    buf = new, p = new + len;

    *p++ = dstate->accepting << 1 | dstate->terminating;
    for (int chr = 0; chr < 256;) {
      int start = chr;
      while (chr < 255 &&
             dstate->transitions[chr] == dstate->transitions[chr + 1])
        chr++;
      *p++ = chr - start; // run length
      leb128_put(&p, dstate->transitions[chr++]->id);
    }
  }

  *size = p - buf;
  return realloc(buf, p - buf); // don't be wasteful
}

struct dstate *dfa_deserialize(uint8_t *buf, size_t *size) {
  // deserialize a DFA from a **trusted** buffer. `size` is an out parameter

  uint8_t *p = buf;
  int dfa_size = leb128_get(&p);

  struct dstate *dstates[dfa_size];
  for (int id = 0; id < dfa_size; id++)
    dstates[id] = dstate_alloc(0);

  for (int id = 0; id < dfa_size; id++) {
    dstates[id]->accepting = *p >> 1 & 1;
    dstates[id]->terminating = *p++ & 1;
    for (int chr = 0; chr < 256;) {
      int len = *p++;
      struct dstate *target = dstates[leb128_get(&p)];
      do // run length
        dstates[id]->transitions[chr++] = target;
      while (len--);
    }

    if (id != 0)
      dstates[id - 1]->next = dstates[id];
  }

  *size = p - buf;
  return *dstates;
}

void dfa_dump(struct dstate *dfa) {
  (void)dfa_get_size(dfa);

  printf("graph LR\n");
  printf("  I( ) --> %d\n", dfa->id);

  for (struct dstate *ds1 = dfa; ds1; ds1 = ds1->next) {
    if (ds1->accepting)
      printf("  %d --> F( )\n", ds1->id);

    for (struct dstate *ds2 = dfa; ds2; ds2 = ds2->next) {
      bool empty = true;
      symset_t transitions = {0};
      for (int chr = 0; chr < 256; chr++)
        if (ds1->transitions[chr] == ds2)
          bitset_set(transitions, chr), empty = false;

      if (empty)
        continue;

      // avoid breaking Mermaid
      printf("  %d --", ds1->id);
      for (char *fmt = symset_fmt(transitions); *fmt; fmt++)
        printf(strchr("\\\"#&{}()xo=- ", *fmt) ? "#%hhu;" : "%c", *fmt);
      printf("--> %d\n", ds2->id);
    }
  }
}

// some invariants for parsers on parse error:
// - `error` shall be set to a non-`NULL` error message
// - `regex` shall point to the error location
// - the returned NFA shall be the null NFA
// - the caller is responsible for backtracking

static unsigned parse_natural(char **regex, char **error) {
  if (!isdigit(**regex)) {
    *error = "expected natural number";
    return 0;
  }

  unsigned natural = 0;
  for (; isdigit(**regex); ++*regex) {
    int digit = **regex - '0';

    if (natural > UINT_MAX / 10 || natural * 10 > UINT_MAX - digit) {
      *error = "natural number overflow";
      return UINT_MAX; // indicate overflow condition
    }

    natural *= 10, natural += digit;
  }
  return natural;
}

static uint8_t parse_hexbyte(char **regex, char **error) {
  uint8_t byte = 0;
  for (int i = 0; i < 2; i++) {
    byte <<= 4;
    char chr = **regex;
    if (isdigit(chr))
      byte |= chr - '0';
    else if (isxdigit(chr))
      byte |= tolower(chr) - 'a' + 10;
    else {
      *error = "expected hex digit";
      return 0;
    }
    ++*regex;
  }
  return byte;
}

static uint8_t parse_escape(char **regex, char **error) {
  if (strchr(METACHARS, **regex))
    return *(*regex)++;

  switch (*(*regex)++) {
  case 'a':
    return '\a';
  case 'b':
    return '\b';
  case 'f':
    return '\f';
  case 'n':
    return '\n';
  case 'r':
    return '\r';
  case 't':
    return '\t';
  case 'v':
    return '\v';
  case 'x':;
    uint8_t chr = parse_hexbyte(regex, error);
    if (*error)
      return 0;
    return chr;
  }

  --*regex;
  *error = "unknown escape";
  return 0;
}

static uint8_t parse_symbol(char **regex, char **error) {
  if (**regex == '\\') {
    ++*regex;
    uint8_t escape = parse_escape(regex, error);
    if (*error)
      return 0;

    return escape;
  }

  if (**regex == '\0') {
    *error = "expected symbol";
    return 0;
  }

  if (strchr(METACHARS, **regex)) {
    *error = "unexpected metacharacter";
    return 0;
  }

  if (!isprint(**regex)) {
    *error = "unexpected nonprintable character";
    return 0;
  }

  return *(*regex)++;
}

static void parse_shorthand(symset_t symset, char **regex, char **error) {
  memset(symset, 0x00, sizeof(symset_t));

#define RETURN_SYMSET(COND)                                                    \
  for (int chr = 0; chr < 256; chr++)                                          \
    if (COND)                                                                  \
      bitset_set(symset, chr);                                                 \
  return;

  if (**regex == '\\') {
    ++*regex;

    switch (*(*regex)++) {
    case 'd':
      RETURN_SYMSET(isdigit(chr))
    case 'D':
      RETURN_SYMSET(!isdigit(chr))
    case 's':
      RETURN_SYMSET(isspace(chr))
    case 'S':
      RETURN_SYMSET(!isspace(chr))
    case 'w':
      RETURN_SYMSET(chr == '_' || isalnum(chr))
    case 'W':
      RETURN_SYMSET(chr != '_' && !isalnum(chr))
    }

    --*regex, --*regex;
  }

  if (**regex == '.') {
    ++*regex;
    RETURN_SYMSET(chr != '\n')
  }

#undef RETURN_SYMSET

  *error = "expected shorthand class";
  return;
}

static void parse_symset(symset_t symset, char **regex, char **error) {
  bool complement = false;
  if (**regex == '^')
    ++*regex, complement = true;

  char *last_regex = *regex;
  parse_shorthand(symset, regex, error);
  if (!*error)
    goto process_complement;
  *error = NULL;
  *regex = last_regex;

  if (**regex == '[') {
    ++*regex;

    memset(symset, 0x00, sizeof(symset_t));
    // hacky lookahead for better diagnostics
    while (!strchr("]", **regex)) {
      symset_t sub;
      parse_symset(sub, regex, error);
      if (*error)
        return;

      for (int i = 0; i < sizeof(symset_t); i++)
        symset[i] |= sub[i];
    }

    if (**regex != ']') {
      *error = "expected ']'";
      return;
    }

    ++*regex;
    goto process_complement;
  }
  *regex = last_regex;

  if (**regex == '<') {
    ++*regex;

    memset(symset, 0xff, sizeof(symset_t));
    // hacky lookahead for better diagnostics
    while (!strchr(">", **regex)) {
      symset_t sub;
      parse_symset(sub, regex, error);
      if (*error)
        return;

      for (int i = 0; i < sizeof(symset_t); i++)
        symset[i] &= sub[i];
    }

    if (**regex != '>') {
      *error = "expected '>'";
      return;
    }

    ++*regex;
    goto process_complement;
  }
  *regex = last_regex;

  uint8_t begin = parse_symbol(regex, error);
  if (!*error) {
    uint8_t end = begin;
    if (**regex == '-') {
      ++*regex;
      end = parse_symbol(regex, error);
      if (*error)
        return;
    }

    end++; // open upper bound
    memset(symset, 0x00, sizeof(symset_t));
    uint8_t chr = begin;
    do
      bitset_set(symset, chr);
    while (++chr != end);
    goto process_complement;
  }
  return;

process_complement:
  if (complement)
    for (int i = 0; i < sizeof(symset_t); i++)
      symset[i] = ~symset[i];
  return;
}

static struct nfa parse_regex(char **regex, char **error);
static struct nfa parse_atom(char **regex, char **error) {
  if (**regex == '(') {
    ++*regex;
    struct nfa sub = parse_regex(regex, error);
    if (*error)
      return (struct nfa){NULL};

    if (**regex != ')') {
      *error = "expected ')'";
      nfa_free(sub);
      return (struct nfa){NULL};
    }

    ++*regex;
    return sub;
  }

  struct nfa chars = {.initial = nstate_alloc(),
                      .final = nstate_alloc(),
                      .complemented = false};
  chars.initial->next = chars.final;
  chars.initial->target = chars.final;

  parse_symset(chars.initial->label, regex, error);
  if (*error) {
    nfa_free(chars);
    return (struct nfa){NULL};
  }

  return chars;
}

static struct nfa parse_factor(char **regex, char **error) {
  struct nfa atom = parse_atom(regex, error);
  if (*error)
    return (struct nfa){NULL};

  //         <---
  // -->O-->(atom)-->O-->
  //     ----------->
  if (**regex == '*') {
    ++*regex;
    nfa_uncomplement(&atom);
    atom.final->epsilon1 = atom.initial;
    nfa_pad_initial(&atom), nfa_pad_final(&atom);
    atom.initial->epsilon1 = atom.final;
    return atom;
  }

  //         <---
  // -->O-->(atom)-->O-->
  if (**regex == '+') {
    ++*regex;
    nfa_uncomplement(&atom);
    atom.final->epsilon1 = atom.initial;
    nfa_pad_initial(&atom), nfa_pad_final(&atom);
    return atom;
  }

  // -->(atom)-->
  //     --->
  if (**regex == '?') {
    ++*regex;
    nfa_uncomplement(&atom);
    if (atom.initial->epsilon1)
      nfa_pad_initial(&atom);
    atom.initial->epsilon1 = atom.final;
    return atom;
  }

  char *last_regex = *regex;
  if (**regex == '{') {
    ++*regex;
    nfa_uncomplement(&atom);
    unsigned min = parse_natural(regex, error);
    if (*error && min == UINT_MAX) { // overflow condition
      nfa_free(atom);
      return (struct nfa){NULL};
    } else if (*error)
      min = 0, *error = NULL;

    unsigned max = min;
    bool max_unbounded = false;
    if (**regex == ',') {
      ++*regex;
      max = parse_natural(regex, error);
      if (*error && max == UINT_MAX) { // overflow condition
        nfa_free(atom);
        return (struct nfa){NULL};
      } else if (*error)
        max_unbounded = true, *error = NULL;
    }

    if (**regex != '}') {
      *error = "expected '}'";
      nfa_free(atom);
      return (struct nfa){NULL};
    }
    ++*regex;

    if (min > max && !max_unbounded) {
      *regex = last_regex;
      *error = "misbounded quantifier";
      nfa_free(atom);
      return (struct nfa){NULL};
    }

    struct nfa atoms = {.complemented = false};
    atoms.initial = atoms.final = nstate_alloc();

    // if `max` is bounded, make `max` copies and add `?` epsilon transitions:
    // -->(atom)...(atom)(atom)-->
    //              --->  --->
    // if it is not, make `min + 1` copies and add one `*` epsilon transition:
    //                       <---
    // -->...(atom)(atom)-->(atom)-->O-->
    //                   ----------->
    for (unsigned i = 0; max_unbounded ? i <= min : i < max; i++) {
      struct nfa clone = nfa_clone(atom);
      if (i >= min) {
        if (max_unbounded) {
          clone.final->epsilon1 = clone.initial;
          nfa_pad_initial(&clone), nfa_pad_final(&clone);
        }
        clone.initial->epsilon1 = clone.final;
      }

      nfa_concat(&atoms, clone);

      // needed for when `min == UINT_MAX && max_unbounded`. this isn't a bodge;
      // the correct number of copies will have been made by the time we break
      if (i == UINT_MAX)
        break;
    }

    nfa_free(atom);

    return atoms;
  }

  return atom;
}

static struct nfa parse_term(char **regex, char **error) {
  bool complement = false;
  if (**regex == '~')
    ++*regex, complement = true;

  struct nfa term = {.complemented = false};
  term.initial = term.final = nstate_alloc();

  // hacky lookahead for better diagnostics
  while (!strchr(")|&", **regex)) {
    struct nfa factor = parse_factor(regex, error);
    if (*error) {
      nfa_free(term);
      return (struct nfa){NULL};
    }

    nfa_uncomplement(&factor);
    nfa_concat(&term, factor);
  }

  if (complement)
    term.complemented = true;

  return term;
}

static struct nfa parse_regex(char **regex, char **error) {
  struct nfa re = parse_term(regex, error);
  if (*error)
    return (struct nfa){NULL};

  while (**regex == '|' || **regex == '&') {
    bool intersect = *(*regex)++ == '&';
    struct nfa alt = parse_term(regex, error);
    if (*error) {
      nfa_free(re);
      return (struct nfa){NULL};
    }

    // we perform NFA intersection by rewriting into an alternation using De
    // Morgan's law `a&b == ~(~a|~b)`. this isn't nearly as inefficient as it
    // may appear, because NFA complementation is performed lazily
    re.complemented ^= intersect, alt.complemented ^= intersect;
    nfa_uncomplement(&re), nfa_uncomplement(&alt);

    // -->O-->(re)--->
    //     -->(alt)-->O-->
    nfa_pad_initial(&re), nfa_pad_final(&alt);
    re.initial->epsilon1 = alt.initial;
    re.final->epsilon0 = alt.final;
    re.final->next = alt.initial;
    re.final = alt.final;

    re.complemented ^= intersect;
  }

  return re;
}

struct nfa ltre_parse(char **regex, char **error) {
  // returns a null NFA on error; `*regex` will point to the error location and
  // `*error` will be set to an error message. `error` may be set to `NULL` to
  // disable error reporting

  // don't write to `*regex` or `*error` if error reporting is disabled
  char *e, *r = *regex;
  if (error == NULL)
    error = &e, regex = &r;

  *error = NULL;
  struct nfa nfa = parse_regex(regex, error);
  if (*error)
    return (struct nfa){NULL};

  if (**regex != '\0') {
    *error = "expected end of input";
    nfa_free(nfa);
    return (struct nfa){NULL};
  }

  return nfa;
}

struct nfa ltre_fixed_string(char *string) {
  // parses a fixed string into an NFA. never errors

  struct nfa nfa = {.complemented = false};
  nfa.initial = nfa.final = nstate_alloc();

  for (; *string; string++) {
    struct nstate *initial = nfa.final;
    nfa.final = nstate_alloc();
    initial->next = nfa.final;
    initial->target = nfa.final;
    bitset_set(initial->label, (uint8_t)*string);
  }

  return nfa;
}

void ltre_partial(struct nfa *nfa) {
  // enable partial matching. effectively, surround the NFA by a pair of `<>*`s
  nfa_uncomplement(nfa);
  nfa_pad_initial(nfa), nfa_pad_final(nfa);
  nfa->initial->target = nfa->initial;
  nfa->final->target = nfa->final;
  memset(nfa->initial->label, 0xff, sizeof(symset_t));
  memset(nfa->final->label, 0xff, sizeof(symset_t));
}

void ltre_ignorecase(struct nfa *nfa) {
  // enable case-insensitive matching. effectively, for any character a labeled
  // transition contains, make it also contain its swapped-case counterpart
  nfa_uncomplement(nfa);
  for (struct nstate *nstate = nfa->initial; nstate; nstate = nstate->next) {
    for (int chr = 0; chr < 256; chr++) {
      if (bitset_get(nstate->label, chr)) {
        bitset_set(nstate->label, tolower(chr));
        bitset_set(nstate->label, toupper(chr));
      }
    }
  }
}

void ltre_complement(struct nfa *nfa) {
  // complement accepted language. `dfa_step` will read this flag when marking
  // accepting states
  nfa->complemented = !nfa->complemented;
}

static void epsilon_closure(struct nstate *nstate, uint8_t bitset[]) {
  if (!nstate)
    return;

  if (bitset_get(bitset, nstate->id))
    return; // already visited

  bitset_set(bitset, nstate->id);
  epsilon_closure(nstate->epsilon0, bitset);
  epsilon_closure(nstate->epsilon1, bitset);
}

static void dfa_step(struct dstate **dfap, struct dstate *dstate, uint8_t chr,
                     struct nfa nfa, int nfa_size, struct nstate *nstates[]) {
  // step the DFA `*dfap` starting from state `dstate` by consuming input
  // character `chr` according to the NFA `nfa`. `nstates` shall expose the
  // states of the NFA `nfa` as an array of state pointers of length `nfa_size`.
  // this routine creates new DFA states as needed. call initially with `dstate
  // == NULL` to create a DFA state corresponding to the epsilon-closure of the
  // NFA's initial state

  int bitset_size = (nfa_size + 7) / 8; // ceil
  uint8_t bitset_union[bitset_size];
  memset(bitset_union, 0x00, bitset_size);

  if (dstate) {
    // compute the "superposition" of NFA states reachable by consuming `chr`.
    // using the array of state pointers `nstates` speeds up this hot loop by
    // 2.5x over iterating through the states linked list using `nstate->next`,
    // probably because it helps out the prefetches and breaks the memory load
    // dependency chain
    for (int id = 0; id < nfa_size; id++)
      if (bitset_get(dstate->bitset, id) && bitset_get(nstates[id]->label, chr))
        epsilon_closure(nstates[id]->target, bitset_union);
  } else
    epsilon_closure(nfa.initial, bitset_union);

  // create a DFA state whose `bitset` corresponds to this "superposition",
  // if it doesn't already exist. binary tree not necessary, linear search
  // is just as fast
  struct dstate **dstatep = dfap;
  while (*dstatep && memcmp((*dstatep)->bitset, bitset_union, bitset_size))
    dstatep = &(*dstatep)->next;

  if (!*dstatep) {
    *dstatep = dstate_alloc(bitset_size);
    memcpy((*dstatep)->bitset, bitset_union, bitset_size);

    // a DFA state is accepting if and only if it contains the NFA's final state
    // in its `bitset`. also make sure to swap accepting and non-accepting
    // states if the NFA is complemented
    (*dstatep)->accepting = bitset_get(bitset_union, nfa.final->id);
    (*dstatep)->accepting ^= nfa.complemented;
  }

  if (dstate)
    // patch the `chr` transition to point to this new (or existing) state
    dstate->transitions[chr] = *dstatep;
}

struct dstate *ltre_compile(struct nfa nfa) {
  // fully compile DFA. powerset construction followed by DFA minimization

  int nfa_size = nfa_get_size(nfa);
  struct nstate *nstates[nfa_size];
  for (struct nstate *nstate = nfa.initial; nstate; nstate = nstate->next)
    nstates[nstate->id] = nstate;

  // construct new DFA states as we're iterating over them and patching
  // transitions, starting from the epsilon-closure of the NFA's initial state
  struct dstate *dfa = NULL;
  dfa_step(&dfa, NULL, 0, nfa, nfa_size, nstates);
  for (struct dstate *dstate = dfa; dstate; dstate = dstate->next)
    for (int chr = 0; chr < 256; chr++)
      dfa_step(&dfa, dstate, chr, nfa, nfa_size, nstates);

  int dfa_size = dfa_get_size(dfa);
  struct dstate *dstates[dfa_size];
  for (struct dstate *dstate = dfa; dstate; dstate = dstate->next)
    dstates[dstate->id] = dstate;

  // store distinguishability data in a symmetric matrix condensed using bitsets
  uint8_t dis[dfa_size][(dfa_size + 7) / 8]; // ceil
  memset(dis, 0x00, sizeof(dis));
#define ARE_DIS(id1, id2) bitset_get(dis[id1], id2)
#define MAKE_DIS(id1, id2) bitset_set(dis[id1], id2), bitset_set(dis[id2], id1)

  // flag indistinguishable states. a pair of states is indistinguishable if and
  // only if both states have the same `accepting` value and their transitions
  // are equal up to target state indistinguishability. to avoid dealing with
  // cycles, we default to all states being indistinguishable then iteratively
  // rule out the ones that aren't.
  for (struct dstate *ds1 = dfa; ds1; ds1 = ds1->next)
    for (struct dstate *ds2 = ds1->next; ds2; ds2 = ds2->next)
      if (ds1->accepting != ds2->accepting)
        MAKE_DIS(ds1->id, ds2->id);
  for (bool done = false; done = !done;)
    for (int id1 = 0; id1 < dfa_size; id1++)
      for (int id2 = id1 + 1; id2 < dfa_size; id2++)
        if (!ARE_DIS(id1, id2))
          for (int chr = 0; chr < 256; chr++)
            // use irreflexivity of distinguishability as a cheap precheck
            if (dstates[id1]->transitions[chr] !=
                dstates[id2]->transitions[chr])
              if (ARE_DIS(dstates[id1]->transitions[chr]->id,
                          dstates[id2]->transitions[chr]->id)) {
                MAKE_DIS(id1, id2), done = false;
                break;
              }

  // minimize the DFA by merging indistinguishable states. no need to prune
  // unreachable states because the powerset construction yields a DFA with
  // no unreachable states
  for (struct dstate *ds1 = dfa; ds1; ds1 = ds1->next) {
    for (struct dstate *prev = ds1; prev; prev = prev->next) {
      for (struct dstate *ds2; ds2 = prev->next;) {
        if (ARE_DIS(ds1->id, ds2->id))
          break;

        // states are indistinguishable. merge them
        for (struct dstate *dstate = dfa; dstate; dstate = dstate->next)
          for (int chr = 0; chr < 256; chr++)
            if (dstate->transitions[chr] == ds2)
              dstate->transitions[chr] = ds1;

        prev->next = ds2->next, free(ds2);
      }
    }

    // flag "terminating" states. a terminating state is a state which either
    // always or never leads to an accepting state. since `ds1` is now
    // distinguishable from all other states, it is terminating if and only if
    // all its transitions point to itself because, by definition, no other
    // state accepts the same set of words as it does (either all or none)
    ds1->terminating = true;
    for (int chr = 0; chr < 256; chr++)
      if (ds1->transitions[chr] != ds1)
        ds1->terminating = false;
  }

  // nfa_dump(nfa);
  // dfa_dump(dfa);
  // printf("%2d -> %2d\n", dfa_size, dfa_get_size(dfa));

  return dfa;
}

bool ltre_matches(struct dstate *dfa, uint8_t *input) {
  // time linear in the input length :)
  for (; !dfa->terminating && *input; input++)
    dfa = dfa->transitions[*input];
  return dfa->accepting;
}

bool ltre_matches_lazy(struct dstate **dfap, struct nfa nfa, uint8_t *input) {
  // Thompson's algorithm. lazily create new DFA states as we need them. cached
  // DFA states are stored in `*dfap`. call initially with an empty cache using
  // `*dfap == NULL`, and make sure to `dfa_free(*dfap)` when finished with
  // this NFA

  int nfa_size = nfa_get_size(nfa);
  struct nstate *nstates[nfa_size];
  for (struct nstate *nstate = nfa.initial; nstate; nstate = nstate->next)
    nstates[nstate->id] = nstate;

  dfa_step(dfap, NULL, 0, nfa, nfa_size, nstates);

  // time linear in the input length :)
  struct dstate *dstate = *dfap;
  for (; *input; dstate = dstate->transitions[*input++])
    if (!dstate->transitions[*input])
      dfa_step(dfap, dstate, *input, nfa, nfa_size, nstates);

  return dstate->accepting;
}

struct nfa ltre_uncompile(struct dstate *dfa) {
  int dfa_size = dfa_get_size(dfa);

  struct nfa nfa = {.initial = nstate_alloc(),
                    .final = nstate_alloc(),
                    .complemented = false};
  struct nstate *tail = nfa.initial; // used to allocate new NFA states

  struct nstate *nstates[dfa_size]; // mapping from DFA state to NFA state
  for (int id = 0; id < dfa_size; id++)
    nstates[id] = tail->next = nstate_alloc(), tail = tail->next;

  nfa.initial->epsilon1 = nstates[dfa->id];
  // use `epsilon1` transitions to `nfa.final` to model accepting states
  for (struct dstate *dstate = dfa; dstate; dstate = dstate->next)
    if (dstate->accepting)
      nstates[dstate->id]->epsilon1 = nfa.final;

  // the labeled transitions of a `dstate` may target multiple different states,
  // but that of an `nstate` may only target one, namely `nstate.target`. to
  // bridge the gap, DFA states are mapped not to a single NFA state, but to the
  // root of a binary tree of NFA states formed by `epsilon0` and `epsilon1`.
  for (struct dstate *ds1 = dfa; ds1; ds1 = ds1->next) {
    struct nstate *free = NULL;

    for (struct dstate *ds2 = dfa; ds2; ds2 = ds2->next) {
      bool empty = true;
      symset_t transitions = {0};
      for (int chr = 0; chr < 256; chr++)
        if (ds1->transitions[chr] == ds2)
          bitset_set(transitions, chr), empty = false;

      if (empty)
        continue;

      struct nstate *src; // the "source" state for this labeled transition

      // root->O-->O-->O-->O->free-->
      //   |   |   |   |   |   |
      //   O   O   O   O   O   O
      if (free == NULL)
        // first iteration. make sure the root of the tree comes from `nstates`
        free = nstates[ds1->id], src = free;
      else {
        src = tail->next = nstate_alloc(), tail = tail->next;

        // maintain that `free` is an `nstate` with at least one unused epsilon
        // transition. if `epsilon1` is unused then use it to point to our
        // source state, but stay where we are because `epsilon0` is still
        // unused. otherwise, `epsilon0` is the only unused epsilon transition,
        // so use it to point to our source state and move over because the
        // current state has become saturated. the resulting binary tree is
        // unbalanced, but generating it is easy and it is still twice as
        // efficient as a linked list would be
        if (!free->epsilon1)
          free->epsilon1 = tail;
        else
          free->epsilon0 = tail, free = tail;
      }

      src->target = nstates[ds2->id];
      memcpy(src->label, transitions, sizeof(symset_t));
    }
  }

  tail->next = nfa.final;

  // dfa_dump(dfa);
  // nfa_dump(nfa);
  // printf("%2d -> %2d\n", dfa_size, nfa_get_size(nfa));

  return nfa;
}

char *ltre_decompile(struct dstate *dfa) {
  // we codegen the DFA straight into a regular expression string. start by
  // turning the DFA into a GNFA stored as a matrix of `arrow`s on the stack,
  // each of which holds a regular expression string as label and a precedence
  // value for parenthesizing
  enum prec {
    ALT,    // outermost is an alternation
    CONCAT, // outermost is a concatenation
    QUANT,  // outermost is a quantifier
    SYMSET, // outermost is a symset
  };
  struct arrow {
    // we use a `NULL` label as a shorthand for empty /[]/ transitions an an
    // empty string as a shorthand for epsilon /()/ transitions. this way we can
    // test for /[]/ with `if (!label)` and test for /()/ with `if (!*label)`
    char *label;
    enum prec prec;
  };

  int dfa_size = dfa_get_size(dfa);
  // also create an auxiliary state and store it at index `dfa_size`
  struct arrow arrows[dfa_size + 1][dfa_size + 1];
  for (int id = 0; id <= dfa_size; id++)
    arrows[dfa_size][id].label = arrows[id][dfa_size].label = NULL;

  // create an epsilon /()/ transition from the auxiliary state to the DFA's
  // initial state
  arrows[dfa_size][dfa->id].label = malloc(1);
  *arrows[dfa_size][dfa->id].label = '\0';
  arrows[dfa_size][dfa->id].prec = SYMSET;
  for (struct dstate *ds1 = dfa; ds1; ds1 = ds1->next) {
    // create epsilon /()/ transitions from the DFA's accepting states to the
    // auxiliary state
    if (ds1->accepting) {
      arrows[ds1->id][dfa_size].label = malloc(1);
      *arrows[ds1->id][dfa_size].label = '\0';
      arrows[ds1->id][dfa_size].prec = SYMSET;
    }

    for (struct dstate *ds2 = dfa; ds2; ds2 = ds2->next) {
      bool empty = true;
      symset_t transitions = {0};
      for (int chr = 0; chr < 256; chr++)
        if (ds1->transitions[chr] == ds2)
          bitset_set(transitions, chr), empty = false;

      arrows[ds1->id][ds2->id].label = NULL;

      if (empty)
        continue;

      char *fmt = symset_fmt(transitions);
      arrows[ds1->id][ds2->id].label = malloc(strlen(fmt) + 1);
      strcpy(arrows[ds1->id][ds2->id].label, fmt);
      arrows[ds1->id][ds2->id].prec = SYMSET;
    }
  }

  // iteratively select one state according to some heuristic and reroute all
  // its inbound and outbound transitions so as to bypass that state, while also
  // taking care of self-loops. don't ever select the auxiliary state, so that
  // when we're done the final regular expression ends up as a self-loop on the
  // auxiliary state. choosing the state with minimal vertex degree seems to
  // work okay, sa that's the heuristic we're using
  while (1) {
    int best_fit;
    int min_degree = INT_MAX;
    for (int id1 = 0; id1 < dfa_size; id1++) {
      int degree = 0;
      // double-counts self-loops, which penalizes them, which is probably good
      for (int id2 = 0; id2 < dfa_size; id2++)
        degree +=
            (arrows[id1][id2].label != NULL) + (arrows[id2][id1].label != NULL);
      if (degree == 0)
        continue; // state has already been processed
      if (degree < min_degree)
        min_degree = degree, best_fit = id1;
    }

    if (min_degree == INT_MAX)
      break; // all states have been processed

    // iterate through all pairs of inbound and outbound transitions, including
    // those to and from the auxiliary state
    for (int id1 = 0; id1 <= dfa_size; id1++) {
      if (id1 == best_fit)
        continue; // inbound transition must not be a self-loop
      for (int id2 = 0; id2 <= dfa_size; id2++) {
        if (id2 == best_fit)
          continue; // outbound transition must not be a self-loop
        struct arrow in = arrows[id1][best_fit];  // current inbound transition
        struct arrow out = arrows[best_fit][id2]; // current outbout transition
        struct arrow self = arrows[best_fit][best_fit]; // self-transition
        struct arrow existing = arrows[id1][id2]; // existing bypass transition

        // (existing)|[](self)(out) == (existing)|(in)(self)[] == (existing)
        if (!in.label || !out.label)
          continue;

        // get rid of the self-transition by concatenating it either onto the
        // the inbound or outbound transition
        struct arrow first, second;

        ptrdiff_t diff;
        if (!self.label || !*self.label) {
          // (in)[]*(out) == (in)()*(out) == (in)(out)
          first = in, second = out;
        } else if (in.prec >= CONCAT && self.prec >= CONCAT &&
                   (diff = strlen(in.label) - strlen(self.label)) >= 0 &&
                   strcmp(in.label + diff, self.label) == 0) {
          // hackily try to avoid breaking apart symsets in the inbound arrow
          if (diff >= 1 && strchr("^-\\", in.label[diff - 1]) &&
              (diff == 1 || in.label[diff - 2] != '\\'))
            goto nevermind; // preserve ^aa* a-zz* \^a^a* but not \^aa* \-aa*
          if (diff >= 2 && strncmp(&in.label[diff - 2], "\\x", 2) == 0 &&
              (diff == 2 || in.label[diff - 3] != '\\'))
            goto nevermind; // preserve \x0a(0a)* but not \\x0a(0a*)
          if (diff >= 3 && strncmp(&in.label[diff - 3], "\\x", 2) == 0 &&
              (diff == 3 || in.label[diff - 4] != '\\'))
            goto nevermind; // preserve \x0aa* but not \\x0aa*

          // (in_pre)(self)+(out) where (in) == (in_pre)(self)
          first.label = malloc(strlen(in.label) + 5 + 1);
          char *p = first.label;
          if (diff != 0 && in.prec < CONCAT)
            *p++ = '(';
          memcpy(p, in.label, diff), p += diff;
          if (diff != 0 && in.prec < CONCAT)
            *p++ = ')';
          if (self.prec <= QUANT)
            *p++ = '(';
          strcpy(p, self.label), p += strlen(self.label);
          if (self.prec <= QUANT)
            *p++ = ')';
          *p++ = '+';
          *p++ = '\0';
          first.prec = CONCAT;
          second = out;
        } else
        nevermind:
          if (out.prec >= CONCAT && self.prec >= CONCAT &&
              (diff = strlen(out.label) - strlen(self.label)) >= 0 &&
              strncmp(out.label, self.label, strlen(self.label)) == 0) {
            // it may appear that `a*a-z` would be incorrectly broken apart and
            // converted to `a+-z`, but this will never happen because we're
            // decompiling a DFA and therefore if there is a self-loop on `a`,
            // there can be no outbound transitions on `a`

            // (in)(self)+(out_post) where (out) == (self)(out_post)
            second.label = malloc(strlen(out.label) + 5 + 1);
            char *p = second.label;
            if (self.prec <= QUANT)
              *p++ = '(';
            strcpy(p, self.label), p += strlen(self.label);
            if (self.prec <= QUANT)
              *p++ = ')';
            *p++ = '+';
            if (diff != 0 && out.prec < CONCAT)
              *p++ = '(';
            memcpy(p, out.label + diff, diff), p += diff;
            if (diff != 0 && out.prec < CONCAT)
              *p++ = ')';
            *p++ = '\0';
            second.prec = CONCAT;
            first = in;
          } else {
            // (in)(self)*(out)
            second.label =
                malloc(strlen(self.label) + strlen(out.label) + 5 + 1);
            char *p = second.label;
            if (self.prec <= QUANT)
              *p++ = '(';
            strcpy(p, self.label), p += strlen(self.label);
            if (self.prec <= QUANT)
              *p++ = ')';
            *p++ = '*';
            if (out.prec < CONCAT)
              *p++ = '(';
            strcpy(p, out.label), p += strlen(out.label);
            if (out.prec < CONCAT)
              *p++ = ')';
            *p++ = '\0';
            second.prec = CONCAT;
            first = in;
          }

        // concatenate the inbound and outbound transitions to create a
        // transition that bypasses the best-fit state
        struct arrow bypass;

        if (!*first.label) {
          // ()(second) == (second)
          bypass = second;
        } else if (!*second.label) {
          // (first)() == (first)
          bypass = first;
        } else {
          // (first)(second)
          bypass.label =
              malloc(strlen(first.label) + strlen(second.label) + 4 + 1);
          char *p = bypass.label;
          if (first.prec < CONCAT)
            *p++ = '(';
          strcpy(p, first.label), p += strlen(first.label);
          if (first.prec < CONCAT)
            *p++ = ')';
          if (second.prec < CONCAT)
            *p++ = '(';
          strcpy(p, second.label), p += strlen(second.label);
          if (second.prec < CONCAT)
            *p++ = ')';
          *p++ = '\0';
          bypass.prec = CONCAT;
        }

        // if there already exists a transition between the states we're
        // targetting, merge it with our bypass transition using an alternation
        struct arrow merged;

        if (!bypass.label) {
          // (existing)|[] == (existing)
          merged = existing;
        } else if (!existing.label) {
          // []|(bypass) == (bypass)
          if (bypass.label == first.label || bypass.label == second.label) {
            merged.label = malloc(strlen(bypass.label) + 1);
            strcpy(merged.label, bypass.label);
            merged.prec = bypass.prec;
          } else
            merged = bypass;
        } else if (!*existing.label) {
          // ()|(bypass) == (bypass)?
          merged.label = malloc(strlen(bypass.label) + 3 + 1);
          char *p = merged.label;
          if (bypass.prec <= QUANT)
            *p++ = '(';
          strcpy(p, bypass.label), p += strlen(bypass.label);
          if (bypass.prec <= QUANT)
            *p++ = ')';
          *p++ = '?';
          *p++ = '\0';
          merged.prec = QUANT;
        } else {
          // (existing)|(bypass)
          merged.label =
              malloc(strlen(existing.label) + strlen(bypass.label) + 1 + 1);
          char *p = merged.label;
          strcpy(p, existing.label), p += strlen(existing.label);
          *p++ = '|';
          strcpy(p, bypass.label), p += strlen(bypass.label);
          *p++ = '\0';
          merged.prec = ALT;
        }

        if (first.label != in.label)
          free(first.label);
        if (second.label != out.label)
          free(second.label);
        if (bypass.label != first.label && bypass.label != second.label &&
            bypass.label != merged.label)
          free(bypass.label);
        if (existing.label != merged.label)
          free(existing.label);

        // replace the existing bypass transition with our merged transition
        arrows[id1][id2] = merged;
      }
    }

    // all transitions going through the best-fit state have been rerouted, so
    // replace them all with empty /[]/ transitions, effectively eliminating
    // the state by isolating it
    for (int id = 0; id <= dfa_size; id++) {
      free(arrows[id][best_fit].label), arrows[id][best_fit].label = NULL;
      free(arrows[best_fit][id].label), arrows[best_fit][id].label = NULL;
    }
  }

  // the final regular expression ends up as a self-loop on the auxiliary state
  char *regex = arrows[dfa_size][dfa_size].label;
  if (!regex)
    regex = malloc(3), strcpy(regex, "[]");

  return regex;
}
