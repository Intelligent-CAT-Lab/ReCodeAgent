#!/usr/bin/env python3
"""
Copy Rust test files to translation directories and run tests.

Usage: python evaluate_crust_test_original.py <project_name> --source-dir DIR --agent-dir DIR --tool-dir DIR [--skip-copy]
Example: python evaluate_crust_test_original.py 2dpartint --source-dir /path/to/crust --agent-dir /path/to/agent-original-test --tool-dir /path/to/tool
"""

import argparse
import os
import shutil
import subprocess
import re
from pathlib import Path

# Explicit mapping from standard names to tool directory names
STANDARD_TO_TOOL = {
    "2dpartint": "proj_2DPartInt",
    "42-kocaeli-printf": "proj_42_Kocaeli_Printf",
    "bostree": "Bostree",
    "genetic-neural-network-for-simple-control": "Genetic_neural_network_for_simple_control",
    "graph-recogniser": "Graph_recogniser",
    "holdem-odds": "Holdem_Odds",
    "linear-algebra-c": "Linear_Algebra_C",
    "math-library-in-c": "Math_Library_in_C",
    "simple-config": "Simple_Config",
    "simple-sparsehash": "Simple_Sparsehash",
    "aes128-simd": "aes128_SIMD",
    "btree-map": "btree_map",
    "c-aces": "c_aces",
    "c-blind-rsa-signatures": "c_blind_rsa_signatures",
    "c-string": "c_string",
    "gorilla-paper-encode": "gorilla_paper_encode",
    "lambda-calculus-eval": "lambda_calculus_eval",
    "merkle-tree-c": "merkle_tree_c",
    "rbtree-lab": "rbtree_lab",
    "roaring-bitmap": "roaring_bitmap",
}


def find_tool_dir_name(standard_name: str, tool_dir: Path) -> str | None:
    """
    Find the tool directory name for a given standard project name.
    First checks explicit mapping, then does case-insensitive matching.
    """
    # Check explicit mapping first
    if standard_name in STANDARD_TO_TOOL:
        return STANDARD_TO_TOOL[standard_name]

    # For unmapped projects, find by case-insensitive match
    # Convert standard name: replace hyphens with underscores for comparison
    normalized = standard_name.replace("-", "_").lower()

    # List all tool directories
    tool_dirs = [d.name for d in tool_dir.iterdir() if d.is_dir()]

    # Try exact match first (case-insensitive)
    for tool_dir in tool_dirs:
        if tool_dir.lower() == standard_name.lower():
            return tool_dir
        if tool_dir.lower().replace("-", "_") == normalized:
            return tool_dir

    # If no match found, return None
    return None


def copy_test_files(
    standard_name: str,
    tool_name: str,
    source_dir: Path,
    agent_dir: Path,
    tool_dir: Path,
) -> dict:
    """
    Copy .rs test files from source to agent and tool directories.
    Excludes *_generated.rs files.
    Returns dict with copied file info.
    """
    source_bin = source_dir / standard_name / "rust" / "src" / "bin"
    agent_bin = agent_dir / standard_name / "rust" / "src" / "bin"
    tool_bin = tool_dir / tool_name / "src" / "bin"

    copied = {"agent": [], "tool": []}

    if not source_bin.exists():
        print(f"  Warning: Source bin directory not found: {source_bin}")
        return copied

    # Get all .rs files from source, excluding *_generated.rs files
    rs_files = [f for f in source_bin.glob("*.rs") if not f.name.endswith("_generated.rs")]

    if not rs_files:
        print(f"  Warning: No .rs files found in {source_bin}")
        return copied

    # Copy to agent directory (if it exists)
    if agent_bin.parent.exists():  # Check if src/ exists
        agent_bin.mkdir(parents=True, exist_ok=True)
        for rs_file in rs_files:
            dest = agent_bin / rs_file.name
            shutil.copy2(rs_file, dest)
            copied["agent"].append(rs_file.name)
    else:
        print(f"  Warning: Agent directory not found: {agent_bin.parent}")

    # Copy to tool directory
    if tool_bin.parent.exists():  # Check if src/ exists
        tool_bin.mkdir(parents=True, exist_ok=True)
        for rs_file in rs_files:
            dest = tool_bin / rs_file.name
            shutil.copy2(rs_file, dest)
            copied["tool"].append(rs_file.name)
    else:
        print(f"  Warning: Tool directory not found: {tool_bin.parent}")

    return copied


def run_cargo_check(project_dir: Path, bin_names: list = None) -> tuple[bool, str]:
    """
    Check compilability using `cargo test` in compile-only mode.

    We intentionally avoid `cargo check` because it also compiles the normal
    `main` in `src/bin` files, which can fail even when the tests themselves
    would build and run fine. Using `cargo test --no-run` builds the test
    harnesses without compiling the regular binary `main`.

    If bin_names is provided, only checks those specific binaries
    (via `cargo test --bin <name> --no-run`). Otherwise uses
    `cargo test --all-targets --no-run`.

    Returns (success, error_message).
    """
    try:
        if bin_names:
            # Check each binary's tests individually
            all_errors = []
            for bin_name in bin_names:
                # Remove .rs extension if present
                name = bin_name[:-3] if bin_name.endswith(".rs") else bin_name
                result = subprocess.run(
                    ["cargo", "test", "--bin", name, "--no-run"],
                    cwd=project_dir,
                    capture_output=True,
                    text=True,
                    timeout=300,
                )
                if result.returncode != 0:
                    all_errors.append(f"{bin_name}: {result.stderr}")

            if all_errors:
                return False, "\n".join(all_errors)
            return True, ""
        else:
            result = subprocess.run(
                ["cargo", "test", "--all-targets", "--no-run"],
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


def parse_test_output(stdout: str, stderr: str) -> dict:
    """
    Parse cargo test output to extract test results per binary.
    Note: "Running unittests" lines go to stderr, "test result" lines go to stdout.
    We match them by order of appearance, tracking ALL test sources to maintain alignment.
    Returns dict: {binary_name: {"total": N, "passed": N, "failed": N}}
    Only includes src/bin binaries that have at least 1 test.
    """
    results = {}

    # Pattern to match test result lines from stdout
    result_pattern = re.compile(r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed;")

    # Pattern to match ALL "Running unittests" lines from stderr to maintain ordering
    all_binary_pattern = re.compile(r"Running unittests\s+(\S+\.rs)")
    # Also match Doc-tests which produce test results
    doc_tests_pattern = re.compile(r"Doc-tests\s+")

    # Extract ALL test sources from stderr (in order) to match with results
    all_sources = []
    for line in stderr.split("\n"):
        binary_match = all_binary_pattern.search(line)
        if binary_match:
            binary_path = binary_match.group(1)
            all_sources.append(binary_path)
        elif doc_tests_pattern.search(line):
            all_sources.append("doc-tests")

    # Extract test results from stdout (in order)
    test_results = []
    for line in stdout.split("\n"):
        result_match = result_pattern.search(line)
        if result_match:
            passed = int(result_match.group(1))
            failed = int(result_match.group(2))
            test_results.append({"passed": passed, "failed": failed})

    # Match sources with results by position
    # Only include src/bin files in final results
    for i, source_path in enumerate(all_sources):
        if i < len(test_results) and source_path.startswith("src/bin/"):
            result = test_results[i]
            total = result["passed"] + result["failed"]
            if total > 0:
                binary_name = source_path.split("/")[-1]
                results[binary_name] = {"total": total, "passed": result["passed"], "failed": result["failed"]}

    return results


def run_cargo_test_for_bin(project_dir: Path, bin_name: str) -> tuple[dict, str]:
    """
    Run cargo test for a specific binary.
    bin_name should be without .rs extension.
    Returns (results_dict, raw_output).
    """
    try:
        result = subprocess.run(
            ["cargo", "test", "--bin", bin_name, "--", "--test-threads=1"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=300,
        )
        combined_output = result.stdout + result.stderr

        # Parse test results from output
        result_pattern = re.compile(r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed;")

        for line in result.stdout.split("\n"):
            match = result_pattern.search(line)
            if match:
                passed = int(match.group(1))
                failed = int(match.group(2))
                return {"total": passed + failed, "passed": passed, "failed": failed}, combined_output

        # No test result found
        return {"total": 0, "passed": 0, "failed": 0}, combined_output
    except subprocess.TimeoutExpired:
        return {"total": 0, "passed": 0, "failed": 0, "error": "Timeout"}, "Timeout expired"
    except Exception as e:
        return {"total": 0, "passed": 0, "failed": 0, "error": str(e)}, str(e)


def run_tests_for_copied_files(project_dir: Path, copied_files: list) -> dict:
    """
    Run cargo test for each copied test file individually.
    Returns dict: {filename: {"total": N, "passed": N, "failed": N}}
    """
    results = {}

    for filename in copied_files:
        # Remove .rs extension to get binary name
        bin_name = filename[:-3] if filename.endswith(".rs") else filename
        print(f"    Testing {filename}...")

        test_result, _ = run_cargo_test_for_bin(project_dir, bin_name)
        results[filename] = test_result

    return results


def calculate_totals(tests: dict) -> tuple[int, int, int]:
    """Calculate total executed, passed, and failed from tests dict."""
    total_executed = 0
    total_passed = 0
    total_failed = 0

    for result in tests.values():
        total_executed += result.get("total", 0)
        total_passed += result.get("passed", 0)
        total_failed += result.get("failed", 0)

    return total_executed, total_passed, total_failed


def all_tests_passed(tests: dict) -> bool:
    """Check if all tests passed (at least one test and no failures)."""
    if not tests:
        return False
    total_executed, total_passed, total_failed = calculate_totals(tests)
    return total_executed > 0 and total_failed == 0


def print_summary(
    project_name: str,
    tool_name: str,
    agent_compilable: bool,
    agent_tests: dict,
    tool_compilable: bool,
    tool_tests: dict,
    copied_files: dict,
):
    """Print the summary of the test run."""

    print(f"\n{'='*60}")
    print(f"=== Project: {project_name} ===")
    print(f"{'='*60}")

    # Agent summary
    print(f"\nAGENT ({project_name}):")
    print(f"  Compilable: {'Yes' if agent_compilable else 'No'}")
    print(f"  Copied files: {', '.join(copied_files['agent']) if copied_files['agent'] else 'None'}")
    if agent_tests:
        print("  Test Results:")
        for test_file, counts in agent_tests.items():
            print(f"    - {test_file}: {counts['total']} tests ({counts['passed']} passed, {counts['failed']} failed)")
    else:
        print("  Test Results: No tests found or tests did not run")

    # Tool summary
    print(f"\nTOOL ({tool_name}):")
    print(f"  Compilable: {'Yes' if tool_compilable else 'No'}")
    print(f"  Copied files: {', '.join(copied_files['tool']) if copied_files['tool'] else 'None'}")
    if tool_tests:
        print("  Test Results:")
        for test_file, counts in tool_tests.items():
            print(f"    - {test_file}: {counts['total']} tests ({counts['passed']} passed, {counts['failed']} failed)")
    else:
        print("  Test Results: No tests found or tests did not run")

    print(f"\n{'='*60}")

    # Calculate totals and test results
    tool_executed, tool_passed, tool_failed = calculate_totals(tool_tests)
    agent_executed, agent_passed, agent_failed = calculate_totals(agent_tests)

    tool_test_passed = 1 if all_tests_passed(tool_tests) else 0
    agent_test_passed = 1 if all_tests_passed(agent_tests) else 0

    # Print CSV-like summary line
    print(
        f"\n==> {project_name},{1 if tool_compilable else 0},{tool_test_passed},{1 if agent_compilable else 0},{agent_test_passed},{tool_executed},{tool_passed},{tool_failed},{agent_executed},{agent_passed},{agent_failed}"
    )


def main():
    parser = argparse.ArgumentParser(description="Copy Rust test files and run tests for a project")
    parser.add_argument("project_name", help="Standardized project name (e.g., 2dpartint, btree-map)")
    parser.add_argument(
        "--source-dir", type=Path, required=True, help="Source projects directory (e.g. data/tool_projects/crust)"
    )
    parser.add_argument(
        "--agent-dir",
        type=Path,
        required=True,
        help="Agent translations directory (e.g. crust_translations/agent-original-test)",
    )
    parser.add_argument(
        "--tool-dir", type=Path, required=True, help="Tool translations directory (e.g. crust_translations/tool)"
    )
    parser.add_argument("--skip-copy", action="store_true", help="Skip copying files, just run tests")
    args = parser.parse_args()

    project_name = args.project_name.lower()
    source_dir = args.source_dir.resolve()
    agent_dir = args.agent_dir.resolve()
    tool_dir = args.tool_dir.resolve()

    # Validate project exists in source
    source_project = source_dir / project_name
    if not source_project.exists():
        print(f"Error: Project '{project_name}' not found in source directory: {source_dir}")
        return 1

    # Find tool directory name
    tool_name = find_tool_dir_name(project_name, tool_dir)
    if not tool_name:
        print(f"Error: Could not find tool directory for project '{project_name}'")
        return 1

    print(f"Project: {project_name}")
    print(f"Tool directory: {tool_name}")

    # Copy test files
    if not args.skip_copy:
        print("\nCopying test files...")
        copied_files = copy_test_files(project_name, tool_name, source_dir, agent_dir, tool_dir)
        print(f"  Agent: {len(copied_files['agent'])} files copied")
        print(f"  Tool: {len(copied_files['tool'])} files copied")
    else:
        copied_files = {"agent": [], "tool": []}

    # Determine project directories for cargo commands
    agent_project_dir = agent_dir / project_name / "rust"
    tool_project_dir = tool_dir / tool_name

    # Run cargo check for agent (only for copied files)
    agent_compilable = False
    print("\nRunning cargo check for agent...")
    if copied_files["agent"]:
        agent_compilable, agent_check_err = run_cargo_check(agent_project_dir, copied_files["agent"])
    else:
        agent_compilable, agent_check_err = False, "No files copied"
    print(f"  Result: {'Success' if agent_compilable else 'Failed'}")
    if not agent_compilable and agent_check_err:
        print(f"  Error: {agent_check_err[:500]}...")

    # Run cargo check for tool (only for copied files)
    print("\nRunning cargo check for tool...")
    if copied_files["tool"]:
        tool_compilable, tool_check_err = run_cargo_check(tool_project_dir, copied_files["tool"])
    else:
        tool_compilable, tool_check_err = False, "No files copied"
    print(f"  Result: {'Success' if tool_compilable else 'Failed'}")
    if not tool_compilable and tool_check_err:
        print(f"  Error: {tool_check_err[:500]}...")

    # Run cargo test for agent (only if compilable and files were copied)
    agent_tests = {}
    if agent_compilable and copied_files["agent"]:
        print("\nRunning cargo test for agent (copied files only)...")
        agent_tests = run_tests_for_copied_files(agent_project_dir, copied_files["agent"])

    # Run cargo test for tool (only if compilable and files were copied)
    tool_tests = {}
    if tool_compilable and copied_files["tool"]:
        print("\nRunning cargo test for tool (copied files only)...")
        tool_tests = run_tests_for_copied_files(tool_project_dir, copied_files["tool"])

    # Print summary
    print_summary(project_name, tool_name, agent_compilable, agent_tests, tool_compilable, tool_tests, copied_files)

    return 0


if __name__ == "__main__":
    exit(main())
