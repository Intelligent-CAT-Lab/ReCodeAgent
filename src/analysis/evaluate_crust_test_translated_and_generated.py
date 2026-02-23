#!/usr/bin/env python3
"""
Run test files for an agent project.

Usage: python evaluate_crust_test_translated_and_generated.py <project_name> --agent-dir DIR
Example: python evaluate_crust_test_translated_and_generated.py 2dpartint --agent-dir /path/to/agent-gen-test
"""

import argparse
import subprocess
import re
from pathlib import Path


def find_test_files(bin_dir: Path, include_generated: bool = False) -> list:
    """
    Find .rs files in bin directory.
    If include_generated is False, excludes *_generated.rs files.
    If include_generated is True, returns ONLY *_generated.rs files.
    Returns list of filenames.
    """
    if not bin_dir.exists():
        return []

    if include_generated:
        return [f.name for f in bin_dir.glob("*.rs") if f.name.endswith("_generated.rs")]
    else:
        return [f.name for f in bin_dir.glob("*.rs") if not f.name.endswith("_generated.rs")]


def run_cargo_check_bin(project_dir: Path, bin_name: str) -> tuple[bool, str]:
    """
    Check compilability for a specific binary using `cargo test --no-run`.

    We intentionally avoid `cargo check` because it also compiles the normal
    `main` in `src/bin` files, which can fail even when the tests themselves
    would build and run fine. Using `cargo test --bin <name> --no-run` builds
    the test harness without compiling the regular binary `main`.

    Returns (success, error_message).
    """
    try:
        result = subprocess.run(
            ["cargo", "test", "--bin", bin_name, "--no-run"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=300,
        )
        if result.returncode == 0:
            return True, ""
        else:
            return False, result.stderr
    except subprocess.TimeoutExpired:
        return False, "Timeout expired"
    except Exception as e:
        return False, str(e)


def run_cargo_test_bin(project_dir: Path, bin_name: str) -> dict:
    """
    Run cargo test for a specific binary.
    Returns dict with test results.
    """
    try:
        result = subprocess.run(
            ["cargo", "test", "--bin", bin_name, "--", "--test-threads=1"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=300,
        )

        # Parse test results
        result_pattern = re.compile(r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed;")

        for line in result.stdout.split("\n"):
            match = result_pattern.search(line)
            if match:
                passed = int(match.group(1))
                failed = int(match.group(2))
                return {"total": passed + failed, "passed": passed, "failed": failed, "compilable": True}

        return {"total": 0, "passed": 0, "failed": 0, "compilable": True}
    except subprocess.TimeoutExpired:
        return {"total": 0, "passed": 0, "failed": 0, "compilable": True, "error": "Timeout"}
    except Exception as e:
        return {"total": 0, "passed": 0, "failed": 0, "compilable": True, "error": str(e)}


def run_cargo_run_bin(project_dir: Path, bin_name: str) -> dict:
    """
    Run `cargo run --bin <bin_name>` and treat it as a single aggregate test.

    This is used for generated binaries that define a `main` function which
    invokes internal test functions but do not use the `#[test]` attribute,
    so `cargo test` would otherwise report 0 tests.
    """
    try:
        result = subprocess.run(
            ["cargo", "run", "--bin", bin_name],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=300,
        )
        success = result.returncode == 0
        combined_output = result.stdout + result.stderr

        return {
            "total": 1,
            "passed": 1 if success else 0,
            "failed": 0 if success else 1,
            # Treat the binary as compilable; a non-zero exit here generally
            # means a test assertion failed rather than a compilation error.
            "compilable": True,
            "output": combined_output[-500:],
        }
    except subprocess.TimeoutExpired:
        return {"total": 0, "passed": 0, "failed": 0, "compilable": False, "error": "Timeout"}
    except Exception as e:
        return {"total": 0, "passed": 0, "failed": 0, "compilable": False, "error": str(e)}


def file_has_rust_tests(bin_dir: Path, filename: str) -> bool:
    """
    Return True if the given Rust source file contains any `#[test]` attributes.
    """
    path = bin_dir / filename
    try:
        text = path.read_text(encoding="utf-8")
    except Exception:
        return False
    return "#[test]" in text


def run_tests_for_files(project_dir: Path, test_files: list) -> dict:
    """
    Run tests for a list of test files.
    Returns dict of results per file.
    """
    results = {}
    for filename in test_files:
        bin_name = filename[:-3]  # Remove .rs
        print(f"    Checking {filename}...", end=" ")

        # First check if it compiles
        compilable, error = run_cargo_check_bin(project_dir, bin_name)

        if not compilable:
            print("COMPILE ERROR")
            results[filename] = {
                "total": 0,
                "passed": 0,
                "failed": 0,
                "compilable": False,
                "error": error[:200] if error else "Unknown error",
            }
            continue

        # Run tests
        print("OK, running tests...", end=" ")
        test_result = run_cargo_test_bin(project_dir, bin_name)
        results[filename] = test_result

        if test_result["total"] > 0:
            print(f"{test_result['passed']}/{test_result['total']} passed")
        else:
            print("0 tests")

    return results


def run_generated_tests_for_files(project_dir: Path, bin_dir: Path, test_files: list) -> dict:
    """
    Run tests for generated test files.

    - If a file contains `#[test]`, we use the normal `cargo test` path.
    - If it does NOT contain `#[test]` (e.g., `csv_test_generated.rs` which
      defines test-like functions and a `main` that runs them), we execute
      it via `cargo run --bin <name>` so those tests are actually exercised,
      without modifying the Rust test files themselves.
    """
    results = {}
    for filename in test_files:
        bin_name = filename[:-3]  # Remove .rs
        print(f"    Checking {filename}...", end=" ")

        if file_has_rust_tests(bin_dir, filename):
            # Same pipeline as normal tests: compile via `cargo test --no-run`
            # and then run `cargo test`.
            compilable, error = run_cargo_check_bin(project_dir, bin_name)

            if not compilable:
                print("COMPILE ERROR")
                results[filename] = {
                    "total": 0,
                    "passed": 0,
                    "failed": 0,
                    "compilable": False,
                    "error": error[:200] if error else "Unknown error",
                }
                continue

            print("OK, running tests...", end=" ")
            test_result = run_cargo_test_bin(project_dir, bin_name)
            results[filename] = test_result

            if test_result["total"] > 0:
                print(f"{test_result['passed']}/{test_result['total']} passed")
            else:
                print("0 tests")
        else:
            # No #[test] attributes: treat the binary's `main` as the test
            # harness and run it via `cargo run`.
            print("no #[test] attributes, running via `cargo run`...", end=" ")
            test_result = run_cargo_run_bin(project_dir, bin_name)
            results[filename] = test_result

            if test_result["total"] > 0:
                print(f"{test_result['passed']}/{test_result['total']} passed")
            else:
                print("0 tests")

    return results


def run_tests_for_project(project_dir: Path, bin_dir: Path, include_generated: bool = False) -> dict:
    """
    Run tests for test files in a project.
    If include_generated is False, runs non-generated tests.
    If include_generated is True, runs only generated tests.
    Returns dict of results per file.
    """
    test_files = find_test_files(bin_dir, include_generated=include_generated)

    if not test_files:
        file_type = "generated" if include_generated else "non-generated"
        print(f"  No {file_type} test files found")
        return {}

    file_type = "generated" if include_generated else "non-generated"
    print(f"  Found {len(test_files)} {file_type} test file(s): {', '.join(test_files)}")

    if include_generated:
        # For generated test files, we may need to execute binaries that do not
        # use the `#[test]` attribute (e.g. csv_test_generated.rs) via
        # `cargo run` so that their internal tests are exercised.
        return run_generated_tests_for_files(project_dir, bin_dir, test_files)
    else:
        return run_tests_for_files(project_dir, test_files)


def print_results_section(results: dict, indent: str = "  "):
    """Print results for a set of test files."""
    if not results:
        print(f"{indent}No test files found")
        return

    for test_file, result in results.items():
        if not result.get("compilable", True):
            print(f"{indent}- {test_file}: COMPILE ERROR")
        elif result["total"] > 0:
            print(
                f"{indent}- {test_file}: {result['total']} tests ({result['passed']} passed, {result['failed']} failed)"
            )
        else:
            print(f"{indent}- {test_file}: 0 tests")


def calculate_totals(results: dict) -> tuple[int, int, int]:
    """Calculate total executed, passed, and failed from results dict."""
    total_executed = 0
    total_passed = 0
    total_failed = 0

    for result in results.values():
        if result.get("compilable", True):
            total_executed += result.get("total", 0)
            total_passed += result.get("passed", 0)
            total_failed += result.get("failed", 0)

    return total_executed, total_passed, total_failed


def print_summary(project_name: str, results: dict, generated_results: dict):
    """Print the summary of the test run."""

    print(f"\n{'='*60}")
    print(f"=== Project: {project_name} ===")
    print(f"{'='*60}")

    if results or generated_results:
        if results:
            print("\nNon-generated tests:")
            print_results_section(results, indent="  ")

        if generated_results:
            print("\nGenerated tests:")
            print_results_section(generated_results, indent="  ")
    else:
        print("\nNo test files found")

    print(f"\n{'='*60}")

    # Calculate totals
    ng_executed, ng_passed, ng_failed = calculate_totals(results)
    gen_executed, gen_passed, gen_failed = calculate_totals(generated_results)

    # Print CSV-like summary line
    print(f"\n==> {project_name},{ng_executed},{ng_passed},{ng_failed},{gen_executed},{gen_passed},{gen_failed}")


def main():
    parser = argparse.ArgumentParser(description="Run test files for an agent project")
    parser.add_argument("project_name", help="Project name (e.g., 2dpartint, btree-map)")
    parser.add_argument(
        "--agent-dir",
        type=Path,
        required=True,
        help="Agent translations directory (e.g. crust_translations/agent-gen-test)",
    )
    args = parser.parse_args()

    project_name = args.project_name.lower()
    agent_dir = args.agent_dir.resolve()

    # Define paths
    agent_project_dir = agent_dir / project_name / "rust"
    agent_bin_dir = agent_project_dir / "src" / "bin"

    # Check if project exists
    if not agent_project_dir.exists():
        print(f"Error: Project directory not found: {agent_project_dir}")
        return 1

    print(f"Project: {project_name}")

    # Run non-generated tests
    print(f"\n--- Non-generated tests ---")
    results = run_tests_for_project(agent_project_dir, agent_bin_dir, include_generated=False)

    # Run generated tests
    print(f"\n--- Generated tests ---")
    generated_results = run_tests_for_project(agent_project_dir, agent_bin_dir, include_generated=True)

    # Print summary
    print_summary(project_name, results, generated_results)

    return 0


if __name__ == "__main__":
    exit(main())
