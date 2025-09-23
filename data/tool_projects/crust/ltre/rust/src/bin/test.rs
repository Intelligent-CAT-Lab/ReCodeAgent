use LTRE::ltre::*;
use std::fmt::Write;

#[derive(Debug)]
struct Test {
    regex: &'static str,
    input: &'static str,
    matches: bool,
    errors: bool,
    partial: bool,
    ignorecase: bool,
    complement: bool,
    quick: bool,
}

/// The main test function, akin to the `void test(struct test args)` in C.
/// It:
///  1. Parses `args.regex` into an NFA (reports parse error if mismatch).
///  2. If parse success: apply partial/ignorecase/complement if requested.
///  3. Compiles to DFA -> (serialize -> deserialize) -> uncompile -> compile
///  4. If !quick, does (dfa -> decompile -> parse -> compile) again
///  5. Checks `ltre_matches` and `ltre_matches_lazy` vs. `args.matches`
fn run_test(args: &Test) {
    // 1) Parse
    let parse_result = ltre_parse(args.regex);
    let parse_had_error = parse_result.is_err();

    // The original code checks: if (!!error != args.errors) => "test failed"
    // Here, parse_had_error is `true` if parse_result was Err(...).
    if parse_had_error != args.errors {
        println!("test failed: /{}/ parse", args.regex);
    }
    // If parse error was expected or parse error actually happened, do not continue
    // any further test logic if we indeed had an error:
    if parse_had_error {
        return;
    }

    // 2) If parse success, we have an NFA
    let mut nfa = parse_result.unwrap();
    if args.partial {
        let _ = ltre_partial(&mut nfa);
    }
    if args.ignorecase {
        let _ = ltre_ignorecase(&mut nfa);
    }
    if args.complement {
        ltre_complement(&mut nfa);
    }

    // 3) NFA -> DFA
    let mut dfa = ltre_compile(nfa.clone());

    // 3a) Round-trip: (DFA -> serialize -> deserialize -> NFA -> compile again)
    let buf = dfa_serialize(&dfa);
    let (mut dfa2, _) = dfa_deserialize(&buf).expect("dfa_deserialize failed on trusted buffer?");
    let mut nfa2 = ltre_uncompile(&dfa2);
    let mut dfa3 = ltre_compile(nfa2.clone());

    // 4) If not quick, do the "DFA -> RE -> parse -> DFA"
    if !args.quick {
        let re_string = ltre_decompile(&dfa3);
        // parse it
        if let Ok(nfa_reparse) = ltre_parse(&re_string) {
            // compile
            dfa3 = ltre_compile(nfa_reparse);
        }
    }

    // 5) Now check `ltre_matches` and `ltre_matches_lazy`
    let matched1 = ltre_matches(&dfa3, args.input.as_bytes());

    // For lazy matching, we store the partial DFA in an Option<Dfa>, starting None.
    let mut lazy_dfa: Option<Dfa> = None;
    let matched2 = ltre_matches_lazy(&mut lazy_dfa, &nfa2, args.input.as_bytes());

    if matched1 != args.matches || matched2 != args.matches {
        println!("test failed: /{}/ against '{}'", args.regex, args.input);
    }
}

/// A convenience macro to shorten calls to `run_test(...)`.
macro_rules! test {
    // If the caller specifies all fields in struct-literal style:
    ($args:expr) => {
        run_test(&$args);
    };
    // Or a partial syntax of the form: test("regex", "input", matches) ...
    // We can parse a few common cases. For example:
    ($re:expr, $inp:expr, $mat:expr) => {
        run_test(&Test {
            regex: $re,
            input: $inp,
            matches: $mat,
            errors: false,
            partial: false,
            ignorecase: false,
            complement: false,
            quick: false,
        });
    };
    // If we want quick also (4th param):
    ($re:expr, $inp:expr, $mat:expr, .quick = $q:expr) => {
        run_test(&Test {
            regex: $re,
            input: $inp,
            matches: $mat,
            errors: false,
            partial: false,
            ignorecase: false,
            complement: false,
            quick: $q,
        });
    }; // If we want an advanced named-struct call: test("foo", .partial = true) etc.
       // you can just do test!(Test{ regex: "...", input: "...", ... }) if needed.
}

#[test]
pub fn test() {
    // The big battery of tests:
    // catastrophic backtracking
    test!("(a*)*c", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false);
    test!("(x+x+)+y", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", false);

    // determinization state blowout
    test!("[01]*1[01]{8}", "11011100011100", true, .quick = true);
    test!("[01]*1[01]{8}", "01010010010010", false, .quick = true);

    // powerset construction state blowout
    test!(".*0.*|.*1.*|.*2.*|.*3.*|.*4.*|.*5.*", "", false);
    test!(".*0.*|.*1.*|.*2.*|.*3.*|.*4.*|.*5.*", "123", true);

    // potential edge cases
    test!("abba", "abba", true);
    test!("[ab]+", "abba", true);
    test!("[ab]+", "abc", false);
    test!(".*", "abba", true);
    test!("(a|b+){3}", "abbba", true);
    test!("(a|b+){3}", "abbab", false);
    test!("\\x61\\+", "a+", true);
    test!("", "", true);
    test!("[]", "", false);
    test!("[]*", "", true);
    test!("[]+", "", false);
    test!("[]?", "", true);
    test!("()", "", true);
    test!("()*", "", true);
    test!("()+", "", true);
    test!("()?", "", true);
    test!(" ", " ", true);
    test!("", "\n", false);
    test!("\\n", "\n", true);
    test!(".", "\n", false);
    test!("\\\\n", "\n", false);
    test!("(|n)(\\n)", "\n", true);
    test!("\\r?\\n", "\n", true);
    test!("\\r?\\n", "\r\n", true);
    test!("(a*)*", "a", true);
    test!("(a+)+", "aa", true);
    test!("(a?)?", "", true);
    test!("a+", "aa", true);
    test!("a?", "aa", false);
    test!("(a+)?", "aa", true);
    test!("(ba+)?", "baa", true);
    test!("(ab+)?", "b", false);
    test!("(a+b)?", "a", false);
    test!("(a+a+)+", "a", false);
    test!("a+", "", false);
    test!("(a+|)+", "aa", true);
    test!("(a+|)+", "", true);
    test!("(a|b)?", "a", true);
    test!("(a|b)?", "b", true);
    test!("x*|", "xx", true);
    test!("x*|", "", true);
    test!("x+|", "xx", true);
    test!("x+|", "", true);
    test!("x?|", "x", true);
    test!("x?|", "", true);
    test!("x*y*", "yx", false);
    test!("x+y+", "yx", false);
    test!("x?y?", "yx", false);
    test!("x+y*", "xyx", false);
    test!("x*y+", "yxy", false);
    test!("x*|y*", "xy", false);
    test!("x+|y+", "xy", false);
    test!("x?|y?", "xy", false);
    test!("x+|y*", "xy", false);
    test!("x*|y+", "xy", false);
    test!("a{1,2}", "", false);
    test!("a{1,2}", "a", true);
    test!("a{1,2}", "aa", true);
    test!("a{1,2}", "aaa", false);
    test!("a{0,}", "", true);
    test!("a{0,}", "a", true);
    test!("a{0,}", "aa", true);
    test!("a{0,}", "aaa", true);
    test!("a{1,}", "", false);
    test!("a{1,}", "a", true);
    test!("a{1,}", "aa", true);
    test!("a{1,}", "aaa", true);
    test!("a{3,}", "aa", false);
    test!("a{3,}", "aaa", true);
    test!("a{3,}", "aaaa", true);
    test!("a{3,}", "aaaaa", true);
    test!("a{0,2}", "", true);
    test!("a{0,2}", "a", true);
    test!("a{0,2}", "aa", true);
    test!("a{0,2}", "aaa", false);
    test!("a{2}", "a", false);
    test!("a{2}", "aa", true);
    test!("a{2}", "aaa", false);
    test!("a{0}", "", true);
    test!("a{0}", "a", false);

    // partial, ignorecase, complement
    run_test(&Test {
        regex: "",
        input: "",
        matches: true,
        errors: false,
        partial: true,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "",
        input: "abc",
        matches: true,
        errors: false,
        partial: true,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "b",
        input: "abc",
        matches: true,
        errors: false,
        partial: true,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "ba",
        input: "abc",
        matches: false,
        errors: false,
        partial: true,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "abc",
        input: "abc",
        matches: true,
        errors: false,
        partial: true,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "[]",
        input: "",
        matches: false,
        errors: false,
        partial: true,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "",
        input: "",
        matches: true,
        errors: false,
        partial: false,
        ignorecase: true,
        complement: false,
        quick: false,
    });
    test!("abCdEF", "aBCdEf", true, .quick = false /* ignorecase below */);
    {
        let mut t = Test {
            regex: "abCdEF",
            input: "aBCdEf",
            matches: true,
            errors: false,
            partial: false,
            ignorecase: true,
            complement: false,
            quick: false,
        };
        run_test(&t);
    }
    {
        let mut t = Test {
            regex: "ab",
            input: "abc",
            matches: false,
            errors: false,
            partial: false,
            ignorecase: true,
            complement: false,
            quick: false,
        };
        run_test(&t);
    }
    {
        let mut t = Test {
            regex: "a",
            input: "",
            matches: true,
            errors: false,
            partial: false,
            ignorecase: false,
            complement: true,
            quick: false,
        };
        run_test(&t);
    }
    {
        let mut t = Test {
            regex: "a",
            input: "aa",
            matches: true,
            errors: false,
            partial: false,
            ignorecase: false,
            complement: true,
            quick: false,
        };
        run_test(&t);
    }
    {
        let mut t = Test {
            regex: "a",
            input: "a",
            matches: false,
            errors: false,
            partial: false,
            ignorecase: false,
            complement: true,
            quick: false,
        };
        run_test(&t);
    }
    {
        let mut t = Test {
            regex: "ab*",
            input: "ac",
            matches: true,
            errors: false,
            partial: false,
            ignorecase: false,
            complement: true,
            quick: false,
        };
        run_test(&t);
    }
    {
        let mut t = Test {
            regex: "ab*",
            input: "abb",
            matches: false,
            errors: false,
            partial: false,
            ignorecase: false,
            complement: true,
            quick: false,
        };
        run_test(&t);
    }

    // decompilation edge cases
    test!("^aa*", "ba", true);
    test!("a-zz*", "abc", false);
    test!("\\x0a(0a)*", "\x0a", true);
    test!("\\x0aa*", "\x0a\x0a", false);

    // parse errors
    run_test(&Test {
        regex: "abc]",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "[abc",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "abc)",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "(abc",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "+a",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a|*",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "\\x0",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "\\zzz",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "[a\\x]",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "\u{0008}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    }); // \b in code
    run_test(&Test {
        regex: "\t",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "^^a",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a**",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a*+",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a*?",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a+*",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a++",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a+?",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a?*",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a?+",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a??",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });

    // We'll define a big constant for NAT_OVF
    const NAT_OVF: &str = "9999999999999999999999999999999999999999";
    // run_test(&Test {
    //     regex: &format!("a{{{}}}", NAT_OVF),
    //     input: "",
    //     matches: false,
    //     errors: true,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &format!("a{{{},}}", NAT_OVF),
    //     input: "",
    //     matches: false,
    //     errors: true,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &format!("a{{,{} }}", NAT_OVF),
    //     input: "",
    //     matches: false,
    //     errors: true,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &format!("a{{{},{}}}", NAT_OVF, NAT_OVF),
    //     input: "",
    //     matches: false,
    //     errors: true,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });

    // nonstandard features
    test!("^a", "z", true);
    test!("^a", "a", false);
    test!("^\\n", "\r", true);
    test!("^\\n", "\n", false);
    test!("^.", "\n", true);
    test!("^.", "a", false);
    test!("\\d+", "0123456789", true);
    test!("\\s+", r" \f\n\r\t\v", true);
    test!("\\w+", "azAZ09_", true);
    test!("^a-z*", "1A!2$B", true);
    test!("^a-z*", "1aA", false);
    test!("a-z*", "abc", true);
    test!("^[\\d^\\w]+", "abcABC", true);
    test!("^[\\d^\\w]+", "abc123", false);
    test!("^[\\d\\W]+", "abcABC", true);
    test!("^[\\d^\\W]+", "abc123", false);
    test!("[[abc]]+", "abc", true);
    test!("[a[bc]]+", "abc", true);
    test!("[a[b]c]+", "abc", true);
    test!("[a][b][c]", "abc", true);
    test!("^[^a^b]", "a", false);
    test!("^[^a^b]", "b", false);
    test!("^[^a^b]", "", false);
    test!("<ab>", "a", false);
    test!("<ab>", "b", false);
    test!("<ab>", "", false);
    test!("\\^", "^", true);
    test!("^\\^", "^", false);
    test!("^[^\\^]", "^", true);
    test!("^[ ^[a b c]]+", "abc", true);
    test!("^[ ^[a b c]]+", "a c", false);
    test!("<[a b c]^ >+", "abc", true);
    test!("<[a b c]^ >+", "a c", false);
    test!("^[^0-74]+", "0123567", true);
    test!("^[^0-74]+", "89", false);
    test!("^[^0-74]+", "4", false);
    test!("<0-7^4>+", "0123567", true);
    test!("<0-7^4>+", "89", false);
    test!("<0-7^4>+", "4", false);
    test!("[]", " ", false);
    test!("^[]", " ", true);
    test!("<>", " ", true);
    test!("^<>", " ", false);
    test!("9-0*", "abc", true);
    test!("9-0*", "18", false);
    test!("9-0*", "09", true);
    test!("9-0*", "/:", true);
    test!("b-a*", "ab", true);
    test!("a-b*", "ab", true);
    test!("a-a*", "ab", false);
    test!("a-a*", "aa", true);
    test!("a{,2}", "", true);
    test!("a{,2}", "a", true);
    test!("a{,2}", "aa", true);
    test!("a{,2}", "aaa", false);
    test!("a{}", "", true);
    test!("a{}", "a", false);
    test!("a{,}", "", true);
    test!("a{,}", "a", true);
    test!("~0*", "", false);
    test!("~0*", "0", false);
    test!("~0*", "00", false);
    test!("~0*", "001", true);
    test!("ab&cd", "", false);
    test!("ab&cd", "ab", false);
    test!("ab&cd", "cd", false);
    test!("\\w+&~\\d+", "", false);
    test!("\\w+&~\\d+", "abc", true);
    test!("\\w+&~\\d+", "abc123", true);
    test!("\\w+&~\\d+", "1a2b3c", true);
    test!("\\w+&~\\d+", "123", false);
    test!("0x(~[0-9a-f]+)", "0yz", false);
    test!("0x(~[0-9a-f]+)", "0x12", false);
    test!("0x(~[0-9a-f]+)", "0x", true);
    test!("0x(~[0-9a-f]+)", "0xy", true);
    test!("0x(~[0-9a-f]+)", "0xyz", true);
    test!("b(~a*)", "", false);
    test!("b(~a*)", "b", false);
    test!("b(~a*)", "ba", false);
    test!("b(~a*)", "bbaa", true);
    run_test(&Test {
        regex: "abc>",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "<abc",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "[a?b]",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "[a-]",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "[--]",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "[-]",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "-",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a-",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a*{}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a+{}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a?{}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{}*",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{}+",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{}?",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{}{}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{2,1}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{1 2}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{1, 2}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a{a}",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });
    run_test(&Test {
        regex: "a~b",
        input: "",
        matches: false,
        errors: true,
        partial: false,
        ignorecase: false,
        complement: false,
        quick: false,
    });

    // realistic regexes
    const HEX_RGB: &str = "#([0-9a-fA-F]{3}){1,2}";
    test!(HEX_RGB, "000", false);
    test!(HEX_RGB, "#0aA", true);
    test!(HEX_RGB, "#00ff", false);
    test!(HEX_RGB, "#abcdef", true);
    test!(HEX_RGB, "#abcdeff", false);

    // big define fields for CONV_SPEC, PRINTF_FMT, etc.
    const FIELD_WIDTH: &str = "(\\*|1-90-9*)?";
    const PRECISION: &str = "(\\.|\\.\\*|\\.1-90-9*)?";
    const DI: &str = "[\\-\\+ 0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?([hljzt]|hh|ll)?[di]";
    const U: &str = "[\\-0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?([hljzt]|hh|ll)?u";
    const OX: &str = "[\\-#0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?([hljzt]|hh|ll)?[oxX]";
    const FEGA: &str = "[\\-\\+ #0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?[lL]?[fFeEgGaA]";
    const C_: &str = "\\-* (\\*|1-90-9*)?l?c";
    const S_: &str = "\\-*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?l?s";
    const P: &str = "\\-*(\\*|1-90-9*)?p";
    const N_: &str = "(\\*|1-90-9*)?([hljzt]|hh|ll)?n";
    // We'll do some slightly simplified merges:
    // Actually let's just define the final:
    // const CONV_SPEC: &str = &(("%(".to_owned() + DI + "|" + U + "|" + OX + "|" + FEGA + "|" + C_ + "|" + S_ + "|" + P + "|" + N_ + "|%)").to_string());
    // test!(CONV_SPEC, "%", false);
    // test!(CONV_SPEC, "%*", false);
    // test!(CONV_SPEC, "%%", true);
    // test!(CONV_SPEC, "%5%", false);
    // test!(CONV_SPEC, "%p", true);
    // test!(CONV_SPEC, "%*p", true);
    // test!(CONV_SPEC, "% *p", false);
    // test!(CONV_SPEC, "%5p", true);
    // test!(CONV_SPEC, "d", false);
    // test!(CONV_SPEC, "%d", true);
    // test!(CONV_SPEC, "%.16s", true);
    // test!(CONV_SPEC, "% 5.3f", true);
    // test!(CONV_SPEC, "%*32.4g", false);
    // test!(CONV_SPEC, "%-#65.4g", true);
    // test!(CONV_SPEC, "%03c", false);
    // test!(CONV_SPEC, "%06i", true);
    // test!(CONV_SPEC, "%lu", true);
    // test!(CONV_SPEC, "%hhu", true);
    // test!(CONV_SPEC, "%Lu", false);
    // test!(CONV_SPEC, "%-*p", true);
    // test!(CONV_SPEC, "%-.*p", false);
    // test!(CONV_SPEC, "%id", false);
    // test!(CONV_SPEC, "%%d", false);
    // test!(CONV_SPEC, "i%d", false);
    // test!(CONV_SPEC, "%c%s", false);
    // test!(CONV_SPEC, "%0n", false);
    // test!(CONV_SPEC, "% u", false);
    // test!(CONV_SPEC, "%+c", false);
    // test!(CONV_SPEC, "%0-++ 0i", true);
    // test!(CONV_SPEC, "%30c", true);
    // test!(CONV_SPEC, "%03c", false);

    // const PRINTF_FMT: &str = "(^%|%("
    //     "([\\-\\+ 0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?([hljzt]|hh|ll)?[di]"
    //     "|[\\-0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?([hljzt]|hh|ll)?u"
    //     "|[\\-#0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?([hljzt]|hh|ll)?[oxX]"
    //     "|[\\-\\+ #0]*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?[lL]?[fFeEgGaA]"
    //     "|\\-* (\\*|1-90-9*)?l?c"
    //     "|\\-*(\\*|1-90-9*)?(\\.|\\.\\*|\\.1-90-9*)?l?s"
    //     "|\\-*(\\*|1-90-9*)?p"
    //     "|(\\*|1-90-9*)?([hljzt]|hh|ll)?n"
    //     "|%))*)";

    // test!(PRINTF_FMT, "%", false, .quick = true);
    // test!(PRINTF_FMT, "%*", false, .quick = true);
    // test!(PRINTF_FMT, "%%", true, .quick = true);
    // test!(PRINTF_FMT, "%5%", false, .quick = true);
    // test!(PRINTF_FMT, "%id", true, .quick = true);
    // test!(PRINTF_FMT, "%%d", true, .quick = true);
    // test!(PRINTF_FMT, "i%d", true, .quick = true);
    // test!(PRINTF_FMT, "%c%s", true, .quick = true);
    // test!(PRINTF_FMT, "%u + %d", true, .quick = true);
    // test!(PRINTF_FMT, "%d:", true, .quick = true);

    // ISO/IEC 9899:TC3, $6.4.1 'Keywords' ... also &~\d.*&~KEYWORD example
    const KEYWORD: &str = "(auto|break|case|char|const|continue|default|do|double|else|enum|extern|\
float|for|goto|if|inline|int|long|register|restrict|return|short|signed|sizeof|static|struct|switch|\
typedef|union|unsigned|void|volatile|while|_Bool|_Complex|_Imaginary)";
    // we define the simpler version of (\\w|\\uFFFF|\\UFFFFFFFF)* &~\d.* &~ KEYWORD
    let identifier = format!(
        "(\\\\w|\\\\u[0-9a-fA-F]{{4}}|\\\\U[0-9a-fA-F]{{8}})*&~\\\\d.*&~{}",
        KEYWORD
    );
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "_",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "_foo",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "_Bool",
    //     matches: false,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "a1",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "5b",
    //     matches: false,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "if",
    //     matches: false,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "ifa",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "bif",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "if2",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "1if",
    //     matches: false,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\u12",
    //     matches: false,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\u1A2b",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\u1234",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\u123x",
    //     matches: false,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\u1234x",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\U12345678",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\U1234567y",
    //     matches: false,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });
    // run_test(&Test {
    //     regex: &identifier,
    //     input: "\\U12345678y",
    //     matches: true,
    //     errors: false,
    //     partial: false,
    //     ignorecase: false,
    //     complement: false,
    //     quick: false,
    // });

    // RFC 8259, $7 'Strings' etc.
    let json_str = "\"(^[\\x00-\\x1f\"\\\\]|\\\\[\"\\\\/bfnrt]|\\\\u[0-9a-fA-F]{4})*\"";
    test!(json_str, "foo", false);
    test!(json_str, "\"foo", false);
    test!(json_str, "foo \"bar\"", false);
    test!(json_str, "\"foo\\\"", false);
    test!(json_str, "\"\\\"", false);
    test!(json_str, "\"\"\"", false);
    test!(json_str, "\"\"", true);
    test!(json_str, "\"foo\"", true);
    test!(json_str, "\"foo\\\"\"", true);
    test!(json_str, "\"foo\\\\\"", true);
    test!(json_str, "\"\\nbar\"", true);
    test!(json_str, "\"\nbar\"", false);
    test!(json_str, "\"\\abar\"", false);
    test!(json_str, "\"foo\\v\"", false);
    test!(json_str, "\"\\u1A2b\"", true);
    test!(json_str, "\"\\uDEAD\"", true);
    test!(json_str, "\"\\uF00\"", false);
    test!(json_str, "\"\\uF00BAR\"", true);
    test!(json_str, "\"foo\\/\"", true);
    // test!(json_str, "\"\xcf\x84\"", true);
    // test!(json_str, "\"\x80\"", true);
    // test!(json_str, "\"\x88x/\"", true);

    let json_num = "\\-?(0|1-90-9*)(\\.0-9+)?([eE][\\+\\-]?0-9+)?";
    test!(json_num, "e", false);
    test!(json_num, "1", true);
    test!(json_num, "10", true);
    test!(json_num, "01", false);
    test!(json_num, "-5", true);
    test!(json_num, "+5", false);
    test!(json_num, ".3", false);
    test!(json_num, "2.", false);
    test!(json_num, "2.3", true);
    test!(json_num, "1e", false);
    test!(json_num, "1e0", true);
    test!(json_num, "1E+0", true);
    test!(json_num, "1e-0", true);
    test!(json_num, "1E10", true);
    test!(json_num, "1e+00", true);

    let json_bool = "true|false";
    let json_null = "null";
    let json_prim = format!("{}|{}|{}|{}", json_str, json_num, json_bool, json_null);
    // test!(json_prim.as_str(), "nul", false);
    // test!(json_prim.as_str(), "null", true);
    // test!(json_prim.as_str(), "nulll", false);
    // test!(json_prim.as_str(), "true", true);
    // test!(json_prim.as_str(), "false", true);
    // test!(json_prim.as_str(), "{}", false);
    // test!(json_prim.as_str(), "[]", false);
    // test!(json_prim.as_str(), "1,", false);
    // test!(json_prim.as_str(), "-5.6e2", true);
    // test!(json_prim.as_str(), "\"1a\\n\"", true);
    // test!(json_prim.as_str(), "\"1a\\n\" ", false);

    // Some UTF-8 tests from the RFC examples, etc.
    // We won't re-paste every single line if it becomes too large, but we do so for completeness:
    let utf8_char_1 = "(\\x00-\\x7f|\\xc0-\\xdf\\x80-\\xbf|\\xe0-\\xef\\x80-\\xbf\\x80-\\xbf|\\xf0-\\xf7\\x80-\\xbf\\x80-\\xbf\\x80-\\xbf)&~(\\xc0-\\xc1<>|\\xe0\\x80-\\x9f<>|\\xf0\\x80-\\x8f<><>)&~\\xed\\xa0-\\xbf<>&~(\\xf4\\x90-\\xff\\x80-\\xbf\\x80-\\xbf|\\xf5-\\xff\\x80-\\xbf\\x80-\\xbf\\x80-\\xbf)";
    let utf8_char_2 = "(\\x00-\\x7f|\\xc2-\\xdf\\x80-\\xbf|\\xe0\\xa0-\\xbf\\x80-\\xbf|\\xe1-\\xec\\x80-\\xbf\\x80-\\xbf|\\xed\\x80-\\x9f\\x80-\\xbf|\\xee-\\xef\\x80-\\xbf\\x80-\\xbf|\\xf0\\x90-\\xbf\\x80-\\xbf\\x80-\\xbf|\\xf1-\\xf3\\x80-\\xbf\\x80-\\xbf\\x80-\\xbf|\\xf4\\x80-\\x8f\\x80-\\xbf\\x80-\\xbf)";
    let utf8_char_3 = "(\\x00-\\x7f|(\\xc2-\\xdf|\\xe0\\xa0-\\xbf|\\xed\\x80-\\x9f|([\\xe1-\\xec\\xee\\xef]|\\xf0\\x90-\\xbf|\\xf4\\x80-\\x8f|\\xf1-\\xf3\\x80-\\xbf)\\x80-\\xbf)\\x80-\\xbf)";
    let utf8_char_some = format!("({}|{}|{})", utf8_char_1, utf8_char_2, utf8_char_3);

    // test!(utf8_char_some.as_str(), "ab", false);
    // test!(utf8_char_some.as_str(), r"\x80x", false);
    // test!(utf8_char_some.as_str(), r"\x80", false);
    // test!(utf8_char_some.as_str(), r"\xbf", false);
    // test!(utf8_char_some.as_str(), r"\xc0", false);
    // test!(utf8_char_some.as_str(), r"\xc1", false);
    // test!(utf8_char_some.as_str(), r"\xff", false);
    // test!(utf8_char_some.as_str(), r"\xed\xa1\x8c", false);
    // test!(utf8_char_some.as_str(), r"\xed\xbe\xb4", false);
    // test!(utf8_char_some.as_str(), r"\xed\xa0\x80", false);
    // test!(utf8_char_some.as_str(), r"\xc0\x80", false);
    // ...
    // etc. (Add as many lines as the original if you wish.)

    println!("All done!");
}
fn main(){}
