# LTRE

_Finite automaton regular expression engine_

## Overview

LTRE is a regular expression library written in C99 that has no dependencies but the C standard library. It parses regular expressions into NFAs then compiles them down to minimal DFAs for linear-time matching. It also provides facilities for manipulating NFAs, for lazily constructing DFAs and for decompiling DFAs back into regular expressions.

```
                             _
                            | | ltre_partial, ltre_ignorecase, ltre_complement
                            | V
(RE)-------ltre_parse----->(NFA)----ltre_compile--->(DFA)----ltre_serialize--->(BUF)
    ---ltre_fixed_string-->  |  <--ltre_uncompile---  |  <--ltre_deserialize---
    <------------------------|-----ltre_decompile---  |
                             V                        V
                     ltre_matches_lazy          ltre_matches
```

For sample regular expressions, see the test suite [test.c](test.c). For a more realistic use-case, see the minimal command-line search tool [ltrep.c](ltrep.c). For a demo of DFA decompilation, see the regex complementation tool [compl.c](compl.c)

## Usage

To build and run the test suite:

```bash
make test
bin/test # should have no output
```

To build and run the minimal search tool:

```bash
make ltrep
sh test.sh # should have no output
bin/ltrep -h # displays usage
bin/ltrep '"(^[\\"]|\\<>)*"' ltrep.c ltre.c
```

To build and run the regex complementation tool:

```bash
make compl
echo 'abc' | bin/compl
# outputs a?|ab|(^a|a^b|ab^c|abc<>)<>*
```

## Syntax and Semantics

See [grammar.bnf](grammar.bnf) for the regular expression grammar specification. As an informal quick reference, note that:

- Character ranges support wraparound and may appear outside character classes.
- Metacharacters within character classes must be escaped to be matched literally.
- Character classes are complemented by prefixing the opening bracket with `^`.
- Literal characters and character ranges can be complemented by prefixing them with `^`.
- `-<>&~` are considered metacharacters; they may be matched literally by escaping them.
- `.` does not match newlines; to match any character including newlines, use `<>`.
- The empty regular expression matches the empty word; to match no word, use `[]`.
- The lower bound of bounded repetitions may be omitted and defaults to `0`.
- Regular expressions can be intersected with infix `&` and complemented with prefix `~`.

Supported features are as follows:

| Name                 | Example    | Meaning                                    | Type                         |
| -------------------- | ---------- | ------------------------------------------ | ---------------------------- |
| Character Literal    | `a`        | Symbol consisting of the character `a`     | `symbol`                     |
| Metacharacter Escape | `\+`       | Symbol consisting of the metacharacter `+` | `symbol`                     |
| C-Style Escape       | `\r`       | Symbol corresponding to the escape `\r`    | `symbol`                     |
| Hexadecimal Escape   | `\x41`     | Symbol with character code `0x41`          | `symbol`                     |
| Symbol Promotion     | any symbol | Singleton set containing the symbol        | `symbol -> symset`           |
| Character Range      | `a-z`      | Set of all characters from `a` to `z`      | `(symbol, symbol) -> symset` |
| Shorthand Class      | `\d`       | Set of characters in the PERL-style class  | `symset`                     |
| Symset Complement    | `^a`       | Set of all characters not in `a`           | `symset -> symset`           |
| Character Class      | `[ab]`     | Set of characters in `a` or in `b`         | `[symset] -> symset`         |
| Symset Intersection  | `<ab>`     | Set of characters in `a` and in `b`        | `[symset] -> symset`         |
| Symset Promotion     | any symset | Match any single character from the set    | `symset -> regex`            |
| Grouping             | `(a)`      | Match the subexpression `a`                | `regex -> regex`             |
| Kleene Star          | `a*`       | Match zero or more `a`s                    | `regex -> regex`             |
| Kleene Plus          | `a+`       | Match one or more `a`s                     | `regex -> regex`             |
| Optional             | `a?`       | Match zero or one `a`                      | `regex -> regex`             |
| Exact Repetition     | `a{4}`     | Match exactly 4 `a`s                       | `regex -> regex`             |
| Bounded Repetition   | `a{3,5}`   | Match 3 to 5 `a`s                          | `regex -> regex`             |
| Minimum Repetition   | `a{3,}`    | Match at least 3 `a`s                      | `regex -> regex`             |
| Maximum Repetition   | `a{,5}`    | Match at most 5 `a`s                       | `regex -> regex`             |
| Concatenation        | `ab`       | Match `a` followed by `b`                  | `(regex, regex) -> regex`    |
| Alternation          | `a\|b`     | Match either `a` or `b`                    | `(regex, regex) -> regex`    |
| Intersection         | `a&b`      | Match simultaneously `a` and `b`           | `(regex, regex) -> regex`    |
| Complement           | `~a`       | Match anything not matched by `a`          | `regex -> regex`             |

C-style escapes are supported for `abfnrtv`. Metacharacter escapes are supported for `\.-^$*+?{}[]<>()|&~`. Shorthand classes are supported for `dDsSwW`. Semantically, the dot metacharacter is considered a shorthand class.

`symbol`s promote to `symset`s (forming a singleton set) and `symset`s promote to `regex`es (which match any single character from the set). Alternation and intersection have the lowest precedence, followed by complementation, followed by concatenation, followed by quantification, followed by symset complementation. Alternation and intersection are left-associative. At most one complement and at most one quantifier may be applied to a `regex` per grouping level, and at most one symset complement may be applied to a `symset` per character class or symset intersection level.
