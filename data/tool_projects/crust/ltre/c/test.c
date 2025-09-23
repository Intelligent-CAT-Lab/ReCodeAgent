#include "ltre.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct test {
  char *regex;
  char *input;
  bool matches;
  bool errors;
  bool partial;
  bool ignorecase;
  bool complement;
  bool quick;
};

void test(struct test args) {
#define test(...) test((struct test){__VA_ARGS__})
  static struct test memo = {0};
  static struct nfa nfa = {NULL};
  static struct dstate *dfa = NULL, *ldfa = NULL;

  if (dfa && strcmp(memo.regex, args.regex) == 0 &&
      memcmp(&memo.errors, &args.errors, sizeof(bool[5])) == 0)
    goto check_matches;
  memo = args;

  char *error = NULL, *loc = args.regex;
  nfa_free(nfa), nfa = ltre_parse(&loc, &error);

  if (!!error != args.errors)
    printf("test failed: /%s/ parse\n", args.regex);
  // if (error)
  //   printf("note: /%s/ %s near '%.16s'\n", args.regex, error, loc);

  if (error)
    return;

  if (args.partial)
    ltre_partial(&nfa);
  if (args.ignorecase)
    ltre_ignorecase(&nfa);
  if (args.complement)
    ltre_complement(&nfa);

  // NFA -> DFA
  dfa_free(dfa), dfa = ltre_compile(nfa);

  // DFA -> BUF -> DFA -> NFA -> DFA
  uint8_t *buf = dfa_serialize(dfa, &(size_t){0});
  dfa_free(dfa), dfa = dfa_deserialize(buf, &(size_t){0});
  nfa_free(nfa), nfa = ltre_uncompile(dfa);
  dfa_free(dfa), dfa = ltre_compile(nfa);
  free(buf);

  if (!args.quick) {
    // DFA -> RE -> NFA -> DFA
    char *re = ltre_decompile(dfa);
    nfa_free(nfa), nfa = ltre_parse(&re, NULL);
    dfa_free(dfa), dfa = ltre_compile(nfa);
    free(re);
  }

  dfa_free(ldfa), ldfa = NULL;

check_matches:
  if (ltre_matches(dfa, (uint8_t *)args.input) != args.matches ||
      ltre_matches_lazy(&ldfa, nfa, (uint8_t *)args.input) != args.matches)
    printf("test failed: /%s/ against '%s'\n", args.regex, args.input);
}

int main(void) {
  // catastrophic backtracking
  test("(a*)*c", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false);
  test("(x+x+)+y", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", false);

  // determinization state blowout
  test("[01]*1[01]{8}", "11011100011100", true, .quick = true);
  test("[01]*1[01]{8}", "01010010010010", false, .quick = true);

  // powerset construction state blowout
  test(".*0.*|.*1.*|.*2.*|.*3.*|.*4.*|.*5.*", "", false);
  test(".*0.*|.*1.*|.*2.*|.*3.*|.*4.*|.*5.*", "123", true);

  // potential edge cases
  test("abba", "abba", true);
  test("[ab]+", "abba", true);
  test("[ab]+", "abc", false);
  test(".*", "abba", true);
  test("(a|b+){3}", "abbba", true);
  test("(a|b+){3}", "abbab", false);
  test("\\x61\\+", "a+", true);
  test("", "", true);
  test("[]", "", false);
  test("[]*", "", true);
  test("[]+", "", false);
  test("[]?", "", true);
  test("()", "", true);
  test("()*", "", true);
  test("()+", "", true);
  test("()?", "", true);
  test(" ", " ", true);
  test("", "\n", false);
  test("\\n", "\n", true);
  test(".", "\n", false);
  test("\\\\n", "\n", false);
  test("(|n)(\\n)", "\n", true);
  test("\\r?\\n", "\n", true);
  test("\\r?\\n", "\r\n", true);
  test("(a*)*", "a", true);
  test("(a+)+", "aa", true);
  test("(a?)?", "", true);
  test("a+", "aa", true);
  test("a?", "aa", false);
  test("(a+)?", "aa", true);
  test("(ba+)?", "baa", true);
  test("(ab+)?", "b", false);
  test("(a+b)?", "a", false);
  test("(a+a+)+", "a", false);
  test("a+", "", false);
  test("(a+|)+", "aa", true);
  test("(a+|)+", "", true);
  test("(a|b)?", "a", true);
  test("(a|b)?", "b", true);
  test("x*|", "xx", true);
  test("x*|", "", true);
  test("x+|", "xx", true);
  test("x+|", "", true);
  test("x?|", "x", true);
  test("x?|", "", true);
  test("x*y*", "yx", false);
  test("x+y+", "yx", false);
  test("x?y?", "yx", false);
  test("x+y*", "xyx", false);
  test("x*y+", "yxy", false);
  test("x*|y*", "xy", false);
  test("x+|y+", "xy", false);
  test("x?|y?", "xy", false);
  test("x+|y*", "xy", false);
  test("x*|y+", "xy", false);
  test("a{1,2}", "", false);
  test("a{1,2}", "a", true);
  test("a{1,2}", "aa", true);
  test("a{1,2}", "aaa", false);
  test("a{0,}", "", true);
  test("a{0,}", "a", true);
  test("a{0,}", "aa", true);
  test("a{0,}", "aaa", true);
  test("a{1,}", "", false);
  test("a{1,}", "a", true);
  test("a{1,}", "aa", true);
  test("a{1,}", "aaa", true);
  test("a{3,}", "aa", false);
  test("a{3,}", "aaa", true);
  test("a{3,}", "aaaa", true);
  test("a{3,}", "aaaaa", true);
  test("a{0,2}", "", true);
  test("a{0,2}", "a", true);
  test("a{0,2}", "aa", true);
  test("a{0,2}", "aaa", false);
  test("a{2}", "a", false);
  test("a{2}", "aa", true);
  test("a{2}", "aaa", false);
  test("a{0}", "", true);
  test("a{0}", "a", false);

  // partial, ignorecase, complement
  test("", "", true, .partial = true);
  test("", "abc", true, .partial = true);
  test("b", "abc", true, .partial = true);
  test("ba", "abc", false, .partial = true);
  test("abc", "abc", true, .partial = true);
  test("[]", "", false, .partial = true);
  test("", "", true, .ignorecase = true);
  test("abCdEF", "aBCdEf", true, .ignorecase = true);
  test("ab", "abc", false, .ignorecase = true);
  test("a", "", true, .complement = true);
  test("a", "aa", true, .complement = true);
  test("a", "a", false, .complement = true);
  test("ab*", "ac", true, .complement = true);
  test("ab*", "abb", false, .complement = true);

  // decompilation edge cases
  test("^aa*", "ba", true);
  test("a-zz*", "abc", false);
  test("\\x0a(0a)*", "\x0a", true);
  test("\\x0aa*", "\x0a\x0a", false);

  // parse errors
  test("abc]", .errors = true);
  test("[abc", .errors = true);
  test("abc)", .errors = true);
  test("(abc", .errors = true);
  test("+a", .errors = true);
  test("a|*", .errors = true);
  test("\\x0", .errors = true);
  test("\\zzz", .errors = true);
  test("[a\\x]", .errors = true);
  test("\b", .errors = true);
  test("\t", .errors = true);
  test("^^a", .errors = true);
  test("a**", .errors = true);
  test("a*+", .errors = true);
  test("a*?", .errors = true);
  test("a+*", .errors = true);
  test("a++", .errors = true);
  test("a+?", .errors = true);
  test("a?*", .errors = true);
  test("a?+", .errors = true);
  test("a??", .errors = true);
#define NAT_OVF "9999999999999999999999999999999999999999"
  test("a{" NAT_OVF "}", .errors = true);
  test("a{" NAT_OVF ",}", .errors = true);
  test("a{," NAT_OVF "}", .errors = true);
  test("a{" NAT_OVF "," NAT_OVF "}", .errors = true);

  // nonstandard features
  test("^a", "z", true);
  test("^a", "a", false);
  test("^\\n", "\r", true);
  test("^\\n", "\n", false);
  test("^.", "\n", true);
  test("^.", "a", false);
  test("\\d+", "0123456789", true);
  test("\\s+", " \f\n\r\t\v", true);
  test("\\w+", "azAZ09_", true);
  test("^a-z*", "1A!2$B", true);
  test("^a-z*", "1aA", false);
  test("a-z*", "abc", true);
  test("^[\\d^\\w]+", "abcABC", true);
  test("^[\\d^\\w]+", "abc123", false);
  test("^[\\d\\W]+", "abcABC", true);
  test("^[\\d^\\W]+", "abc123", false);
  test("[[abc]]+", "abc", true);
  test("[a[bc]]+", "abc", true);
  test("[a[b]c]+", "abc", true);
  test("[a][b][c]", "abc", true);
  test("^[^a^b]", "a", false);
  test("^[^a^b]", "b", false);
  test("^[^a^b]", "", false);
  test("<ab>", "a", false);
  test("<ab>", "b", false);
  test("<ab>", "", false);
  test("\\^", "^", true);
  test("^\\^", "^", false);
  test("^[^\\^]", "^", true);
  test("^[ ^[a b c]]+", "abc", true);
  test("^[ ^[a b c]]+", "a c", false);
  test("<[a b c]^ >+", "abc", true);
  test("<[a b c]^ >+", "a c", false);
  test("^[^0-74]+", "0123567", true);
  test("^[^0-74]+", "89", false);
  test("^[^0-74]+", "4", false);
  test("<0-7^4>+", "0123567", true);
  test("<0-7^4>+", "89", false);
  test("<0-7^4>+", "4", false);
  test("[]", " ", false);
  test("^[]", " ", true);
  test("<>", " ", true);
  test("^<>", " ", false);
  test("9-0*", "abc", true);
  test("9-0*", "18", false);
  test("9-0*", "09", true);
  test("9-0*", "/:", true);
  test("b-a*", "ab", true);
  test("a-b*", "ab", true);
  test("a-a*", "ab", false);
  test("a-a*", "aa", true);
  test("a{,2}", "", true);
  test("a{,2}", "a", true);
  test("a{,2}", "aa", true);
  test("a{,2}", "aaa", false);
  test("a{}", "", true);
  test("a{}", "a", false);
  test("a{,}", "", true);
  test("a{,}", "a", true);
  test("~0*", "", false);
  test("~0*", "0", false);
  test("~0*", "00", false);
  test("~0*", "001", true);
  test("ab&cd", "", false);
  test("ab&cd", "ab", false);
  test("ab&cd", "cd", false);
  test("\\w+&~\\d+", "", false);
  test("\\w+&~\\d+", "abc", true);
  test("\\w+&~\\d+", "abc123", true);
  test("\\w+&~\\d+", "1a2b3c", true);
  test("\\w+&~\\d+", "123", false);
  test("0x(~[0-9a-f]+)", "0yz", false);
  test("0x(~[0-9a-f]+)", "0x12", false);
  test("0x(~[0-9a-f]+)", "0x", true);
  test("0x(~[0-9a-f]+)", "0xy", true);
  test("0x(~[0-9a-f]+)", "0xyz", true);
  test("b(~a*)", "", false);
  test("b(~a*)", "b", false);
  test("b(~a*)", "ba", false);
  test("b(~a*)", "bbaa", true);
  test("abc>", .errors = true);
  test("<abc", .errors = true);
  test("[a?b]", .errors = true);
  test("[a-]", .errors = true);
  test("[--]", .errors = true);
  test("[-]", .errors = true);
  test("-", .errors = true);
  test("a-", .errors = true);
  test("a*{}", .errors = true);
  test("a+{}", .errors = true);
  test("a?{}", .errors = true);
  test("a{}*", .errors = true);
  test("a{}+", .errors = true);
  test("a{}?", .errors = true);
  test("a{}{}", .errors = true);
  test("a{2,1}", .errors = true);
  test("a{1 2}", .errors = true);
  test("a{1, 2}", .errors = true);
  test("a{a}", .errors = true);
  test("a~b", .errors = true);

  // realistic regexes
#define HEX_RGB "#([0-9a-fA-F]{3}){1,2}"
  test(HEX_RGB, "000", false);
  test(HEX_RGB, "#0aA", true);
  test(HEX_RGB, "#00ff", false);
  test(HEX_RGB, "#abcdef", true);
  test(HEX_RGB, "#abcdeff", false);
  // ISO/IEC 9899:TC3, $7.19.6.1 'The fprintf function'.
  // see also gcc-14/gcc/c-family/c-format.cc:713 'print_char_table'
  // and gcc-14/gcc/c-family/c-format.h:25 'enum format_lengths'
#define FIELD_WIDTH "(\\*|1-90-9*)?"
#define PRECISION "(\\.|\\.\\*|\\.1-90-9*)?"
#define DI "[\\-\\+ 0]*" FIELD_WIDTH PRECISION "([hljzt]|hh|ll)?[di]"
#define U "[\\-0]*" FIELD_WIDTH PRECISION "([hljzt]|hh|ll)?u"
#define OX "[\\-#0]*" FIELD_WIDTH PRECISION "([hljzt]|hh|ll)?[oxX]"
#define FEGA "[\\-\\+ #0]*" FIELD_WIDTH PRECISION "[lL]?[fFeEgGaA]"
#define C "\\-*" FIELD_WIDTH "l?c"
#define S "\\-*" FIELD_WIDTH PRECISION "l?s"
#define P "\\-*" FIELD_WIDTH "p"
#define N FIELD_WIDTH "([hljzt]|hh|ll)?n"
#define CONV_SPEC "%(" DI "|" U "|" OX "|" FEGA "|" C "|" S "|" P "|" N "|%)"
  test(CONV_SPEC, "%", false);
  test(CONV_SPEC, "%*", false);
  test(CONV_SPEC, "%%", true);
  test(CONV_SPEC, "%5%", false);
  test(CONV_SPEC, "%p", true);
  test(CONV_SPEC, "%*p", true);
  test(CONV_SPEC, "% *p", false);
  test(CONV_SPEC, "%5p", true);
  test(CONV_SPEC, "d", false);
  test(CONV_SPEC, "%d", true);
  test(CONV_SPEC, "%.16s", true);
  test(CONV_SPEC, "% 5.3f", true);
  test(CONV_SPEC, "%*32.4g", false);
  test(CONV_SPEC, "%-#65.4g", true);
  test(CONV_SPEC, "%03c", false);
  test(CONV_SPEC, "%06i", true);
  test(CONV_SPEC, "%lu", true);
  test(CONV_SPEC, "%hhu", true);
  test(CONV_SPEC, "%Lu", false);
  test(CONV_SPEC, "%-*p", true);
  test(CONV_SPEC, "%-.*p", false);
  test(CONV_SPEC, "%id", false);
  test(CONV_SPEC, "%%d", false);
  test(CONV_SPEC, "i%d", false);
  test(CONV_SPEC, "%c%s", false);
  test(CONV_SPEC, "%0n", false);
  test(CONV_SPEC, "% u", false);
  test(CONV_SPEC, "%+c", false);
  test(CONV_SPEC, "%0-++ 0i", true);
  test(CONV_SPEC, "%30c", true);
  test(CONV_SPEC, "%03c", false);
#define PRINTF_FMT "(^%|" CONV_SPEC ")*"
  test(PRINTF_FMT, "%", false, .quick = true);
  test(PRINTF_FMT, "%*", false, .quick = true);
  test(PRINTF_FMT, "%%", true, .quick = true);
  test(PRINTF_FMT, "%5%", false, .quick = true);
  test(PRINTF_FMT, "%id", true, .quick = true);
  test(PRINTF_FMT, "%%d", true, .quick = true);
  test(PRINTF_FMT, "i%d", true, .quick = true);
  test(PRINTF_FMT, "%c%s", true, .quick = true);
  test(PRINTF_FMT, "%u + %d", true, .quick = true);
  test(PRINTF_FMT, "%d:", true, .quick = true);
  // ISO/IEC 9899:TC3, $6.4.1 'Keywords', $6.4.2 'Identifiers' and $6.4.3
  // 'Universal character names'. does not uphold $6.4.3 paragraph 2
#define KEYWORD                                                                \
  "(auto|break|case|char|const|continue|default|do|double|else|enum|extern|"   \
  "float|for|goto|if|inline|int|long|register|restrict|return|short|signed|"   \
  "sizeof|static|struct|switch|typedef|union|unsigned|void|volatile|while|"    \
  "_Bool|_Complex|_Imaginary)"
#define HEX_QUAD "[0-9a-fA-F]{4}"
#define IDENTIFIER                                                             \
  "(\\w|\\\\u" HEX_QUAD "|\\\\U" HEX_QUAD HEX_QUAD ")*&~\\d.*&~" KEYWORD
  test(IDENTIFIER, "_", true);
  test(IDENTIFIER, "_foo", true);
  test(IDENTIFIER, "_Bool", false);
  test(IDENTIFIER, "a1", true);
  test(IDENTIFIER, "5b", false);
  test(IDENTIFIER, "if", false);
  test(IDENTIFIER, "ifa", true);
  test(IDENTIFIER, "bif", true);
  test(IDENTIFIER, "if2", true);
  test(IDENTIFIER, "1if", false);
  test(IDENTIFIER, "\\u12", false);
  test(IDENTIFIER, "\\u1A2b", true);
  test(IDENTIFIER, "\\u1234", true);
  test(IDENTIFIER, "\\u123x", false);
  test(IDENTIFIER, "\\u1234x", true);
  test(IDENTIFIER, "\\U12345678", true);
  test(IDENTIFIER, "\\U1234567y", false);
  test(IDENTIFIER, "\\U12345678y", true);
  // RFC 8259, $7 'Strings'. the RFC requires that UTF-8 be used for open
  // exchange of JSON; this regex lets through any byte outside 7-bit ASCII,
  // and so really accepts a superset of UTF-8-encoded JSON strings. also, the
  // RFC points out that the grammar would allow string values that are invalid
  // Unicode, but doesn't elaborate, so I guess we can let those through too?
#define HEX_QUAD "[0-9a-fA-F]{4}"
#define JSON_STR                                                               \
  "\"(^[\\x00-\\x1f\"\\\\]|\\\\[\"\\\\/bfnrt]|\\\\u" HEX_QUAD ")*\""
  test(JSON_STR, "foo", false);
  test(JSON_STR, "\"foo", false);
  test(JSON_STR, "foo \"bar\"", false);
  test(JSON_STR, "\"foo\\\"", false);
  test(JSON_STR, "\"\\\"", false);
  test(JSON_STR, "\"\"\"", false);
  test(JSON_STR, "\"\"", true);
  test(JSON_STR, "\"foo\"", true);
  test(JSON_STR, "\"foo\\\"\"", true);
  test(JSON_STR, "\"foo\\\\\"", true);
  test(JSON_STR, "\"\\nbar\"", true);
  test(JSON_STR, "\"\nbar\"", false);
  test(JSON_STR, "\"\\abar\"", false);
  test(JSON_STR, "\"foo\\v\"", false);
  test(JSON_STR, "\"\\u1A2b\"", true);
  test(JSON_STR, "\"\\uDEAD\"", true);
  test(JSON_STR, "\"\\uF00\"", false);
  test(JSON_STR, "\"\\uF00BAR\"", true);
  test(JSON_STR, "\"foo\\/\"", true);
  test(JSON_STR, "\"\xcf\x84\"", true);
  test(JSON_STR, "\"\x80\"", true);
  test(JSON_STR, "\"\x88x/\"", true);
  // RFC 8259, $6 'Numbers'
#define JSON_NUM "\\-?(0|1-90-9*)(\\.0-9+)?([eE][\\+\\-]?0-9+)?"
  test(JSON_NUM, "e", false);
  test(JSON_NUM, "1", true);
  test(JSON_NUM, "10", true);
  test(JSON_NUM, "01", false);
  test(JSON_NUM, "-5", true);
  test(JSON_NUM, "+5", false);
  test(JSON_NUM, ".3", false);
  test(JSON_NUM, "2.", false);
  test(JSON_NUM, "2.3", true);
  test(JSON_NUM, "1e", false);
  test(JSON_NUM, "1e0", true);
  test(JSON_NUM, "1E+0", true);
  test(JSON_NUM, "1e-0", true);
  test(JSON_NUM, "1E10", true);
  test(JSON_NUM, "1e+00", true);
#define JSON_BOOL "true|false"
#define JSON_NULL "null"
#define JSON_PRIM JSON_STR "|" JSON_NUM "|" JSON_BOOL "|" JSON_NULL
  test(JSON_PRIM, "nul", false);
  test(JSON_PRIM, "null", true);
  test(JSON_PRIM, "nulll", false);
  test(JSON_PRIM, "true", true);
  test(JSON_PRIM, "false", true);
  test(JSON_PRIM, "{}", false);
  test(JSON_PRIM, "[]", false);
  test(JSON_PRIM, "1,", false);
  test(JSON_PRIM, "-5.6e2", true);
  test(JSON_PRIM, "\"1a\\n\"", true);
  test(JSON_PRIM, "\"1a\\n\" ", false);
  // RFC 3629, $3 'UTF-8 definition'. derived from the plain English definition
#define TAIL "\\x80-\\xbf"
#define BYTE_PAT                                                               \
  "(\\x00-\\x7f|\\xc0-\\xdf" TAIL "|\\xe0-\\xef" TAIL TAIL                     \
  "|\\xf0-\\xf7" TAIL TAIL TAIL ")"
#define OVERLONG "(\\xc0-\\xc1<>|\\xe0\\x80-\\x9f<>|\\xf0\\x80-\\x8f<><>)"
#define SURROGATE "\\xed\\xa0-\\xbf<>"
#define TOO_BIG "(\\xf4\\x90-\\xff" TAIL TAIL "|\\xf5-\\xff" TAIL TAIL TAIL ")"
#define UTF8_CHAR_1 "(" BYTE_PAT "&~" OVERLONG "&~" SURROGATE "&~" TOO_BIG ")"
  // RFC 3629, $4 'Syntax of UTF-8 Byte Sequences'. direct transcription of ABNF
#define TAIL "\\x80-\\xbf"
#define UTF8_1 "\\x00-\\x7f"
#define UTF8_2 "\\xc2-\\xdf" TAIL
#define UTF8_3                                                                 \
  "\\xe0\\xa0-\\xbf" TAIL "|\\xe1-\\xec" TAIL TAIL "|\\xed\\x80-\\x9f" TAIL    \
  "|\\xee-\\xef" TAIL TAIL
#define UTF8_4                                                                 \
  "\\xf0\\x90-\\xbf" TAIL TAIL "|\\xf1-\\xf3" TAIL TAIL TAIL                   \
  "|\\xf4\\x80-\\x8f" TAIL TAIL
#define UTF8_CHAR_2 "(" UTF8_1 "|" UTF8_2 "|" UTF8_3 "|" UTF8_4 ")"
  // generated by `ltre_decompile` then tweaked manually to minimize length
#define UTF8_CHAR_3                                                            \
  "(\\x00-\\x7f|(\\xc2-\\xdf|\\xe0\\xa0-\\xbf|\\xed\\x80-\\x9f|"               \
  "([\\xe1-\\xec\\xee\\xef]|\\xf0\\x90-\\xbf|\\xf4\\x80-\\x8f|"                \
  "\\xf1-\\xf3\\x80-\\xbf)\\x80-\\xbf)\\x80-\\xbf)"
  // all three regular expressions above should accept the same language
#define UTF8_CHAR_ALL UTF8_CHAR_1 "&" UTF8_CHAR_2 "&" UTF8_CHAR_3
#define UTF8_CHARS_ALL UTF8_CHAR_1 "*&" UTF8_CHAR_2 "*&" UTF8_CHAR_3 "*"
#define UTF8_CHAR_SOME UTF8_CHAR_1 "|" UTF8_CHAR_2 "|" UTF8_CHAR_3
#define UTF8_CHARS_SOME UTF8_CHAR_1 "*|" UTF8_CHAR_2 "*|" UTF8_CHAR_3 "*"
  test(UTF8_CHAR_SOME, "ab", false);
  test(UTF8_CHAR_SOME, "\x80x", false);
  test(UTF8_CHAR_SOME, "\x80", false);
  test(UTF8_CHAR_SOME, "\xbf", false);
  test(UTF8_CHAR_SOME, "\xc0", false);
  test(UTF8_CHAR_SOME, "\xc1", false);
  test(UTF8_CHAR_SOME, "\xff", false);
  test(UTF8_CHAR_SOME, "\xed\xa1\x8c", false); // RFC ex. (surrogate)
  test(UTF8_CHAR_SOME, "\xed\xbe\xb4", false); // RFC ex. (surrogate)
  test(UTF8_CHAR_SOME, "\xed\xa0\x80", false); // d800, first surrogate
  test(UTF8_CHAR_SOME, "\xc0\x80", false);     // RFC ex. (overlong nul)
  test(UTF8_CHAR_ALL, "\x7f", true);
  test(UTF8_CHAR_ALL, "\xF0\x9E\x84\x93", true);
  test(UTF8_CHAR_ALL, "\x2f", true);               // solidus
  test(UTF8_CHAR_SOME, "\xc0\xaf", false);         // overlong solidus
  test(UTF8_CHAR_SOME, "\xe0\x80\xaf", false);     // overlong solidus
  test(UTF8_CHAR_SOME, "\xf0\x80\x80\xaf", false); // overlong solidus
  test(UTF8_CHAR_SOME, "\xf7\xbf\xbf\xbf", false); // 1fffff, too big
  test(UTF8_CHARS_ALL, "\x41\xe2\x89\xa2\xce\x91\x2e", true);         // RFC ex.
  test(UTF8_CHARS_ALL, "\xed\x95\x9c\xea\xb5\xad\xec\x96\xb4", true); // RFC ex.
  test(UTF8_CHARS_ALL, "\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e", true); // RFC ex.
  test(UTF8_CHARS_ALL, "\xef\xbb\xbf\xf0\xa3\x8e\xb4", true);         // RFC ex.
  test(UTF8_CHARS_ALL, "abcABC123<=>", true);
  test(UTF8_CHARS_ALL, "\xc2\x80", true);
  test(UTF8_CHARS_SOME, "\xc2\x7f", false);     // bad tail
  test(UTF8_CHARS_SOME, "\xe2\x28\xa1", false); // bad tail
  test(UTF8_CHARS_SOME, "\x80x/", false);
}
