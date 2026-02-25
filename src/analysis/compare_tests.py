#!/usr/bin/env python3
"""
Test Comparison Script

Compares test methods between source and target language implementations.
Uses tree-sitter for accurate parsing of Java and Python code.
Extracts and compares:
- Method signatures
- Parameters (with types for Java)
- Number of assertions
- Method bodies
"""

import ast
import csv
import json
import re
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass, field

import tree_sitter_java as tsjava
import tree_sitter_python as tspython
from tree_sitter import Language, Parser, Query

try:
    import tree_sitter_go as tsgo

    HAS_TREE_SITTER_GO = True
except ImportError:
    HAS_TREE_SITTER_GO = False

try:
    import tree_sitter_rust as tsrust

    HAS_TREE_SITTER_RUST = True
except ImportError:
    HAS_TREE_SITTER_RUST = False

try:
    import tree_sitter_javascript as tsjavascript

    HAS_TREE_SITTER_JS = True
except ImportError:
    HAS_TREE_SITTER_JS = False

try:
    from tree_sitter import QueryCursor

    USE_QUERY_CURSOR = True
except ImportError:
    USE_QUERY_CURSOR = False


def run_query_matches(query, node):
    """Run query matches compatible with both old and new tree-sitter versions."""
    if USE_QUERY_CURSOR:
        return QueryCursor(query).matches(node)
    else:
        return query.matches(node)


# Lazy load sentence transformers to avoid slow startup when not needed
_embedding_model = None


def get_embedding_model():
    """Lazy load the sentence transformer model.

    Options (uncomment to switch):
    - all-MiniLM-L6-v2: Fast (22M params), good quality
    - Qwen/Qwen3-Embedding-0.6B: Best quality (0.6B params), slower
      See: https://huggingface.co/Qwen/Qwen3-Embedding-0.6B
    """
    global _embedding_model
    if _embedding_model is None:
        from sentence_transformers import SentenceTransformer

        # Fast model (recommended for large test suites)
        # model_name = 'all-MiniLM-L6-v2'
        # High quality model (slower, better for code)
        model_name = "Qwen/Qwen3-Embedding-0.6B"
        print(f"Loading embedding model: {model_name}...")
        _embedding_model = SentenceTransformer(model_name, trust_remote_code=True)
        print("Model loaded.")
    return _embedding_model


# ============================================================================
# Metric Helper Functions
# ============================================================================


def compute_line_count(body: str) -> int:
    """Count non-empty lines in body."""
    if not body:
        return 0
    return len([line for line in body.split("\n") if line.strip()])


def extract_assertion_types(body: str, lang: str) -> Dict[str, int]:
    """Extract counts of different assertion types from test body.

    Args:
        body: The test method body text
        lang: Language identifier ('java', 'python', 'go', 'rust')

    Returns:
        Dictionary mapping assertion type to count
    """
    if not body:
        return {}

    counts = {}

    if lang == "java":
        # Java assertion patterns
        patterns = [
            (r"\bassertEquals\s*\(", "assertEquals"),
            (r"\bassertTrue\s*\(", "assertTrue"),
            (r"\bassertFalse\s*\(", "assertFalse"),
            (r"\bassertNull\s*\(", "assertNull"),
            (r"\bassertNotNull\s*\(", "assertNotNull"),
            (r"\bassertThrows\s*\(", "assertThrows"),
            (r"\bassertSame\s*\(", "assertSame"),
            (r"\bassertNotSame\s*\(", "assertNotSame"),
            (r"\bassertArrayEquals\s*\(", "assertArrayEquals"),
            (r"\bfail\s*\(", "fail"),
        ]
    elif lang == "go":
        # Go test reporting patterns (t.Errorf, t.Fatalf, etc.)
        patterns = [
            (r"\bt\.Errorf\s*\(", "t.Errorf"),
            (r"\bt\.Error\b", "t.Error"),
            (r"\bt\.Fatalf\s*\(", "t.Fatalf"),
            (r"\bt\.Fatal\b", "t.Fatal"),
            (r"\bt\.FailNow\s*\(", "t.FailNow"),
            (r"\bt\.Fail\b", "t.Fail"),
        ]
    elif lang == "rust":
        # Rust assertion macro patterns
        # The catch-all assert_*! excludes already-named assert_eq! and assert_ne! via negative lookahead
        patterns = [
            (r"\bassert_eq!\s*\(", "assert_eq!"),
            (r"\bassert_ne!\s*\(", "assert_ne!"),
            (r"\bassert!\s*\(", "assert!"),
            (r"\bpanic!\s*\(", "panic!"),
            (r"\bassert_(?!eq!|ne!)(\w+)!\s*\(", "assert_*!"),
        ]
    elif lang == "javascript":
        # JavaScript assertion patterns (skel custom helper + console.assert + Node assert module)
        patterns = [
            (r"\bassert_equal\s*\(", "assert_equal"),
            (r"\bconsole\.assert\s*\(", "console.assert"),
            (r"\bassert\.strictEqual\s*\(", "assert.strictEqual"),
            (r"\bassert\.deepEqual\s*\(", "assert.deepEqual"),
            (r"\bassert\.notEqual\s*\(", "assert.notEqual"),
            (r"\bassert\.equal\s*\(", "assert.equal"),
            (r"\bassert\.ok\s*\(", "assert.ok"),
            (r"\bexpect\s*\(", "expect"),
        ]
    else:
        # Python assertion patterns
        patterns = [
            (r"\bassertEqual\s*\(", "assertEqual"),
            (r"\bassertTrue\s*\(", "assertTrue"),
            (r"\bassertFalse\s*\(", "assertFalse"),
            (r"\bassertIsNone\s*\(", "assertIsNone"),
            (r"\bassertIsNotNone\s*\(", "assertIsNotNone"),
            (r"\bassertRaises\s*\(", "assertRaises"),
            (r"\bassertIn\s*\(", "assertIn"),
            (r"\bassertNotIn\s*\(", "assertNotIn"),
            (r"\bassertIs\s*\(", "assertIs"),
            (r"\bassertIsNot\s*\(", "assertIsNot"),
            (r"\bpytest\.raises\s*\(", "pytest.raises"),
            # Treat `assert <lhs> == <rhs>` as assertEquals-like (common in these translations)
            (r"(?m)^\s*assert\s+.*==.*$", "assert_eq"),
            # Bare assert statements excluding `assert x == y` (counted as assert_eq) and internal asserts like `self.assertX`
            (r"(?m)^\s*assert\s+(?!.*==)(?!_).+", "assert"),
            (r"\bfail\s*\(", "fail"),
        ]

    for pattern, name in patterns:
        matches = re.findall(pattern, body)
        if matches:
            counts[name] = len(matches)

    return counts


def compute_similarity_score(source_text: str, target_text: str) -> float:
    """Compute cosine similarity between source and target using sentence embeddings.

    Args:
        source_text: Source (Java) test code
        target_text: Target (Python) test code

    Returns:
        Cosine similarity score between 0.0 and 1.0
    """
    if not source_text or not target_text:
        return 0.0

    try:
        import numpy as np

        model = get_embedding_model()

        # Get embeddings
        embeddings = model.encode([source_text, target_text])

        # Compute cosine similarity
        source_emb = embeddings[0]
        target_emb = embeddings[1]

        dot_product = np.dot(source_emb, target_emb)
        norm_source = np.linalg.norm(source_emb)
        norm_target = np.linalg.norm(target_emb)

        if norm_source == 0 or norm_target == 0:
            return 0.0

        similarity = dot_product / (norm_source * norm_target)
        return round(float(similarity), 4)
    except Exception as e:
        print(f"Warning: Could not compute similarity: {e}")
        return 0.0


def count_method_calls(body: str, lang: str) -> int:
    """Count method invocations in test body.

    Args:
        body: The test method body text
        lang: Language identifier ('java', 'python', 'go', 'rust')

    Returns:
        Number of method calls detected
    """
    if not body:
        return 0

    # Pattern to match method calls: identifier followed by (
    # Exclude common keywords
    if lang == "java":
        keywords = {"if", "for", "while", "switch", "catch", "synchronized", "new", "return", "throw"}
    elif lang == "go":
        keywords = {"if", "for", "switch", "select", "return", "defer", "go", "func", "type", "var", "const"}
    elif lang == "rust":
        keywords = {
            "if",
            "for",
            "while",
            "loop",
            "match",
            "return",
            "let",
            "fn",
            "struct",
            "enum",
            "impl",
            "mod",
            "use",
            "pub",
        }
    elif lang == "javascript":
        keywords = {
            "if",
            "for",
            "while",
            "switch",
            "return",
            "const",
            "let",
            "var",
            "function",
            "new",
            "typeof",
            "instanceof",
            "catch",
            "throw",
        }
    else:
        keywords = {"if", "for", "while", "with", "except", "return", "raise", "yield", "lambda", "def", "class"}

    # Match word followed by ( but not keywords
    pattern = r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\("
    matches = re.findall(pattern, body)

    # Filter out keywords
    method_calls = [m for m in matches if m.lower() not in keywords]
    return len(method_calls)


def _normalize_string_escapes(s: str) -> str:
    """Normalize escape sequences in a string for comparison.

    Converts actual newlines/tabs/etc. to their escape sequence representation
    so that Java '\r\n' (actual newline) and Python '\\r\\n' (literal) can be compared.
    """
    # Replace actual control characters with their escape sequences
    s = s.replace("\r", "\\r").replace("\n", "\\n").replace("\t", "\\t")
    return s


def _combine_java_string_concatenation(arg: str) -> str:
    """Combine Java string concatenation expressions into a single string literal.

    Handles patterns like:
    - "str1" + "str2"
    - "str1"\n        + "str2"

    Returns the combined string literal or the original arg if no concatenation found.
    """
    # Check if this looks like a string concatenation
    if "+" not in arg or ('"' not in arg and "'" not in arg):
        return arg

    # Pattern to match string literals: "..." or '...'
    # We'll try to find all string literals and combine them
    parts = []
    i = 0
    while i < len(arg):
        # Skip whitespace
        if arg[i].isspace():
            i += 1
            continue

        # Check for string literal start
        if arg[i] in ('"', "'"):
            quote_char = arg[i]
            i += 1
            string_start = i
            # Find the end of the string literal
            while i < len(arg):
                if arg[i] == "\\":
                    i += 2  # Skip escaped character
                    continue
                if arg[i] == quote_char:
                    # Found end of string literal
                    string_content = arg[string_start:i]
                    parts.append((quote_char, string_content))
                    i += 1
                    break
                i += 1
            continue

        # Check for + operator (string concatenation)
        if arg[i] == "+" and parts:
            # Skip the + and any whitespace after it
            i += 1
            while i < len(arg) and arg[i].isspace():
                i += 1
            continue

        # If we hit something else and we have parts, we might be done
        # But let's try to continue in case there are more strings
        if parts and not arg[i].isspace():
            # Non-whitespace, non-string, non-+ character - probably end of concatenation
            break
        i += 1

    # If we found multiple string parts, combine them
    if len(parts) > 1:
        # Use the quote style of the first part
        quote_char = parts[0][0]
        combined_content = "".join(part[1] for part in parts)
        return f"{quote_char}{combined_content}{quote_char}"

    # If we only found one part or none, return original
    if len(parts) == 1:
        quote_char, content = parts[0]
        return f"{quote_char}{content}{quote_char}"

    return arg


def extract_literal_value(arg: str) -> Tuple[Optional[str], Optional[Any]]:
    """Extract literal value and type from an argument string.

    Args:
        arg: The argument string (may contain expressions)

    Returns:
        Tuple of (type_name, value) or (None, None) if not a literal
        type_name is one of: 'string', 'int', 'float', 'bool'
        For strings, the value is normalized (escape sequences converted to literal form)
    """
    arg = arg.strip()

    # Try to parse as a string literal first (handles both Java and Python style)
    # Check if it starts and ends with matching quotes
    if (arg.startswith('"') and arg.endswith('"')) or (arg.startswith("'") and arg.endswith("'")):
        try:
            # Use ast.literal_eval to properly decode escape sequences
            # This handles both Python single/double quotes and escape sequences
            decoded_value = ast.literal_eval(arg)
            if isinstance(decoded_value, str):
                # Normalize escape sequences for comparison
                normalized = _normalize_string_escapes(decoded_value)
                return ("string", normalized)
        except (ValueError, SyntaxError):
            # If ast.literal_eval fails, try manual extraction for Java-style strings
            # Java uses double quotes and different escape sequences
            if arg.startswith('"') and arg.endswith('"'):
                # Remove outer quotes
                inner = arg[1:-1]
                # Decode common Java escape sequences: \", \\, \r, \n, \t
                # Note: This is a simplified decoder - full Java string decoding is complex
                decoded = (
                    inner.replace('\\"', '"')
                    .replace("\\\\", "\\")
                    .replace("\\r", "\r")
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                )
                # Normalize escape sequences for comparison
                normalized = _normalize_string_escapes(decoded)
                return ("string", normalized)

    # Boolean (Java: true/false, Python: True/False)
    if arg in ("true", "True"):
        return ("bool", True)
    if arg in ("false", "False"):
        return ("bool", False)

    # Integer
    int_match = re.match(r"^-?\d+$", arg)
    if int_match:
        return ("int", int(arg))

    # Float
    float_match = re.match(r"^-?\d+\.\d+$", arg)
    if float_match:
        return ("float", float(arg))

    # null/None
    if arg in ("null", "None"):
        return ("null", None)

    return (None, None)


def extract_assertEquals_args(body: str, lang: str) -> List[Tuple[str, str]]:
    """Extract argument pairs from assertEquals/assertEqual/assert_eq! calls.

    Args:
        body: The test method body text
        lang: Language identifier ('java', 'python', 'go', 'rust')

    Returns:
        List of (arg1, arg2) tuples
    """
    if not body:
        return []

    results = []

    # Pattern to match assertEquals(arg1, arg2) or assertEqual(arg1, arg2) or assert_eq!(arg1, arg2)
    # This is tricky because args can contain nested parentheses
    if lang == "java":
        patterns = [r"\bassertEquals\s*\("]
    elif lang == "rust":
        patterns = [r"\bassert_eq!\s*\(", r"\bassert_ne!\s*\("]
    elif lang == "go":
        # Go doesn't use assertEquals; no pairs to extract
        return []
    elif lang == "javascript":
        patterns = [
            r"\bassert_equal\s*\(",
            r"\bassert\.equal\s*\(",
            r"\bassert\.strictEqual\s*\(",
            r"\bassert\.deepEqual\s*\(",
        ]
    else:
        patterns = [r"\b(?:assertEqual|assertEquals)\s*\("]

    for pattern in patterns:
        # Find all assertEquals calls and extract their arguments
        for match in re.finditer(pattern, body):
            start = match.end()
            # Find the matching closing parenthesis
            paren_count = 1
            pos = start
            arg_start = start
            args = []

            while pos < len(body) and paren_count > 0:
                char = body[pos]
                if char == "(":
                    paren_count += 1
                elif char == ")":
                    paren_count -= 1
                    if paren_count == 0:
                        # End of assertEquals call
                        args.append(body[arg_start:pos].strip())
                elif char == "," and paren_count == 1:
                    # Top-level comma separating arguments
                    args.append(body[arg_start:pos].strip())
                    arg_start = pos + 1
                elif char in ('"', "'"):
                    # Skip string literals
                    quote_char = char
                    pos += 1
                    while pos < len(body) and body[pos] != quote_char:
                        if body[pos] == "\\":
                            pos += 1  # Skip escaped char
                        pos += 1
                pos += 1

            # Java: assertEquals(expected, actual) or assertEquals(message, expected, actual)
            if lang == "java" and len(args) >= 3:
                # 3-arg form: message is first, expected and actual are 2nd and 3rd
                # Handle string concatenation in Java arguments
                arg1 = _combine_java_string_concatenation(args[1])
                arg2 = _combine_java_string_concatenation(args[2])
                results.append((arg1, arg2))
            elif len(args) >= 2:
                # Handle string concatenation in Java arguments
                if lang == "java":
                    arg1 = _combine_java_string_concatenation(args[0])
                    arg2 = _combine_java_string_concatenation(args[1])
                    results.append((arg1, arg2))
                else:
                    results.append((args[0], args[1]))

    return results


def _python_join_line_continuations(body: str) -> str:
    """Join Python lines using explicit backslash continuations: `\\\n`."""
    if not body:
        return ""
    # Replace backslash-newline + indentation with a single space
    return re.sub(r"\\\s*\n\s*", " ", body)


def _strip_top_level_comma_suffix(expr: str) -> str:
    """Strip `, message` from expressions like: `a == b, "msg"` (top-level only)."""
    depth = 0
    quote_char: Optional[str] = None
    escape = False
    for i, ch in enumerate(expr):
        if escape:
            escape = False
            continue
        if quote_char:
            if ch == "\\":
                escape = True
            elif ch == quote_char:
                quote_char = None
            continue
        if ch in ('"', "'"):
            quote_char = ch
            continue
        if ch in "([{":
            depth += 1
            continue
        if ch in ")]}":
            depth = max(0, depth - 1)
            continue
        if ch == "," and depth == 0:
            return expr[:i].strip()
    return expr.strip()


def _find_top_level_double_equals(expr: str) -> Optional[int]:
    """Return index of the first top-level '==' in expr (outside quotes/brackets)."""
    depth = 0
    quote_char: Optional[str] = None
    escape = False
    i = 0
    while i < len(expr) - 1:
        ch = expr[i]
        if escape:
            escape = False
            i += 1
            continue
        if quote_char:
            if ch == "\\":
                escape = True
            elif ch == quote_char:
                quote_char = None
            i += 1
            continue
        if ch in ('"', "'"):
            quote_char = ch
            i += 1
            continue
        if ch in "([{":
            depth += 1
            i += 1
            continue
        if ch in ")]}":
            depth = max(0, depth - 1)
            i += 1
            continue
        if depth == 0 and expr[i : i + 2] == "==":
            return i
        i += 1
    return None


def extract_python_assert_eq_pairs(body: str) -> List[Tuple[str, str]]:
    """Extract pairs from Python `assert <lhs> == <rhs>` lines.

    Notes:
    - Handles `assert a == b, "message"` (drops the message).
    - Joins explicit backslash continuations (`\\\n`) before scanning.
    """
    if not body:
        return []

    body = _python_join_line_continuations(body)
    results: List[Tuple[str, str]] = []

    for raw_line in body.splitlines():
        line = raw_line.strip()
        if not line.startswith("assert "):
            continue
        # Ignore "not equals" comparisons
        if "!=" in line:
            continue
        if "==" not in line:
            continue

        expr = line[len("assert ") :].strip()
        expr = _strip_top_level_comma_suffix(expr)

        # Unwrap a single pair of parens: assert (a == b)
        if expr.startswith("(") and expr.endswith(")"):
            expr = expr[1:-1].strip()

        eq_idx = _find_top_level_double_equals(expr)
        if eq_idx is None:
            continue

        lhs = expr[:eq_idx].strip()
        rhs = expr[eq_idx + 2 :].strip()
        if lhs and rhs:
            results.append((lhs, rhs))

    return results


@dataclass
class AssertionInfo:
    """Information about a single assertion"""

    type: str  # e.g., 'assertEquals', 'assertTrue', etc.
    args: List[str]  # List of argument strings
    line_number: Optional[int] = None  # Optional line number for debugging


def extract_individual_assertions(body: str, lang: str) -> List[AssertionInfo]:
    """Extract individual assertions with their types and arguments from test body.

    Args:
        body: The test method body text
        lang: Language identifier ('java', 'python', 'go', 'rust')

    Returns:
        List of AssertionInfo objects, ordered by appearance in code
    """
    if not body:
        return []

    assertions = []
    assertion_positions = []  # Track positions to maintain order

    if lang == "java":
        # Java assertion patterns with their method names
        patterns = [
            (r"\b(assertEquals)\s*\(", "assertEquals"),
            (r"\b(assertTrue)\s*\(", "assertTrue"),
            (r"\b(assertFalse)\s*\(", "assertFalse"),
            (r"\b(assertNull)\s*\(", "assertNull"),
            (r"\b(assertNotNull)\s*\(", "assertNotNull"),
            (r"\b(assertThrows)\s*\(", "assertThrows"),
            (r"\b(assertSame)\s*\(", "assertSame"),
            (r"\b(assertNotSame)\s*\(", "assertNotSame"),
            (r"\b(assertArrayEquals)\s*\(", "assertArrayEquals"),
            (r"\b(fail)\s*\(", "fail"),
        ]
    elif lang == "go":
        # Go test reporting patterns
        patterns = [
            (r"\b(t\.Errorf)\s*\(", "t.Errorf"),
            (r"\b(t\.Error)\b", "t.Error"),
            (r"\b(t\.Fatalf)\s*\(", "t.Fatalf"),
            (r"\b(t\.Fatal)\b", "t.Fatal"),
            (r"\b(t\.FailNow)\s*\(", "t.FailNow"),
            (r"\b(t\.Fail)\b", "t.Fail"),
        ]
    elif lang == "rust":
        # Rust assertion macro patterns
        patterns = [
            (r"\b(assert_eq)!\s*\(", "assert_eq!"),
            (r"\b(assert_ne)!\s*\(", "assert_ne!"),
            (r"\b(assert)!\s*\(", "assert!"),
            (r"\b(panic)!\s*\(", "panic!"),
            (r"\b(assert_(?!eq!|ne!)\w+)!\s*\(", "assert_*!"),
        ]
    elif lang == "javascript":
        # JavaScript assertion patterns (skel custom helper + console.assert + Node assert module)
        patterns = [
            (r"\b(assert_equal)\s*\(", "assert_equal"),
            (r"\b(console\.assert)\s*\(", "console.assert"),
            (r"\b(assert\.strictEqual)\s*\(", "assert.strictEqual"),
            (r"\b(assert\.deepEqual)\s*\(", "assert.deepEqual"),
            (r"\b(assert\.notEqual)\s*\(", "assert.notEqual"),
            (r"\b(assert\.equal)\s*\(", "assert.equal"),
            (r"\b(assert\.ok)\s*\(", "assert.ok"),
            (r"\b(expect)\s*\(", "expect"),
        ]
    else:
        # Python assertion patterns
        patterns = [
            (r"\b(assertEqual|assertEquals)\s*\(", "assertEqual"),
            (r"\b(assertTrue)\s*\(", "assertTrue"),
            (r"\b(assertFalse)\s*\(", "assertFalse"),
            (r"\b(assertIsNone)\s*\(", "assertIsNone"),
            (r"\b(assertIsNotNone)\s*\(", "assertIsNotNone"),
            (r"\b(assertIsInstance)\s*\(", "assertIsInstance"),
            (r"\b(assertRaises)\s*\(", "assertRaises"),
            (r"\b(assertIn)\s*\(", "assertIn"),
            (r"\b(assertNotIn)\s*\(", "assertNotIn"),
            (r"\b(assertIs)\s*\(", "assertIs"),
            (r"\b(assertIsNot)\s*\(", "assertIsNot"),
            (r"\b(pytest\.raises)\s*\(", "pytest.raises"),
            (r"\b(fail)\s*\(", "fail"),
        ]

    # Extract method-call style assertions
    for pattern, assert_type in patterns:
        for match in re.finditer(pattern, body):
            start = match.end()
            # Find the matching closing parenthesis
            paren_count = 1
            pos = start
            arg_start = start
            args = []

            while pos < len(body) and paren_count > 0:
                char = body[pos]
                if char == "(":
                    paren_count += 1
                elif char == ")":
                    paren_count -= 1
                    if paren_count == 0:
                        args.append(body[arg_start:pos].strip())
                elif char == "," and paren_count == 1:
                    args.append(body[arg_start:pos].strip())
                    arg_start = pos + 1
                elif char in ('"', "'"):
                    quote_char = char
                    pos += 1
                    while pos < len(body) and body[pos] != quote_char:
                        if body[pos] == "\\":
                            pos += 1
                        pos += 1
                pos += 1

            # Handle Java assertEquals with message (3 args)
            if lang == "java" and assert_type == "assertEquals" and len(args) >= 3:
                # Skip message, use expected and actual
                args = args[1:3]

            # Handle string concatenation in Java
            if lang == "java":
                args = [_combine_java_string_concatenation(arg) for arg in args]

            assertion_positions.append((match.start(), AssertionInfo(type=assert_type, args=args, line_number=None)))

    # Also handle Python assert statements (extract separately to maintain order)
    if lang == "python":
        body_for_assert = _python_join_line_continuations(body)
        for line_num, line in enumerate(body_for_assert.splitlines(), 1):
            line_stripped = line.strip()
            if line_stripped.startswith("assert "):
                # Find position in original body
                pos_in_body = body.find(line)
                if pos_in_body < 0:
                    continue

                expr = line_stripped[len("assert ") :].strip()
                expr = _strip_top_level_comma_suffix(expr)

                # Handle assert not x (equivalent to assertFalse)
                if expr.startswith("not "):
                    # Extract the negated expression
                    negated_expr = expr[4:].strip()
                    if negated_expr.startswith("(") and negated_expr.endswith(")"):
                        negated_expr = negated_expr[1:-1].strip()
                    assertion_positions.append(
                        (
                            pos_in_body,
                            AssertionInfo(
                                type="assertFalse",  # assert not x is semantically assertFalse
                                args=[negated_expr],
                                line_number=line_num,
                            ),
                        )
                    )
                # Handle assert x == y
                elif "==" in expr and "!=" not in expr:
                    if expr.startswith("(") and expr.endswith(")"):
                        expr = expr[1:-1].strip()
                    eq_idx = _find_top_level_double_equals(expr)
                    if eq_idx is not None:
                        lhs = expr[:eq_idx].strip()
                        rhs = expr[eq_idx + 2 :].strip()
                        assertion_positions.append(
                            (pos_in_body, AssertionInfo(type="assert_eq", args=[lhs, rhs], line_number=line_num))
                        )
                # Handle other assert statements (bare assert x)
                else:
                    # This is a bare assert statement, treat as assertTrue
                    if expr.startswith("(") and expr.endswith(")"):
                        expr = expr[1:-1].strip()
                    assertion_positions.append(
                        (
                            pos_in_body,
                            AssertionInfo(
                                type="assertTrue",  # assert x is semantically assertTrue
                                args=[expr],
                                line_number=line_num,
                            ),
                        )
                    )

    # Sort by position to maintain order
    assertion_positions.sort(key=lambda x: x[0])
    assertions = [assert_info for _, assert_info in assertion_positions]

    return assertions


# Predefined "good match" mappings for research reporting
# These represent assertion types that are semantically equivalent or acceptable translations
GOOD_MATCH_MAPPINGS = {
    "assertEquals": {
        "assertEqual",  # Direct translation
        "assert_eq",  # assert x == y form
        "assertTrue",  # assertEquals(x, True) -> assertTrue(x)
        "assertFalse",  # assertEquals(x, False) -> assertFalse(x)
        "assertIsNone",  # assertEquals(x, null) -> assertIsNone(x)
        "assertIsNotNone",  # assertEquals(x, not null) -> assertIsNotNone(x)
    },
    "assertTrue": {
        "assertTrue",  # Direct translation
        "assertEqual",  # assertEqual(x, True) or assertEqual(True, x)
        "assert_eq",  # assert x == True or assert True == x
        "assertIn",  # assertTrue(x in y) -> assertIn(x, y)
        "assertIsInstance",  # assertTrue(x instanceof Y) -> assertIsInstance(x, Y)
    },
    "assertFalse": {
        "assertFalse",  # Direct translation
        "assertEqual",  # assertEqual(x, False) or assertEqual(False, x)
        "assert_eq",  # assert x == False or assert False == x
        "assertIsNone",  # assertFalse(x) where x is null check
        "assertIsNot",  # assertFalse(x is y) -> assertIsNot(x, y)
    },
    "assertNull": {
        "assertIsNone",  # Direct translation
        "assertEqual",  # assertEqual(x, None)
        "assert_eq",  # assert x == None
    },
    "assertNotNull": {
        "assertIsNotNone",  # Direct translation
        "assertEqual",  # assertEqual(x, not None)
        "assert_eq",  # assert x != None (but we extract as assertIsNotNone)
    },
    "assertThrows": {
        "assertRaises",  # Direct translation
        "pytest.raises",  # pytest.raises(...)
    },
    "assertSame": {
        "assertIs",  # Direct translation
    },
    "assertNotSame": {
        "assertIsNot",  # Direct translation
    },
    "assertArrayEquals": {
        "assertEqual",  # Direct translation
        "assert_eq",  # assert x == y for arrays
    },
    "fail": {
        "fail",  # Direct translation
        "assertRaises",  # fail in try-catch -> assertRaises
        "pytest.raises",  # fail in try-catch -> pytest.raises
    },
    # ── Go → Rust ────────────────────────────────────────────────────────────
    # In Go, tests report failure via t.Error/t.Errorf/t.Fatal/t.Fatalf.
    # These are all conditional (wrapped in an `if`), so any Rust assertion
    # macro is a valid translation of any of them.
    "t.Errorf": {"assert_eq!", "assert_ne!", "assert!", "panic!", "assert_*!"},
    "t.Error": {"assert_eq!", "assert_ne!", "assert!", "panic!", "assert_*!"},
    "t.Fatalf": {"assert_eq!", "assert_ne!", "assert!", "panic!", "assert_*!"},
    "t.Fatal": {"assert_eq!", "assert_ne!", "assert!", "panic!", "assert_*!"},
    "t.FailNow": {"assert_eq!", "assert_ne!", "assert!", "panic!", "assert_*!"},
    "t.Fail": {"assert_eq!", "assert_ne!", "assert!", "panic!", "assert_*!"},
}


def _is_semantically_compatible(src_type: str, tgt_type: str) -> bool:
    """Check if source and target assertion types are semantically compatible.

    Returns True if the mapping makes sense, False otherwise.
    """
    return tgt_type in GOOD_MATCH_MAPPINGS.get(src_type, set())


def match_assertions(
    source_assertions: List[AssertionInfo], target_assertions: List[AssertionInfo]
) -> Dict[str, Dict[str, int]]:
    """Match assertions between source and target and build type mapping.

    Args:
        source_assertions: List of assertions from source (Java)
        target_assertions: List of assertions from target (Python)

    Returns:
        Dictionary mapping source_assertion_type -> target_assertion_type -> count
    """
    mapping: Dict[str, Dict[str, int]] = {}

    # Strategy: Match by position, but skip semantically incompatible mappings
    # Also handle cases where assertion counts don't match (e.g., try-catch + fail -> assertRaises)

    src_idx = 0
    tgt_idx = 0

    while src_idx < len(source_assertions) and tgt_idx < len(target_assertions):
        src_assert = source_assertions[src_idx]
        tgt_assert = target_assertions[tgt_idx]

        # Check for semantic matches first
        matched = False

        # assertEquals(x, True) -> assertTrue(x)
        if src_assert.type == "assertEquals" and len(src_assert.args) >= 2:
            arg1_type, arg1_val = extract_literal_value(src_assert.args[0])
            arg2_type, arg2_val = extract_literal_value(src_assert.args[1])

            # Check if one argument is a boolean literal
            if arg1_type == "bool" and arg1_val is True:
                if tgt_assert.type == "assertTrue":
                    _increment_mapping(mapping, "assertEquals", "assertTrue")
                    src_idx += 1
                    tgt_idx += 1
                    matched = True
                    continue
            elif arg1_type == "bool" and arg1_val is False:
                if tgt_assert.type == "assertFalse":
                    _increment_mapping(mapping, "assertEquals", "assertFalse")
                    src_idx += 1
                    tgt_idx += 1
                    matched = True
                    continue
            elif arg2_type == "bool" and arg2_val is True:
                if tgt_assert.type == "assertTrue":
                    _increment_mapping(mapping, "assertEquals", "assertTrue")
                    src_idx += 1
                    tgt_idx += 1
                    matched = True
                    continue
            elif arg2_type == "bool" and arg2_val is False:
                if tgt_assert.type == "assertFalse":
                    _increment_mapping(mapping, "assertEquals", "assertFalse")
                    src_idx += 1
                    tgt_idx += 1
                    matched = True
                    continue

            # Check for null/None comparisons
            if arg1_type == "null" or arg2_type == "null":
                if tgt_assert.type in ("assertIsNone", "assertIsNotNone"):
                    _increment_mapping(mapping, "assertEquals", tgt_assert.type)
                    src_idx += 1
                    tgt_idx += 1
                    matched = True
                    continue

            # Check if assertEquals maps to assertEqual or assert_eq
            if tgt_assert.type in ("assertEqual", "assert_eq"):
                _increment_mapping(mapping, "assertEquals", tgt_assert.type)
                src_idx += 1
                tgt_idx += 1
                matched = True
                continue

        # Handle try-catch + fail -> assertRaises pattern
        # If source has assertEquals followed by fail, and target has assertRaises,
        # the assertEquals might be skipped in translation
        if (
            src_assert.type == "assertEquals"
            and src_idx + 1 < len(source_assertions)
            and source_assertions[src_idx + 1].type == "fail"
            and tgt_assert.type in ("assertRaises", "pytest.raises")
        ):
            # Skip the assertEquals, match fail -> assertRaises
            src_idx += 1  # Skip assertEquals
            src_assert = source_assertions[src_idx]  # Now it's the fail
            _increment_mapping(mapping, "fail", tgt_assert.type)
            src_idx += 1
            tgt_idx += 1
            matched = True
            continue

        # Check if assertTrue maps to assert_eq with boolean literal (assert x is True)
        if src_assert.type == "assertTrue" and tgt_assert.type == "assert_eq" and len(tgt_assert.args) >= 2:
            # Check if assert_eq has True/False as one argument
            arg1_type, arg1_val = extract_literal_value(tgt_assert.args[0])
            arg2_type, arg2_val = extract_literal_value(tgt_assert.args[1])
            if (arg1_type == "bool" and arg1_val is True) or (arg2_type == "bool" and arg2_val is True):
                # assert_eq(x, True) is semantically assertTrue(x)
                _increment_mapping(mapping, "assertTrue", "assertTrue")  # Map to assertTrue semantically
                src_idx += 1
                tgt_idx += 1
                matched = True
                continue

        # Direct type-to-type mapping if semantically compatible
        if _is_semantically_compatible(src_assert.type, tgt_assert.type):
            _increment_mapping(mapping, src_assert.type, tgt_assert.type)
            src_idx += 1
            tgt_idx += 1
            matched = True
            continue

        # If not compatible, try to advance one index
        # Prefer advancing target if source has more assertions (might be skipped in translation)
        # Prefer advancing source if target has more assertions (might be additional assertions)
        if len(source_assertions) - src_idx > len(target_assertions) - tgt_idx:
            # More source assertions remaining - advance source (skip unmatched source assertion)
            src_idx += 1
        elif len(target_assertions) - tgt_idx > len(source_assertions) - src_idx:
            # More target assertions remaining - advance target (skip unmatched target assertion)
            tgt_idx += 1
        else:
            # Equal remaining - advance both (both unmatched)
            src_idx += 1
            tgt_idx += 1

    # Track unmatched source assertions (when we run out of target assertions)
    # These represent assertions that exist in source but not in target
    # We could add them to a special "unmatched" category, but for now we just skip them
    # This is expected when target has fewer assertions than source

    return mapping


def _increment_mapping(mapping: Dict[str, Dict[str, int]], source_type: str, target_type: str):
    """Helper to increment assertion type mapping counts."""
    if source_type not in mapping:
        mapping[source_type] = {}
    if target_type not in mapping[source_type]:
        mapping[source_type][target_type] = 0
    mapping[source_type][target_type] += 1


def calculate_assertion_match_percentages(
    assertion_type_mapping: Dict[str, Dict[str, int]], source_assertion_counts: Dict[str, int]
) -> Dict[str, Dict[str, Any]]:
    """Calculate match percentages for each assertion type.

    Args:
        assertion_type_mapping: Global mapping of source_type -> target_type -> count
        source_assertion_counts: Total count of each assertion type in source

    Returns:
        Dictionary mapping source_type -> {
            'good_matches': {target_type: count, ...},
            'good_match_percentage': float,
            'other_matches': {target_type: count, ...},
            'other_match_percentage': float,
            'unmatched_count': int,
            'unmatched_percentage': float,
            'total_mapped': int,
            'total_source': int
        }
    """
    results = {}

    for src_type, target_map in assertion_type_mapping.items():
        total_source = source_assertion_counts.get(src_type, 0)
        if total_source == 0:
            continue

        good_matches = {}
        other_matches = {}
        good_match_set = GOOD_MATCH_MAPPINGS.get(src_type, set())

        total_mapped = 0
        for tgt_type, count in target_map.items():
            total_mapped += count
            if tgt_type in good_match_set:
                good_matches[tgt_type] = count
            else:
                other_matches[tgt_type] = count

        unmatched_count = total_source - total_mapped

        good_match_count = sum(good_matches.values())
        other_match_count = sum(other_matches.values())

        results[src_type] = {
            "good_matches": good_matches,
            "good_match_count": good_match_count,
            "good_match_percentage": round(100 * good_match_count / total_source, 2) if total_source > 0 else 0.0,
            "other_matches": other_matches,
            "other_match_count": other_match_count,
            "other_match_percentage": round(100 * other_match_count / total_source, 2) if total_source > 0 else 0.0,
            "unmatched_count": unmatched_count,
            "unmatched_percentage": round(100 * unmatched_count / total_source, 2) if total_source > 0 else 0.0,
            "total_mapped": total_mapped,
            "total_source": total_source,
        }

    return results


def compare_assertEquals_values(
    source_body: str, target_body: str, source_lang: str = "java", target_lang: str = "python"
) -> Dict[str, Any]:
    """Compare literal values in assertEquals calls between source and target.

    Args:
        source_body: Source test body
        target_body: Target test body
        source_lang: Source language ('java', 'go', 'rust', 'python')
        target_lang: Target language ('java', 'go', 'rust', 'python')

    Returns:
        Dictionary with comparison metrics
    """
    source_args = extract_assertEquals_args(source_body, lang=source_lang)
    target_args = extract_assertEquals_args(target_body, lang=target_lang)
    # Also treat `assert <lhs> == <rhs>` as assertEquals-like on Python side
    target_assert_eq_pairs = extract_python_assert_eq_pairs(target_body)
    target_pairs = list(target_args) + list(target_assert_eq_pairs)

    # Extract literal values from source
    source_literals = []
    for arg1, arg2 in source_args:
        type1, val1 = extract_literal_value(arg1)
        type2, val2 = extract_literal_value(arg2)
        if type1 or type2:
            source_literals.append({"arg1": {"type": type1, "value": val1}, "arg2": {"type": type2, "value": val2}})

    # Extract literal values from target
    target_literals = []
    for arg1, arg2 in target_pairs:
        type1, val1 = extract_literal_value(arg1)
        type2, val2 = extract_literal_value(arg2)
        if type1 or type2:
            target_literals.append({"arg1": {"type": type1, "value": val1}, "arg2": {"type": type2, "value": val2}})

    # Compare: for each source literal pair, try to find a matching target pair
    comparable_count = min(len(source_literals), len(target_literals))
    matching_count = 0

    # Build sets of all literal values from each side
    source_values = set()
    target_values = set()

    for lit in source_literals:
        if lit["arg1"]["type"]:
            source_values.add((lit["arg1"]["type"], lit["arg1"]["value"]))
        if lit["arg2"]["type"]:
            source_values.add((lit["arg2"]["type"], lit["arg2"]["value"]))

    for lit in target_literals:
        if lit["arg1"]["type"]:
            target_values.add((lit["arg1"]["type"], lit["arg1"]["value"]))
        if lit["arg2"]["type"]:
            target_values.add((lit["arg2"]["type"], lit["arg2"]["value"]))

    # Count matching values (intersection)
    matching_values = source_values & target_values

    # For per-assertion matching, check each source assertion against target assertions
    used_target_indices = set()
    for src_lit in source_literals:
        src_vals = set()
        if src_lit["arg1"]["type"]:
            src_vals.add((src_lit["arg1"]["type"], src_lit["arg1"]["value"]))
        if src_lit["arg2"]["type"]:
            src_vals.add((src_lit["arg2"]["type"], src_lit["arg2"]["value"]))

        # Try to find a matching target assertion
        for i, tgt_lit in enumerate(target_literals):
            if i in used_target_indices:
                continue
            tgt_vals = set()
            if tgt_lit["arg1"]["type"]:
                tgt_vals.add((tgt_lit["arg1"]["type"], tgt_lit["arg1"]["value"]))
            if tgt_lit["arg2"]["type"]:
                tgt_vals.add((tgt_lit["arg2"]["type"], tgt_lit["arg2"]["value"]))

            # Check if any value matches (same or cross position)
            if src_vals & tgt_vals:
                matching_count += 1
                used_target_indices.add(i)
                break

    return {
        "source_assertEquals_count": len(source_args),
        "target_assertEqual_count": len(target_args),
        "target_assert_eq_count": len(target_assert_eq_pairs),
        "source_with_literals": len(source_literals),
        "target_with_literals": len(target_literals),
        "comparable_pairs": comparable_count,
        "matching_assertions": matching_count,
        "match_rate": round(matching_count / comparable_count, 4) if comparable_count > 0 else None,
        "unique_literal_values_source": len(source_values),
        "unique_literal_values_target": len(target_values),
        "common_literal_values": len(matching_values),
    }


@dataclass
class ParameterInfo:
    """Information about a method parameter"""

    name: str
    type: str = ""  # Type info available for Java


@dataclass
class TestMethodInfo:
    """Information about a test method"""

    name: str
    signature: str
    parameters: List[ParameterInfo] = field(default_factory=list)
    param_count: int = 0
    assertion_count: int = 0
    body: str = ""
    found: bool = True


@dataclass
class TestComparison:
    """Comparison result for a single test"""

    source_test_path: str
    source_test_name: str
    target_test_path: str
    target_test_name: str
    source_method: Optional[TestMethodInfo] = None
    target_method: Optional[TestMethodInfo] = None
    params_match: Optional[bool] = None
    assertions_match: Optional[bool] = None


class JavaTestParser:
    """Parser for Java test files using tree-sitter"""

    def __init__(self):
        self.language = Language(tsjava.language())
        self.parser = Parser(self.language)

    def parse_test_file(self, file_path: Path) -> Dict[str, TestMethodInfo]:
        """Parse a Java test file and extract test methods.

        Supports both:
        - JUnit4/5 style: methods with @Test annotation
        - JUnit3 style: public void testXxx() methods (extends TestCase)
        """
        if not file_path.exists():
            return {}

        content = file_path.read_bytes()
        tree = self.parser.parse(content)
        methods = {}

        # Query for method declarations with @Test annotation (marker_annotation)
        query_marker = Query(
            self.language,
            """
            (method_declaration
                (modifiers
                    (marker_annotation
                        name: (identifier) @annotation_name))
                name: (identifier) @method_name
                parameters: (formal_parameters) @params
                body: (block) @body) @method
        """,
        )

        # Query for method declarations with @Test(expected=...) annotation (annotation)
        query_annotation = Query(
            self.language,
            """
            (method_declaration
                (modifiers
                    (annotation
                        name: (identifier) @annotation_name))
                name: (identifier) @method_name
                parameters: (formal_parameters) @params
                body: (block) @body) @method
        """,
        )

        # Query for JUnit3-style tests: public void testXxx() methods
        query_junit3 = Query(
            self.language,
            """
            (method_declaration
                (modifiers) @mods
                type: (void_type)
                name: (identifier) @method_name
                parameters: (formal_parameters) @params
                body: (block) @body) @method
        """,
        )

        # Process JUnit4/5 style tests (@Test annotation)
        all_matches = []
        all_matches.extend(run_query_matches(query_marker, tree.root_node))
        all_matches.extend(run_query_matches(query_annotation, tree.root_node))

        for pattern_idx, captures in all_matches:
            annotation_nodes = captures.get("annotation_name", [])
            method_name_nodes = captures.get("method_name", [])
            params_nodes = captures.get("params", [])
            body_nodes = captures.get("body", [])
            method_nodes = captures.get("method", [])

            annotation = annotation_nodes[0].text.decode("utf-8") if annotation_nodes else None
            method_name_node = method_name_nodes[0] if method_name_nodes else None
            params_node = params_nodes[0] if params_nodes else None
            body_node = body_nodes[0] if body_nodes else None
            method_node = method_nodes[0] if method_nodes else None

            if annotation == "Test" and method_name_node:
                method_name = method_name_node.text.decode("utf-8")
                methods[method_name] = self._create_method_info(
                    method_name, method_node, params_node, body_node, content
                )
                # Check if @Test has expected= argument
                if method_node and self._has_test_expected(method_node):
                    methods[method_name].assertion_count += 1

        # Process JUnit3-style tests (public void testXxx methods)
        junit3_matches = run_query_matches(query_junit3, tree.root_node)
        for pattern_idx, captures in junit3_matches:
            mods_nodes = captures.get("mods", [])
            method_name_nodes = captures.get("method_name", [])
            params_nodes = captures.get("params", [])
            body_nodes = captures.get("body", [])
            method_nodes = captures.get("method", [])

            method_name_node = method_name_nodes[0] if method_name_nodes else None
            mods_node = mods_nodes[0] if mods_nodes else None
            params_node = params_nodes[0] if params_nodes else None
            body_node = body_nodes[0] if body_nodes else None
            method_node = method_nodes[0] if method_nodes else None

            if method_name_node:
                method_name = method_name_node.text.decode("utf-8")
                # JUnit3 test methods start with "test" and are public
                if method_name.startswith("test") and method_name not in methods:
                    # Check if it's public
                    if mods_node and "public" in mods_node.text.decode("utf-8"):
                        methods[method_name] = self._create_method_info(
                            method_name, method_node, params_node, body_node, content
                        )

        return methods

    def _create_method_info(
        self, method_name: str, method_node, params_node, body_node, content: bytes
    ) -> TestMethodInfo:
        """Create TestMethodInfo from parsed nodes."""
        parameters = []
        if params_node:
            parameters = self._extract_parameters(params_node)

        body = ""
        if body_node:
            body = body_node.text.decode("utf-8")
            body = body.strip()
            if body.startswith("{") and body.endswith("}"):
                body = body[1:-1].strip()

        signature = self._build_signature(method_node, content) if method_node else ""
        assertion_count = self._count_assertions(body_node) if body_node else 0

        return TestMethodInfo(
            name=method_name,
            signature=signature,
            parameters=parameters,
            param_count=len(parameters),
            assertion_count=assertion_count,
            body=body,
        )

    def _extract_parameters(self, params_node) -> List[ParameterInfo]:
        """Extract parameter information from formal_parameters node"""
        parameters = []

        for child in params_node.children:
            if child.type == "formal_parameter":
                param_type = ""
                param_name = ""

                for param_child in child.children:
                    if param_child.type in (
                        "type_identifier",
                        "array_type",
                        "generic_type",
                        "integral_type",
                        "floating_point_type",
                        "boolean_type",
                    ):
                        param_type = param_child.text.decode("utf-8")
                    elif param_child.type == "identifier":
                        param_name = param_child.text.decode("utf-8")

                if param_name:
                    parameters.append(ParameterInfo(name=param_name, type=param_type))

        return parameters

    def _build_signature(self, method_node, content: bytes) -> str:
        """Build method signature from method declaration"""
        # Get text from start of method to opening brace
        method_text = method_node.text.decode("utf-8")

        # Find the opening brace and return everything before it
        brace_idx = method_text.find("{")
        if brace_idx > 0:
            return method_text[:brace_idx].strip()
        return method_text.split("\n")[0].strip()

    def _count_assertions(self, body_node) -> int:
        """Count assertion method calls in method body.

        Handles:
        - Direct calls: assertEquals(...)
        - Qualified calls: Assert.assertEquals(...)
        - Fully qualified: org.junit.Assert.assertEquals(...)
        """
        count = 0

        def walk_node(node):
            nonlocal count

            if node.type == "method_invocation":
                # In tree-sitter Java, method_invocation structure is:
                # - Direct: identifier (method_name) + argument_list
                # - Qualified: identifier (object) + "." + identifier (method_name) + argument_list
                # - Fully qualified: field_access (package.object) + "." + identifier (method_name) + argument_list
                # The method name is always the last identifier before argument_list

                method_name = None
                children_list = list(node.children)

                # Find the argument_list and work backwards to find the method name
                for i in range(len(children_list) - 1, -1, -1):
                    child = children_list[i]
                    if child.type == "argument_list":
                        # Look backwards for the method name (identifier before argument_list)
                        # Skip over "." tokens
                        for j in range(i - 1, -1, -1):
                            prev_child = children_list[j]
                            if prev_child.type == "identifier":
                                method_name = prev_child.text.decode("utf-8")
                                break
                            elif prev_child.type not in (".", "field_access", "scoped_identifier"):
                                # Stop if we hit something else
                                break
                        break

                # Count assert* and fail methods
                if method_name and (method_name.startswith("assert") or method_name == "fail"):
                    count += 1

            for child in node.children:
                walk_node(child)

        walk_node(body_node)
        return count

    def _has_test_expected(self, method_node) -> bool:
        """Check if @Test annotation has 'expected' argument (e.g., @Test(expected = Exception.class))"""
        # Look for annotation node in modifiers
        for child in method_node.children:
            if child.type == "modifiers":
                for mod_child in child.children:
                    if mod_child.type == "annotation":
                        # Check if it's @Test with arguments
                        annotation_text = mod_child.text.decode("utf-8")
                        if "Test" in annotation_text and "expected" in annotation_text:
                            return True
        return False


def _body_text_without_comments(body_node, comment_types: set) -> str:
    """Return the body node's text with all comment nodes removed.

    Uses the tree-sitter AST to precisely locate comment spans so that
    assertion patterns in comments are not counted.

    Args:
        body_node: A tree-sitter node (e.g. the block/body of a function)
        comment_types: Set of tree-sitter node type strings that are comments
                       e.g. {'comment'} for Go, {'line_comment', 'block_comment'} for Rust

    Returns:
        Source text of the node with comment content blanked out (replaced with
        spaces so byte offsets of non-comment code are preserved).
    """
    if body_node is None:
        return ""

    raw = body_node.text  # bytes
    base = body_node.start_byte

    # Collect byte ranges of all comment nodes (recursive)
    comment_ranges: List[Tuple[int, int]] = []

    def collect_comments(node):
        if node.type in comment_types:
            comment_ranges.append((node.start_byte - base, node.end_byte - base))
        for child in node.children:
            collect_comments(child)

    collect_comments(body_node)

    if not comment_ranges:
        return raw.decode("utf-8")

    # Build cleaned bytes: blank out comment spans with spaces
    result = bytearray(raw)
    for start, end in comment_ranges:
        for i in range(start, end):
            result[i] = ord(" ")

    return result.decode("utf-8")


class GoTestParser:
    """Parser for Go test files using tree-sitter"""

    def __init__(self):
        if not HAS_TREE_SITTER_GO:
            raise ImportError("tree-sitter-go is not installed. Run: pip install tree-sitter-go")
        self.language = Language(tsgo.language())
        self.parser = Parser(self.language)

    def parse_test_file(self, file_path: Path) -> Dict[str, TestMethodInfo]:
        """Parse a Go test file and extract test functions.

        Go test functions follow the pattern: func TestXxx(t *testing.T) { ... }
        """
        if not file_path.exists():
            return {}

        try:
            content = file_path.read_bytes()
            tree = self.parser.parse(content)
            methods = {}

            query = Query(
                self.language,
                """
                (function_declaration
                    name: (identifier) @func_name
                    parameters: (parameter_list) @params
                    body: (block) @body) @func
            """,
            )

            matches = run_query_matches(query, tree.root_node)

            for pattern_idx, captures in matches:
                func_name_nodes = captures.get("func_name", [])
                params_nodes = captures.get("params", [])
                body_nodes = captures.get("body", [])
                func_nodes = captures.get("func", [])

                func_name_node = func_name_nodes[0] if func_name_nodes else None
                params_node = params_nodes[0] if params_nodes else None
                body_node = body_nodes[0] if body_nodes else None
                func_node = func_nodes[0] if func_nodes else None

                if func_name_node:
                    method_name = func_name_node.text.decode("utf-8")
                    # Go test functions: Test*, Example*, and Benchmark* prefixes
                    if (
                        method_name.startswith("Test")
                        or method_name.startswith("test")
                        or method_name.startswith("Example")
                        or method_name.startswith("Benchmark")
                    ):
                        parameters = self._extract_parameters(params_node) if params_node else []
                        full_method = func_node.text.decode("utf-8") if func_node else ""
                        signature = self._build_signature(func_node) if func_node else ""
                        assertion_count = self._count_assertions(body_node) if body_node else 0

                        methods[method_name] = TestMethodInfo(
                            name=method_name,
                            signature=signature,
                            parameters=parameters,
                            param_count=len(parameters),
                            assertion_count=assertion_count,
                            body=full_method,
                        )

            return methods
        except Exception as e:
            print(f"Warning: Could not parse {file_path}: {e}")
            return {}

    def _extract_parameters(self, params_node) -> List[ParameterInfo]:
        """Extract parameter information from parameter_list node."""
        parameters = []
        for child in params_node.children:
            if child.type == "parameter_declaration":
                param_name = ""
                param_type = ""
                for param_child in child.children:
                    if param_child.type == "identifier" and not param_name:
                        param_name = param_child.text.decode("utf-8")
                    elif param_child.type not in ("identifier", ",", "(", ")"):
                        param_type = param_child.text.decode("utf-8")
                if param_name:
                    parameters.append(ParameterInfo(name=param_name, type=param_type))
        return parameters

    def _build_signature(self, func_node) -> str:
        """Build function signature (first line of the function)."""
        func_text = func_node.text.decode("utf-8")
        lines = func_text.split("\n")
        return lines[0].strip() if lines else ""

    def _count_assertions(self, body_node) -> int:
        """Count Go test assertion calls: t.Error, t.Errorf, t.Fatal, t.Fatalf, t.FailNow, t.Fail."""
        body_text = _body_text_without_comments(body_node, {"comment"})
        count = 0
        patterns = [
            r"\bt\.Errorf?\s*\(",
            r"\bt\.Fatalf?\s*\(",
            r"\bt\.FailNow\s*\(",
            r"\bt\.Fail\s*\(",
            r"\bt\.Log\s*\(",
        ]
        for pattern in patterns:
            count += len(re.findall(pattern, body_text))
        return count


class RustTestParser:
    """Parser for Rust test files using tree-sitter"""

    def __init__(self):
        if not HAS_TREE_SITTER_RUST:
            raise ImportError("tree-sitter-rust is not installed. Run: pip install tree-sitter-rust")
        self.language = Language(tsrust.language())
        self.parser = Parser(self.language)

    def parse_test_file(self, file_path: Path) -> Dict[str, TestMethodInfo]:
        """Parse a Rust test file and extract test functions.

        Rust test functions are annotated with #[test].
        """
        if not file_path.exists():
            return {}

        try:
            content = file_path.read_bytes()
            tree = self.parser.parse(content)
            methods = {}

            self._collect_tests_from_node(tree.root_node, methods)

            return methods
        except Exception as e:
            print(f"Warning: Could not parse {file_path}: {e}")
            return {}

    def _collect_tests_from_node(self, container_node, methods: dict):
        """Recursively collect #[test] functions from a node's children.

        Handles both top-level tests and tests nested inside
        #[cfg(test)] mod blocks (declaration_list children).
        """
        children = list(container_node.children)
        i = 0
        while i < len(children):
            node = children[i]

            if node.type == "attribute_item":
                attr_text = node.text.decode("utf-8").strip()
                if "test" in attr_text and "cfg" not in attr_text:
                    # Look ahead for the function_item (skip other attribute_items)
                    func_node = None
                    for j in range(i + 1, min(i + 4, len(children))):
                        if children[j].type == "function_item":
                            func_node = children[j]
                            break
                        elif children[j].type == "attribute_item":
                            continue
                        else:
                            break
                    if func_node:
                        method_name, method_info = self._extract_function_info(func_node)
                        if method_name:
                            methods[method_name] = method_info

            elif node.type == "mod_item":
                # Descend into mod blocks (e.g. #[cfg(test)] mod tests { ... })
                for child in node.children:
                    if child.type == "declaration_list":
                        self._collect_tests_from_node(child, methods)

            i += 1

    def _extract_function_info(self, func_node) -> tuple:
        """Extract TestMethodInfo from a Rust function_item node."""
        method_name = ""
        params_node = None
        body_node = None

        for child in func_node.children:
            if child.type == "identifier" and not method_name:
                method_name = child.text.decode("utf-8")
            elif child.type == "parameters":
                params_node = child
            elif child.type == "block":
                body_node = child

        if not method_name:
            return None, None

        parameters = self._extract_parameters(params_node) if params_node else []
        full_method = func_node.text.decode("utf-8")
        signature = self._build_signature(func_node)
        assertion_count = self._count_assertions(body_node) if body_node else 0

        return method_name, TestMethodInfo(
            name=method_name,
            signature=signature,
            parameters=parameters,
            param_count=len(parameters),
            assertion_count=assertion_count,
            body=full_method,
        )

    def _extract_parameters(self, params_node) -> List[ParameterInfo]:
        """Extract parameter information from Rust parameters node."""
        parameters = []
        for child in params_node.children:
            if child.type == "parameter":
                param_name = ""
                param_type = ""
                for param_child in child.children:
                    if param_child.type == "identifier" and not param_name:
                        param_name = param_child.text.decode("utf-8")
                    elif param_child.type not in ("identifier", ":", ","):
                        param_type = param_child.text.decode("utf-8")
                if param_name:
                    parameters.append(ParameterInfo(name=param_name, type=param_type))
        return parameters

    def _build_signature(self, func_node) -> str:
        """Build function signature (first line of the function)."""
        func_text = func_node.text.decode("utf-8")
        lines = func_text.split("\n")
        return lines[0].strip() if lines else ""

    def _count_assertions(self, body_node) -> int:
        """Count Rust assertion macros: assert_eq!, assert!, assert_ne!, panic!."""
        body_text = _body_text_without_comments(body_node, {"line_comment", "block_comment"})
        count = 0
        # Single unified pattern covering all assert variants and panic
        patterns = [
            r"\bassert_eq!\s*\(",
            r"\bassert_ne!\s*\(",
            r"\bassert!\s*\(",
            r"\bpanic!\s*\(",
            r"\bassert_(?!eq!|ne!)(\w+)!\s*\(",
        ]
        seen_positions = set()
        for pattern in patterns:
            for match in re.finditer(pattern, body_text):
                start = match.start()
                if start not in seen_positions:
                    seen_positions.add(start)
                    count += 1
        return count


class PythonTestParser:
    """Parser for Python test files using tree-sitter"""

    def __init__(self):
        self.language = Language(tspython.language())
        self.parser = Parser(self.language)

    def parse_test_file(self, file_path: Path) -> Dict[str, TestMethodInfo]:
        """Parse a Python test file and extract test methods"""
        if not file_path.exists():
            return {}

        try:
            content = file_path.read_bytes()
            tree = self.parser.parse(content)
            methods = {}

            # Query for function definitions
            query = Query(
                self.language,
                """
                (function_definition
                    name: (identifier) @func_name
                    parameters: (parameters) @params
                    body: (block) @body) @func
            """,
            )

            # Use run_query_matches() which works with both old and new tree-sitter versions
            matches = run_query_matches(query, tree.root_node)

            # Process each match
            for pattern_idx, captures in matches:
                # Get nodes from captures (each capture is a list of nodes)
                func_name_nodes = captures.get("func_name", [])
                params_nodes = captures.get("params", [])
                body_nodes = captures.get("body", [])
                func_nodes = captures.get("func", [])

                # Get first node from each (if exists)
                func_name_node = func_name_nodes[0] if func_name_nodes else None
                params_node = params_nodes[0] if params_nodes else None
                body_node = body_nodes[0] if body_nodes else None
                func_node = func_nodes[0] if func_nodes else None

                # Get function name and check if it's a test
                if func_name_node:
                    method_name = func_name_node.text.decode("utf-8")

                    if "test" in method_name.lower():
                        # Extract parameters
                        parameters = []
                        if params_node:
                            parameters = self._extract_parameters(params_node)

                        # Extract full method (signature + body)
                        full_method = func_node.text.decode("utf-8") if func_node else ""

                        # Build signature
                        signature = self._build_signature(func_node) if func_node else ""

                        # Count assertions
                        assertion_count = self._count_assertions(body_node) if body_node else 0

                        methods[method_name] = TestMethodInfo(
                            name=method_name,
                            signature=signature,
                            parameters=parameters,
                            param_count=len(parameters),
                            assertion_count=assertion_count,
                            body=full_method,
                        )

            return methods
        except Exception as e:
            print(f"Warning: Could not parse {file_path}: {e}")
            return {}

    def _extract_parameters(self, params_node) -> List[ParameterInfo]:
        """Extract parameter information from parameters node"""
        parameters = []

        for child in params_node.children:
            if child.type == "identifier":
                param_name = child.text.decode("utf-8")
                # Skip 'self' parameter
                if param_name != "self":
                    parameters.append(ParameterInfo(name=param_name))
            elif child.type == "typed_parameter":
                param_name = ""
                param_type = ""
                for param_child in child.children:
                    if param_child.type == "identifier":
                        param_name = param_child.text.decode("utf-8")
                    elif param_child.type == "type":
                        param_type = param_child.text.decode("utf-8")
                if param_name and param_name != "self":
                    parameters.append(ParameterInfo(name=param_name, type=param_type))
            elif child.type == "default_parameter":
                for param_child in child.children:
                    if param_child.type == "identifier":
                        param_name = param_child.text.decode("utf-8")
                        if param_name != "self":
                            parameters.append(ParameterInfo(name=param_name))
                        break
            elif child.type == "typed_default_parameter":
                param_name = ""
                param_type = ""
                for param_child in child.children:
                    if param_child.type == "identifier":
                        param_name = param_child.text.decode("utf-8")
                    elif param_child.type == "type":
                        param_type = param_child.text.decode("utf-8")
                if param_name and param_name != "self":
                    parameters.append(ParameterInfo(name=param_name, type=param_type))

        return parameters

    def _build_signature(self, func_node) -> str:
        """Build function signature from function definition"""
        # Get the first line (def line)
        func_text = func_node.text.decode("utf-8")
        lines = func_text.split("\n")

        # Get signature line(s) - may span multiple lines
        signature_lines = []
        for line in lines:
            signature_lines.append(line)
            if ":" in line and not line.strip().startswith("#"):
                break

        return "\n".join(signature_lines).strip()

    def _count_assertions(self, body_node) -> int:
        """Count assertion statements and calls in function body"""
        count = 0

        def walk_node(node):
            nonlocal count

            # Count 'assert' statements
            if node.type == "assert_statement":
                count += 1
            # Count method calls like pytest.raises, self.assert*, etc.
            elif node.type == "call":
                func_node = None
                for child in node.children:
                    if child.type in ("identifier", "attribute"):
                        func_node = child
                        break

                if func_node:
                    func_text = func_node.text.decode("utf-8")
                    # Count pytest.raises, pytest.fail, etc.
                    if "raises" in func_text or "fail" in func_text:
                        count += 1
                    # Count self.assert* or self._assert* methods
                    elif "assert" in func_text.lower():
                        count += 1

            for child in node.children:
                walk_node(child)

        walk_node(body_node)
        return count


class JavaScriptTestParser:
    """Parser for JavaScript test files using tree-sitter (regex fallback if unavailable)"""

    def __init__(self):
        if HAS_TREE_SITTER_JS:
            self.language = Language(tsjavascript.language())
            self.parser = Parser(self.language)
        else:
            self.language = None
            self.parser = None

    def parse_test_file(self, file_path: Path) -> Dict[str, TestMethodInfo]:
        """Parse a JavaScript test file and extract test functions"""
        if not file_path.exists():
            return {}

        try:
            content = file_path.read_text(encoding="utf-8")

            if HAS_TREE_SITTER_JS:
                return self._parse_with_tree_sitter(file_path, content)
            else:
                return self._parse_with_regex(content)
        except Exception as e:
            print(f"Warning: Could not parse {file_path}: {e}")
            return {}

    def _parse_with_tree_sitter(self, file_path: Path, content: str) -> Dict[str, TestMethodInfo]:
        """Parse using tree-sitter for accurate AST-based extraction"""
        content_bytes = content.encode("utf-8")
        tree = self.parser.parse(content_bytes)
        methods = {}

        query = Query(
            self.language,
            """
            (function_declaration
                name: (identifier) @func_name
                parameters: (formal_parameters) @params
                body: (statement_block) @body) @func
            """,
        )

        matches = run_query_matches(query, tree.root_node)

        for pattern_idx, captures in matches:
            func_name_nodes = captures.get("func_name", [])
            params_nodes = captures.get("params", [])
            func_nodes = captures.get("func", [])

            func_name_node = func_name_nodes[0] if func_name_nodes else None
            params_node = params_nodes[0] if params_nodes else None
            func_node = func_nodes[0] if func_nodes else None

            if func_name_node:
                method_name = func_name_node.text.decode("utf-8")

                if "test" in method_name.lower():
                    parameters = []
                    if params_node:
                        parameters = self._extract_parameters(params_node)

                    full_method = func_node.text.decode("utf-8") if func_node else ""
                    signature = self._build_signature(func_node) if func_node else ""
                    assertion_count = self._count_assertions_text(full_method)

                    methods[method_name] = TestMethodInfo(
                        name=method_name,
                        signature=signature,
                        parameters=parameters,
                        param_count=len(parameters),
                        assertion_count=assertion_count,
                        body=full_method,
                    )

        return methods

    def _parse_with_regex(self, content: str) -> Dict[str, TestMethodInfo]:
        """Regex-based fallback parser for JavaScript test functions"""
        methods = {}

        # Match top-level function declarations for any function whose name contains "test"
        pattern = re.compile(r"^function\s+(\w*[Tt]est\w*)\s*\(([^)]*)\)\s*\{", re.MULTILINE)

        for match in pattern.finditer(content):
            method_name = match.group(1)
            params_str = match.group(2).strip()

            # Extract body by matching braces
            body_start = match.end() - 1  # position of opening {
            body_text = self._extract_brace_body(content, body_start)
            full_method = content[match.start() : match.start() + len(match.group(0)) - 1 + len(body_text)]

            # Parse parameters
            parameters = []
            if params_str:
                for p in params_str.split(","):
                    p = p.strip()
                    if p:
                        parameters.append(ParameterInfo(name=p))

            signature = f"function {method_name}({params_str})"
            assertion_count = self._count_assertions_text(full_method)

            methods[method_name] = TestMethodInfo(
                name=method_name,
                signature=signature,
                parameters=parameters,
                param_count=len(parameters),
                assertion_count=assertion_count,
                body=full_method,
            )

        return methods

    def _extract_brace_body(self, content: str, brace_pos: int) -> str:
        """Extract the content of a brace-delimited block starting at brace_pos"""
        depth = 0
        pos = brace_pos
        in_string = False
        string_char = ""
        in_line_comment = False
        in_block_comment = False

        while pos < len(content):
            ch = content[pos]

            if in_line_comment:
                if ch == "\n":
                    in_line_comment = False
            elif in_block_comment:
                if ch == "*" and pos + 1 < len(content) and content[pos + 1] == "/":
                    in_block_comment = False
                    pos += 1
            elif in_string:
                if ch == "\\":
                    pos += 1  # skip escaped char
                elif ch == string_char:
                    in_string = False
            else:
                if ch in ('"', "'", "`"):
                    in_string = True
                    string_char = ch
                elif ch == "/" and pos + 1 < len(content) and content[pos + 1] == "/":
                    in_line_comment = True
                elif ch == "/" and pos + 1 < len(content) and content[pos + 1] == "*":
                    in_block_comment = True
                elif ch == "{":
                    depth += 1
                elif ch == "}":
                    depth -= 1
                    if depth == 0:
                        return content[brace_pos : pos + 1]
            pos += 1

        return content[brace_pos:]

    def _extract_parameters(self, params_node) -> List[ParameterInfo]:
        """Extract parameter information from formal_parameters node"""
        parameters = []
        for child in params_node.children:
            if child.type == "identifier":
                param_name = child.text.decode("utf-8")
                parameters.append(ParameterInfo(name=param_name))
            elif child.type in ("assignment_pattern", "rest_pattern"):
                # Default or rest params: extract the name part
                for sub in child.children:
                    if sub.type == "identifier":
                        parameters.append(ParameterInfo(name=sub.text.decode("utf-8")))
                        break
        return parameters

    def _build_signature(self, func_node) -> str:
        """Build function signature (first line up to opening brace)"""
        func_text = func_node.text.decode("utf-8")
        lines = func_text.split("\n")
        signature_lines = []
        for line in lines:
            signature_lines.append(line)
            if "{" in line:
                break
        return "\n".join(signature_lines).strip()

    def _count_assertions_text(self, body: str) -> int:
        """Count assertion calls in JavaScript body text"""
        patterns = [
            r"\bassert_equal\s*\(",
            r"\bconsole\.assert\s*\(",
            r"\bassert\.strictEqual\s*\(",
            r"\bassert\.deepEqual\s*\(",
            r"\bassert\.notEqual\s*\(",
            r"\bassert\.equal\s*\(",
            r"\bassert\.ok\s*\(",
            r"\bexpect\s*\(",
        ]
        count = 0
        for pattern in patterns:
            count += len(re.findall(pattern, body))
        return count


class TestComparator:
    """Main comparator class"""

    def __init__(
        self,
        mapping_csv: Path,
        source_lang_base: Path,
        target_lang_base: Path,
        superclass_map: Optional[Dict[str, str]] = None,
        compute_similarity: bool = False,
    ):
        self.mapping_csv = mapping_csv
        self.source_lang_base = source_lang_base
        self.target_lang_base = target_lang_base
        self.comparisons: List[TestComparison] = []
        # Map of class name -> superclass name for inherited test lookups
        self.superclass_map = superclass_map or {}
        # Whether to compute embedding similarity (slow, loads model)
        self.compute_similarity = compute_similarity

        # Determine language types from base paths
        self.source_lang = self._detect_language(source_lang_base)
        self.target_lang = self._detect_language(target_lang_base)

        if self.source_lang == "java":
            self.source_parser = JavaTestParser()
        elif self.source_lang == "go":
            self.source_parser = GoTestParser()
        elif self.source_lang == "rust":
            self.source_parser = RustTestParser()
        elif self.source_lang == "javascript":
            self.source_parser = JavaScriptTestParser()
        else:
            self.source_parser = PythonTestParser()

        if self.target_lang == "java":
            self.target_parser = JavaTestParser()
        elif self.target_lang == "go":
            self.target_parser = GoTestParser()
        elif self.target_lang == "rust":
            self.target_parser = RustTestParser()
        elif self.target_lang == "javascript":
            self.target_parser = JavaScriptTestParser()
        else:
            self.target_parser = PythonTestParser()

    def _detect_language(self, base_path: Path) -> str:
        """Detect language from base path structure.

        The base_path is now the test root directory (e.g., java/src/test/java or python/src/test).
        For oxidizer projects: go/ or rust/ directories.
        We detect by checking path names or looking at file extensions.
        """
        path_str = str(base_path).lower()

        # Check path components for language hints
        parts = base_path.parts

        # Check for oxidizer languages first (go/ and rust/ are top-level dirs in the project)
        if "go" in parts:
            return "go"
        if "rust" in parts:
            return "rust"

        # Check for skel JavaScript projects
        if "javascript" in parts:
            return "javascript"

        # Look for 'python' in path FIRST (before checking for 'java' at the end)
        # This handles cases like python/src/test/java where the last part is 'java'
        if "python" in parts:
            return "python"

        # Look for 'java' in path (especially at the end for Java test dirs)
        if parts and parts[-1] == "java":
            return "java"

        # Fallback: check for .java or .py files in the tree
        # Look one level deep for 'org' directory and check extensions
        org_dir = base_path / "org"
        if org_dir.exists():
            # Walk to find a file and check extension
            for p in org_dir.rglob("*"):
                if p.is_file():
                    if p.suffix == ".java":
                        return "java"
                    elif p.suffix == ".py":
                        return "python"
                    break

        # Final fallback based on directory name anywhere in path
        if "java" in path_str:
            return "java"
        elif "python" in path_str:
            return "python"

        return "unknown"

    def _package_to_file_path(self, package_path: str, is_source: bool) -> Path:
        """Convert package path to file system path.

        Args:
            package_path: For Java/Python: dot-separated package path
                          (e.g., org.apache.commons.cli.ApplicationTest).
                          For Go/Rust: direct relative file path
                          (e.g., cosine_test.go or tests/cosine_test.rs).
            is_source: True if looking for source file, False for target file

        Assumes self.source_lang_base and self.target_lang_base already point to
        the correct test root directory (e.g., java/src/test/java, python/src/test,
        go/, rust/).
        """
        if is_source:
            base = self.source_lang_base
            lang = self.source_lang
        else:
            base = self.target_lang_base
            lang = self.target_lang

        # For Go, Rust, and JavaScript, paths in the CSV are already relative file paths
        if lang in ("go", "rust", "javascript"):
            if not package_path:
                # Return a sentinel non-existent path so parsers return {}
                return base / "__missing__"
            return base / Path(package_path)

        # For Python/Java paths that are direct filenames (e.g. skel's "source.py"),
        # treat them as relative file paths rather than dot-separated package names
        if package_path.endswith(".py") or package_path.endswith(".java"):
            if not package_path:
                return base / "__missing__"
            return base / Path(package_path)

        # For Java/Python: split dot-separated package path into directory components
        parts = package_path.split(".")
        class_name = parts[-1]
        package_parts = parts[:-1]

        ext = ".java" if lang == "java" else ".py"

        # Base path should already be the test root, just append package path
        file_path = base / Path(*package_parts) / f"{class_name}{ext}"
        return file_path

    def load_mappings(self) -> List[Dict[str, str]]:
        """Load test mappings from CSV"""
        mappings = []
        with open(self.mapping_csv, "r", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            for row in reader:
                mappings.append(row)
        return mappings

    def compare_all_tests(self):
        """Compare all tests in the mapping"""
        mappings = self.load_mappings()

        for mapping in mappings:
            # Determine source and target column names based on language
            # AlphaTrans CSV: project, java test path, java test name, python test path, python test name
            # Oxidizer CSV:   project, go test path,   go test name,   rust test path,   rust test name
            if self.source_lang == "java":
                source_path_col = "java test path"
                source_name_col = "java test name"
            elif self.source_lang == "go":
                source_path_col = "go test path"
                source_name_col = "go test name"
            elif self.source_lang == "rust":
                source_path_col = "rust test path"
                source_name_col = "rust test name"
            elif self.source_lang == "javascript":
                source_path_col = "javascript test path"
                source_name_col = "javascript test name"
            else:
                source_path_col = "python test path"
                source_name_col = "python test name"

            if self.target_lang == "java":
                target_path_col = "java test path"
                target_name_col = "java test name"
            elif self.target_lang == "go":
                target_path_col = "go test path"
                target_name_col = "go test name"
            elif self.target_lang == "rust":
                target_path_col = "rust test path"
                target_name_col = "rust test name"
            elif self.target_lang == "javascript":
                target_path_col = "javascript test path"
                target_name_col = "javascript test name"
            else:
                target_path_col = "python test path"
                target_name_col = "python test name"

            # Get values from mapping
            source_test_path = mapping.get(source_path_col, "")
            source_test_name = mapping.get(source_name_col, "")
            target_test_path = mapping.get(target_path_col, "")
            target_test_name = mapping.get(target_name_col, "")

            comparison = self._compare_single_test(
                source_test_path, source_test_name, target_test_path, target_test_name
            )
            self.comparisons.append(comparison)

    def _compare_single_test(
        self, source_test_path: str, source_test_name: str, target_test_path: str, target_test_name: str
    ) -> TestComparison:
        """Compare a single test method"""
        comparison = TestComparison(
            source_test_path=source_test_path,
            source_test_name=source_test_name,
            target_test_path=target_test_path,
            target_test_name=target_test_name,
        )

        # Convert package paths to file paths
        source_file = self._package_to_file_path(source_test_path, is_source=True)
        target_file = self._package_to_file_path(target_test_path, is_source=False)

        # Parse source file
        source_methods = self.source_parser.parse_test_file(source_file)
        source_method = source_methods.get(source_test_name)

        # If method not found, try looking in superclass (for Java inheritance)
        if not source_method and self.superclass_map:
            source_method = self._lookup_in_superclass(source_test_path, source_test_name, is_source=True)

        # Parse target file
        target_methods = self.target_parser.parse_test_file(target_file)
        target_method = target_methods.get(target_test_name)

        # If method not found in target, try looking in superclass (for Python inheritance)
        if not target_method and self.superclass_map:
            target_method = self._lookup_in_superclass(target_test_path, target_test_name, is_source=False)

        comparison.source_method = source_method
        comparison.target_method = target_method

        if source_method and target_method:
            comparison.params_match = source_method.param_count == target_method.param_count
            comparison.assertions_match = source_method.assertion_count == target_method.assertion_count
        elif not source_method:
            comparison.params_match = False
            comparison.assertions_match = False
        elif not target_method:
            comparison.params_match = False
            comparison.assertions_match = False

        return comparison

    def _lookup_in_superclass(self, test_path: str, test_name: str, is_source: bool) -> Optional[TestMethodInfo]:
        """Look up a test method in the superclass if the class has a mapped superclass.

        Args:
            test_path: The package path (e.g., org.apache.commons.validator.routines.BigDecimalValidatorTest)
            test_name: The test method name
            is_source: True for source (Java), False for target (Python)
        """
        # Extract class name from package path
        class_name = test_path.split(".")[-1]

        # Check if this class has a mapped superclass
        superclass_name = self.superclass_map.get(class_name)
        if not superclass_name:
            return None

        # Build the superclass package path (same package, different class name)
        package_parts = test_path.split(".")[:-1]
        superclass_path = ".".join(package_parts + [superclass_name])

        # Get the file path for the superclass
        superclass_file = self._package_to_file_path(superclass_path, is_source=is_source)

        # Parse the superclass file and look for the method
        parser = self.source_parser if is_source else self.target_parser
        superclass_methods = parser.parse_test_file(superclass_file)
        return superclass_methods.get(test_name)

    def generate_report(self, output_file: Path):
        """Generate comparison report"""
        total_tests = len(self.comparisons)

        params_match_count = sum(1 for c in self.comparisons if c.params_match is True)
        params_mismatch_count = sum(1 for c in self.comparisons if c.params_match is False)

        assertions_match_count = sum(1 for c in self.comparisons if c.assertions_match is True)
        assertions_mismatch_count = sum(1 for c in self.comparisons if c.assertions_match is False)

        with open(output_file, "w", encoding="utf-8") as f:
            f.write(f"# tests: {total_tests}\n")
            f.write(f"# tests with matching number of parameters: {params_match_count}\n")
            f.write(f"# tests with mismatching number of parameters: {params_mismatch_count}\n")
            f.write(f"# tests with matching number of assertions: {assertions_match_count}\n")
            f.write(f"# tests with mismatching number of assertions: {assertions_mismatch_count}\n")

        print(f"Report generated: {output_file}")
        print(f"Total tests: {total_tests}")
        print(f"Parameters match: {params_match_count}, mismatch: {params_mismatch_count}")
        print(f"Assertions match: {assertions_match_count}, mismatch: {assertions_mismatch_count}")

    def generate_json_report(self, output_file: Path):
        """Generate detailed JSON report with metrics for each test pair"""
        total_tests = len(self.comparisons)
        both_found = sum(1 for c in self.comparisons if c.source_method and c.target_method)
        source_missing = sum(1 for c in self.comparisons if not c.source_method)
        target_missing = sum(1 for c in self.comparisons if not c.target_method)

        # Aggregators for summary metrics
        line_counts_source = []
        line_counts_target = []
        similarity_scores = []
        method_call_counts_source = []
        method_call_counts_target = []
        assertion_types_source_agg = {}
        assertion_types_target_agg = {}
        assertEquals_total_comparable = 0
        assertEquals_total_matching = 0
        # Global assertion type mapping: source_type -> target_type -> count
        global_assertion_mapping: Dict[str, Dict[str, int]] = {}
        # Same as above but only for test pairs where assertion count matches
        global_assertion_mapping_same_count: Dict[str, Dict[str, int]] = {}
        assertion_types_source_agg_same_count: Dict[str, int] = {}

        report_data = {
            "summary": {
                "total_tests": total_tests,
                "both_found": both_found,
                "source_missing": source_missing,
                "target_missing": target_missing,
                "both_found_pct": round(100 * both_found / total_tests, 1) if total_tests > 0 else 0,
                "params_match_count": sum(1 for c in self.comparisons if c.params_match is True),
                "params_mismatch_count": sum(1 for c in self.comparisons if c.params_match is False),
                "assertions_match_count": sum(1 for c in self.comparisons if c.assertions_match is True),
                "assertions_mismatch_count": sum(1 for c in self.comparisons if c.assertions_match is False),
            },
            "test_pairs": [],
        }

        print(f"Computing extended metrics for {total_tests} test pairs...")

        for i, comparison in enumerate(self.comparisons):
            if (i + 1) % 50 == 0:
                print(f"  Processing test {i + 1}/{total_tests}...")

            pair_data = {
                "source_test": {
                    "path": comparison.source_test_path,
                    "name": comparison.source_test_name,
                    "found": comparison.source_method is not None,
                },
                "target_test": {
                    "path": comparison.target_test_path,
                    "name": comparison.target_test_name,
                    "found": comparison.target_method is not None,
                },
                "metrics": {},
            }

            source_body = ""
            target_body = ""

            # Add source method metrics if found
            if comparison.source_method:
                pair_data["source_test"]["signature"] = comparison.source_method.signature
                pair_data["source_test"]["parameters"] = [
                    {"name": p.name, "type": p.type} for p in comparison.source_method.parameters
                ]
                pair_data["source_test"]["param_count"] = comparison.source_method.param_count
                pair_data["source_test"]["assertion_count"] = comparison.source_method.assertion_count
                # Split body by newlines for better readability
                pair_data["source_test"]["body"] = (
                    comparison.source_method.body.split("\n") if comparison.source_method.body else []
                )
                # For Java, concatenate signature + body for full code
                source_body = (
                    comparison.source_method.signature + "\n" + comparison.source_method.body
                    if comparison.source_method.body
                    else comparison.source_method.signature
                )

            # Add target method metrics if found
            if comparison.target_method:
                pair_data["target_test"]["signature"] = comparison.target_method.signature
                pair_data["target_test"]["parameters"] = [
                    {"name": p.name, "type": p.type} for p in comparison.target_method.parameters
                ]
                pair_data["target_test"]["param_count"] = comparison.target_method.param_count
                pair_data["target_test"]["assertion_count"] = comparison.target_method.assertion_count
                # Split body by newlines for better readability
                pair_data["target_test"]["body"] = (
                    comparison.target_method.body.split("\n") if comparison.target_method.body else []
                )
                # Python body already includes signature
                target_body = comparison.target_method.body or ""

            # Add comparison metrics
            pair_data["metrics"] = {
                "params_match": comparison.params_match,
                "assertions_match": comparison.assertions_match,
            }

            # Add detailed comparison if both methods found
            if comparison.source_method and comparison.target_method:
                pair_data["metrics"]["param_count_source"] = comparison.source_method.param_count
                pair_data["metrics"]["param_count_target"] = comparison.target_method.param_count
                pair_data["metrics"]["assertion_count_source"] = comparison.source_method.assertion_count
                pair_data["metrics"]["assertion_count_target"] = comparison.target_method.assertion_count
                pair_data["metrics"]["param_count_diff"] = (
                    comparison.source_method.param_count - comparison.target_method.param_count
                )
                pair_data["metrics"]["assertion_count_diff"] = (
                    comparison.source_method.assertion_count - comparison.target_method.assertion_count
                )

                # === NEW METRICS ===

                # 1. Line count metrics
                line_count_src = compute_line_count(source_body)
                line_count_tgt = compute_line_count(target_body)
                pair_data["metrics"]["line_count_source"] = line_count_src
                pair_data["metrics"]["line_count_target"] = line_count_tgt
                pair_data["metrics"]["line_count_diff"] = line_count_src - line_count_tgt
                line_counts_source.append(line_count_src)
                line_counts_target.append(line_count_tgt)

                # 2. Assertion type breakdown
                assertion_types_src = extract_assertion_types(source_body, lang=self.source_lang)
                assertion_types_tgt = extract_assertion_types(target_body, lang=self.target_lang)
                pair_data["metrics"]["assertion_types_source"] = assertion_types_src
                pair_data["metrics"]["assertion_types_target"] = assertion_types_tgt
                # Aggregate
                for k, v in assertion_types_src.items():
                    assertion_types_source_agg[k] = assertion_types_source_agg.get(k, 0) + v
                for k, v in assertion_types_tgt.items():
                    assertion_types_target_agg[k] = assertion_types_target_agg.get(k, 0) + v

                # 3. Code similarity score (using embeddings) - only when enabled
                if self.compute_similarity:
                    similarity = compute_similarity_score(source_body, target_body)
                    pair_data["metrics"]["similarity_score"] = similarity
                    if similarity > 0:
                        similarity_scores.append(similarity)

                # 4. Method call count
                method_calls_src = count_method_calls(source_body, lang=self.source_lang)
                method_calls_tgt = count_method_calls(target_body, lang=self.target_lang)
                pair_data["metrics"]["method_call_count_source"] = method_calls_src
                pair_data["metrics"]["method_call_count_target"] = method_calls_tgt
                pair_data["metrics"]["method_call_diff"] = method_calls_src - method_calls_tgt
                method_call_counts_source.append(method_calls_src)
                method_call_counts_target.append(method_calls_tgt)

                # 5. assertEquals value comparison
                assertEquals_comparison = compare_assertEquals_values(
                    source_body, target_body, source_lang=self.source_lang, target_lang=self.target_lang
                )
                pair_data["metrics"]["assertEquals_comparison"] = assertEquals_comparison
                if assertEquals_comparison["comparable_pairs"] > 0:
                    assertEquals_total_comparable += assertEquals_comparison["comparable_pairs"]
                    assertEquals_total_matching += assertEquals_comparison["matching_assertions"]

                # 6. Assertion type mapping (how source assertion types map to target types)
                source_assertions = extract_individual_assertions(source_body, lang=self.source_lang)
                target_assertions = extract_individual_assertions(target_body, lang=self.target_lang)
                assertion_mapping = match_assertions(source_assertions, target_assertions)
                pair_data["metrics"]["assertion_type_mapping"] = assertion_mapping

                # Aggregate into global mapping
                for src_type, target_map in assertion_mapping.items():
                    if src_type not in global_assertion_mapping:
                        global_assertion_mapping[src_type] = {}
                    for tgt_type, count in target_map.items():
                        if tgt_type not in global_assertion_mapping[src_type]:
                            global_assertion_mapping[src_type][tgt_type] = 0
                        global_assertion_mapping[src_type][tgt_type] += count

                # Also aggregate only when assertion count matches (for same-count metrics)
                if comparison.source_method.assertion_count == comparison.target_method.assertion_count:
                    for src_type, target_map in assertion_mapping.items():
                        if src_type not in global_assertion_mapping_same_count:
                            global_assertion_mapping_same_count[src_type] = {}
                        for tgt_type, count in target_map.items():
                            if tgt_type not in global_assertion_mapping_same_count[src_type]:
                                global_assertion_mapping_same_count[src_type][tgt_type] = 0
                            global_assertion_mapping_same_count[src_type][tgt_type] += count
                    for k, v in assertion_types_src.items():
                        assertion_types_source_agg_same_count[k] = assertion_types_source_agg_same_count.get(k, 0) + v

            report_data["test_pairs"].append(pair_data)

        # Add aggregate metrics to summary
        if line_counts_source:
            report_data["summary"]["avg_line_count_source"] = round(
                sum(line_counts_source) / len(line_counts_source), 2
            )
            report_data["summary"]["avg_line_count_target"] = round(
                sum(line_counts_target) / len(line_counts_target), 2
            )

        if similarity_scores:
            report_data["summary"]["avg_similarity_score"] = round(sum(similarity_scores) / len(similarity_scores), 4)
            report_data["summary"]["min_similarity_score"] = round(min(similarity_scores), 4)
            report_data["summary"]["max_similarity_score"] = round(max(similarity_scores), 4)

        if method_call_counts_source:
            report_data["summary"]["avg_method_calls_source"] = round(
                sum(method_call_counts_source) / len(method_call_counts_source), 2
            )
            report_data["summary"]["avg_method_calls_target"] = round(
                sum(method_call_counts_target) / len(method_call_counts_target), 2
            )

        report_data["summary"]["assertion_types_source"] = assertion_types_source_agg
        report_data["summary"]["assertion_types_target"] = assertion_types_target_agg

        report_data["summary"]["assertEquals_summary"] = {
            "total_comparable_pairs": assertEquals_total_comparable,
            "total_matching_assertions": assertEquals_total_matching,
            "overall_match_rate": (
                round(assertEquals_total_matching / assertEquals_total_comparable, 4)
                if assertEquals_total_comparable > 0
                else None
            ),
        }

        # Calculate match percentages for research reporting
        # This includes all the mapping information organized by good/other/unmatched
        match_percentages = calculate_assertion_match_percentages(global_assertion_mapping, assertion_types_source_agg)
        report_data["summary"]["assertion_match_percentages"] = match_percentages

        # Same metrics but only for tests where source and target have same assertion count
        tests_same_assertion_count = sum(
            1
            for c in self.comparisons
            if c.source_method
            and c.target_method
            and c.source_method.assertion_count == c.target_method.assertion_count
        )
        report_data["summary"]["tests_with_same_assertion_count"] = tests_same_assertion_count
        match_percentages_same_count = calculate_assertion_match_percentages(
            global_assertion_mapping_same_count, assertion_types_source_agg_same_count
        )
        report_data["summary"]["assertion_match_percentages_same_assertion_count"] = match_percentages_same_count

        with open(output_file, "w", encoding="utf-8") as f:
            json.dump(report_data, f, indent=2, ensure_ascii=False)

        print(f"JSON report generated: {output_file}")


def parse_superclass_map(map_str: str) -> Dict[str, str]:
    """Parse superclass map from string format: 'Class1:Super1,Class2:Super2'"""
    if not map_str:
        return {}

    result = {}
    for pair in map_str.split(","):
        pair = pair.strip()
        if ":" in pair:
            class_name, superclass_name = pair.split(":", 1)
            result[class_name.strip()] = superclass_name.strip()
    return result


def main():
    """Main entry point"""
    import argparse

    parser = argparse.ArgumentParser(description="Compare tests between source and target languages")
    parser.add_argument("--mapping_csv", type=Path, required=True, help="Path to test name mapping CSV file")
    parser.add_argument(
        "--source_lang",
        type=Path,
        required=True,
        help="Path to source language test directory (e.g., project/java/src/test/java)",
    )
    parser.add_argument(
        "--target_lang",
        type=Path,
        required=True,
        help="Path to target language test directory (e.g., project/python/src/test)",
    )
    parser.add_argument(
        "--superclass_map",
        type=str,
        default="",
        help="Superclass mappings for inherited tests (format: Class1:Super1,Class2:Super2)",
    )
    parser.add_argument(
        "--compute_similarity",
        action="store_true",
        help="Compute embedding similarity between source and target (slow, loads model)",
    )
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=Path("test_comparison_report.json"),
        help="Output JSON report file (default: test_comparison_report.json)",
    )
    parser.add_argument("-t", "--txt", type=Path, default=None, help="Output text report file (optional)")

    args = parser.parse_args()

    # Validate paths exist
    if not args.source_lang.exists():
        print(f"Error: Source path does not exist: {args.source_lang}")
        return 1
    if not args.target_lang.exists():
        print(f"Error: Target path does not exist: {args.target_lang}")
        return 1

    # Parse superclass mappings
    superclass_map = parse_superclass_map(args.superclass_map)

    comparator = TestComparator(
        args.mapping_csv, args.source_lang, args.target_lang, superclass_map, compute_similarity=args.compute_similarity
    )
    comparator.compare_all_tests()

    # Generate JSON report (default)
    comparator.generate_json_report(args.output)

    # Generate text report if requested
    if args.txt:
        comparator.generate_report(args.txt)


if __name__ == "__main__":
    main()
