#!/usr/bin/env python3
"""
Compute line coverage for a Crust project using cargo-tarpaulin.

Runs tarpaulin per test binary (translated-only, then translated+generated),
parses JSON reports, merges covered lines across runs, and prints coverage %.

Usage:
  python evaluate_crust_test_coverage.py <project_name> --agent-dir DIR [--timeout N]
  python evaluate_crust_test_coverage.py 2dpartint --agent-dir /path/to/agent-gen-test

Output:
  - Coverage % for "translated" tests (non-generated bins only)
  - Coverage % for "translated+generated" tests (all bins)
"""

import argparse
import json
import subprocess
import tempfile
from pathlib import Path


def find_test_files(bin_dir: Path, include_generated: bool = False) -> list[str]:
    """Return .rs filenames in bin dir. include_generated=False => exclude *_generated.rs."""
    if not bin_dir.exists():
        return []
    if include_generated:
        return [f.name for f in bin_dir.glob("*.rs") if f.name.endswith("_generated.rs")]
    return [f.name for f in bin_dir.glob("*.rs") if not f.name.endswith("_generated.rs")]


def _path_key(path_val) -> str:
    """Normalize path from tarpaulin JSON (list or string) to a single string key."""
    if isinstance(path_val, list):
        return "/".join(p for p in path_val if p).lstrip("/") or "/"
    return str(path_val)


def _is_test_file(path_key: str) -> bool:
    """Return True if the path is a test file (in src/bin/)."""
    # Check if path contains /src/bin/ (test binaries are in src/bin/)
    return "/src/bin/" in path_key


def _covered_lines_from_file(file_obj: dict) -> set[int]:
    """Extract set of covered line numbers from a file entry (traces with Line > 0)."""
    out = set()
    for tr in file_obj.get("traces", []):
        if tr.get("stats", {}).get("Line", 0) > 0:
            out.add(tr["line"])
    return out


def _coverable_from_file(file_obj: dict) -> int:
    """Return coverable line count for this file."""
    return file_obj.get("coverable", 0)


def load_tarpaulin_json(path: Path, exclude_test_files: bool = True) -> tuple[dict[str, set[int]], dict[str, int]]:
    """
    Load a tarpaulin-report.json. Return (covered_per_file, coverable_per_file).
    covered_per_file: path_key -> set of covered line numbers
    coverable_per_file: path_key -> coverable line count

    If exclude_test_files is True, filters out files in src/bin/.
    """
    with open(path, encoding="utf-8") as f:
        d = json.load(f)
    covered = {}
    coverable = {}
    for fi in d.get("files", []):
        key = _path_key(fi.get("path", ""))
        if exclude_test_files and _is_test_file(key):
            continue
        covered[key] = _covered_lines_from_file(fi)
        coverable[key] = _coverable_from_file(fi)
    return covered, coverable


def merge_reports(report_paths: list[Path], exclude_test_files: bool = True) -> tuple[int, int, dict[str, int]]:
    """
    Merge multiple tarpaulin JSON reports: union of covered lines per file,
    max coverable per file. Return (total_covered, total_coverable, coverable_per_file).

    If exclude_test_files is True, filters out files in src/bin/.
    Returns coverable_per_file dict for fair comparison (same denominator).
    """
    merged_covered: dict[str, set[int]] = {}
    merged_coverable: dict[str, int] = {}

    for path in report_paths:
        if not path.exists():
            continue
        try:
            cov, covable = load_tarpaulin_json(path, exclude_test_files=exclude_test_files)
        except (json.JSONDecodeError, OSError):
            continue
        for key, lines in cov.items():
            merged_covered.setdefault(key, set()).update(lines)
        for key, count in covable.items():
            merged_coverable[key] = max(merged_coverable.get(key, 0), count)

    total_covered = sum(len(s) for s in merged_covered.values())
    total_coverable = sum(merged_coverable.values())
    return total_covered, total_coverable, merged_coverable


def run_tarpaulin_bin(project_dir: Path, bin_name: str, out_dir: Path, timeout: int = 300) -> Path | None:
    """
    Run cargo tarpaulin for one binary. Writes JSON to out_dir / f"{bin_name}.json".
    Returns path to JSON file, or None if failed.
    """
    out_dir.mkdir(parents=True, exist_ok=True)
    out_file = out_dir / "tarpaulin-report.json"

    try:
        subprocess.run(
            [
                "cargo",
                "tarpaulin",
                "--bin",
                bin_name,
                "--no-fail-fast",
                "-o",
                "Json",
                "--output-dir",
                str(out_dir),
                "--",
                "--test-threads=1",
            ],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        return None
    except Exception:
        return None

    return out_file if out_file.exists() else None


def run_tarpaulin_for_bins(
    project_dir: Path,
    bin_names: list[str],
    base_out_dir: Path,
) -> list[Path]:
    """Run tarpaulin for each binary; each gets its own subdir. Return list of JSON paths."""
    json_paths = []
    for bin_name in bin_names:
        out_dir = base_out_dir / bin_name
        path = run_tarpaulin_bin(project_dir, bin_name, out_dir)
        if path is not None:
            json_paths.append(path)
    return json_paths


def coverage_percentage(covered: int, coverable: int) -> float:
    if coverable <= 0:
        return 0.0
    return 100.0 * covered / coverable


def main() -> int:
    parser = argparse.ArgumentParser(description="Compute coverage for translated and translated+generated tests")
    parser.add_argument("project_name", help="Project name (e.g., 2dpartint)")
    parser.add_argument(
        "--agent-dir",
        type=Path,
        required=True,
        help="Agent translations directory (e.g. crust_translations/agent-gen-test)",
    )
    parser.add_argument("--timeout", type=int, default=300, help="Timeout per tarpaulin run (seconds)")
    args = parser.parse_args()

    project_name = args.project_name.lower()
    agent_dir = args.agent_dir.resolve()
    project_dir = agent_dir / project_name / "rust"
    bin_dir = project_dir / "src" / "bin"

    if not project_dir.exists():
        print(f"Error: Project directory not found: {project_dir}")
        return 1

    translated = find_test_files(bin_dir, include_generated=False)
    generated = find_test_files(bin_dir, include_generated=True)
    all_bins = [f[:-3] for f in translated + generated]  # strip .rs
    translated_bins = [f[:-3] for f in translated]

    if not all_bins:
        print("No test binaries found.")
        return 0

    with tempfile.TemporaryDirectory(prefix="tarpaulin_cov_") as tmp:
        base_out = Path(tmp)

        # 1) Translated only
        print("Running tarpaulin for translated tests (non-generated bins)...")
        translated_jsons = run_tarpaulin_for_bins(project_dir, translated_bins, base_out / "translated")
        cov_t, _, coverable_dict_t = merge_reports(translated_jsons, exclude_test_files=True)

        # 2) Translated + generated (all bins)
        print("Running tarpaulin for translated+generated tests (all bins)...")
        all_jsons = run_tarpaulin_for_bins(project_dir, all_bins, base_out / "all")
        cov_all, _, coverable_dict_all = merge_reports(all_jsons, exclude_test_files=True)

        # Use the same coverable lines (library code only) for fair comparison
        # Take the union of coverable files from both runs (max per file)
        fair_coverable: dict[str, int] = {}
        for key, count in coverable_dict_t.items():
            fair_coverable[key] = max(fair_coverable.get(key, 0), count)
        for key, count in coverable_dict_all.items():
            fair_coverable[key] = max(fair_coverable.get(key, 0), count)

        total_coverable = sum(fair_coverable.values())

        pct_t = coverage_percentage(cov_t, total_coverable)
        pct_all = coverage_percentage(cov_all, total_coverable)

        print(f"  Translated only: {cov_t}/{total_coverable} lines -> {pct_t:.2f}%")
        print(f"  Translated+generated: {cov_all}/{total_coverable} lines -> {pct_all:.2f}%")

    print(f"\nCoverage (library code only, excluding test files):")
    print(f"    translated={pct_t:.2f}% | translated+generated={pct_all:.2f}%")
    print(f"==>{pct_t:.2f},{pct_all:.2f}")
    return 0


if __name__ == "__main__":
    exit(main())
