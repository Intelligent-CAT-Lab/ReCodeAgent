#!/usr/bin/env python3
"""
Compute cost metrics (USD, input tokens, output tokens, time, turns) from
trajectory project_details.json files and plot a grouped bar chart by agent.
Saves the plot as cost.pdf.

Trajectory dirs are expected as ``{COST_TRAJECTORY_AGENT}.{tool}.{project}``
under results/trajectories (e.g. ``recodeagent.oxidizer.myproj``). Change
``COST_TRAJECTORY_AGENT`` to analyze a different top-level agent; adjust
``RECODEAGENT_TOOLS`` if that agent uses different tool name segments.
"""

import json
import os
from pathlib import Path
from collections import defaultdict

import matplotlib.pyplot as plt
import matplotlib
from matplotlib import ticker

matplotlib.use("Agg")

REPO_ROOT = Path(__file__).resolve().parents[2]
TRAJECTORIES_DIR = REPO_ROOT / "results" / "trajectories"
# First segment of trajectory directory names under TRAJECTORIES_DIR.
COST_TRAJECTORY_AGENT = "recodeagent"
COST_ITEMS = ["usdCost", "input_tokens", "output_tokens", "time_seconds", "num_turns"]
COST_LABELS = ["Cost ($)", "Input Tokens", "Output Tokens", "Time (s)", "# Turns"]
# Left y-axis: token counts; right y-axis: USD, time, turns
TOKEN_ITEMS = ["input_tokens", "output_tokens"]
TOKEN_LABELS = ["Input Tokens", "Output Tokens"]
OTHER_ITEMS = ["usdCost", "time_seconds", "num_turns"]
OTHER_LABELS = ["Cost ($)", "Time (min)", "# Turns"]
OUTPUT_PDF = REPO_ROOT / "cost.pdf"

# Consistent font styling (similar spirit to ablation.py)
FONT_SIZE = 12
plt.rcParams.update({"font.size": FONT_SIZE})

# Recodeagent tools (second segment in trajectory dir name); order for x-axis
# Order on x-axis: Oxidizer, AlphaTrans, Skel, CRUST
RECODEAGENT_TOOLS = ["oxidizer", "alphatrans", "skel", "crust"]
TOOL_DISPLAY_LABELS = {
    "skel": "Skel",
    "oxidizer": "Oxidizer",
    "alphatrans": "AlphaTrans",
    "crust": "CRUST",
}


def get_last_json(block):
    """Get last_json from a result block (may be under agent_output)."""
    if not block:
        return None
    return block.get("last_json") or (block.get("agent_output") or {}).get("last_json")


def sum_model_usage(model_usage):
    """Sum inputTokens, outputTokens, costUSD across all models in modelUsage."""
    if not model_usage or not isinstance(model_usage, dict):
        return 0.0, 0, 0
    total_in, total_out, total_usd = 0, 0, 0.0
    for usage in model_usage.values():
        if not isinstance(usage, dict):
            continue
        total_in += usage.get("inputTokens") or 0
        total_out += usage.get("outputTokens") or 0
        total_usd += float(usage.get("costUSD") or 0)
    return total_usd, total_in, total_out


def extract_from_block(block, execution_time_key="execution_time_seconds"):
    """Extract (usd, input_tokens, output_tokens, time, turns) from analyzer/planning/baseagent block."""
    last = get_last_json(block)
    time_val = float(block.get(execution_time_key) or 0) if block else 0
    turns = 0
    usd, inp, out = 0.0, 0, 0
    if last:
        turns = int(last.get("num_turns") or 0)
        usd, inp, out = sum_model_usage(last.get("modelUsage"))
    return usd, inp, out, time_val, turns


def extract_recodeagent_costs(data):
    """Total cost from analyzer_results, planning_results, translator_results (incl. validator per iteration)."""
    usd, inp, out, time_sec, turns = 0.0, 0, 0, 0.0, 0

    for key, time_key in [
        ("analyzer_results", "execution_time_seconds"),
        ("planning_results", "execution_time_seconds"),
    ]:
        block = data.get(key)
        if block is None:
            continue
        agent_block = block.get("agent_output") or block
        u, i, o, t, n = extract_from_block(agent_block, execution_time_key=time_key)
        if time_key in (block or {}):
            t = float(block.get(time_key) or 0)
        usd += u
        inp += i
        out += o
        time_sec += t
        turns += n

    trans = data.get("translator_results") or {}
    for it in trans.get("iteration_results") or []:
        # Translator
        trans_res = (it.get("translator_results") or {}).get("agent_output") or it.get("translator_results")
        u, i, o, _, n = extract_from_block(trans_res or {}, execution_time_key="")
        usd += u
        inp += i
        out += o
        turns += n
        time_sec += float(it.get("translator_time") or 0)
        # Validator
        val_res = (it.get("validator_results") or {}).get("agent_output") or it.get("validator_results")
        u, i, o, _, n = extract_from_block(val_res or {}, execution_time_key="")
        usd += u
        inp += i
        out += o
        turns += n
        time_sec += float(it.get("validator_time") or 0)

    return usd, inp, out, time_sec, turns


def extract_baseagent_costs(data):
    """Total cost from baseagent_results."""
    block = data.get("baseagent_results")
    if not block:
        return 0.0, 0, 0, 0.0, 0
    agent_block = block.get("agent_output") or block
    usd, inp, out, time_sec, turns = extract_from_block(agent_block, execution_time_key="execution_time_seconds")
    if "execution_time_seconds" in (block or {}):
        time_sec = float(block.get("execution_time_seconds") or 0)
    return usd, inp, out, time_sec, turns


def _init_metrics_dict() -> dict:
    """Helper to create a fresh metrics accumulator."""
    return {
        "usdCost": 0.0,
        "input_tokens": 0,
        "output_tokens": 0,
        "time_seconds": 0.0,
        "num_turns": 0,
    }


def _accumulate_from_last_json(metrics: dict, last_json: dict) -> None:
    """Accumulate tokens, USD, and turns into metrics from a last_json block."""
    if not last_json:
        return
    usd, inp, out = sum_model_usage(last_json.get("modelUsage"))
    metrics["usdCost"] += usd
    metrics["input_tokens"] += inp
    metrics["output_tokens"] += out
    metrics["num_turns"] += int(last_json.get("num_turns") or 0)


def _add_metrics(dst: dict, src: dict) -> None:
    """Add metrics from src into dst in-place."""
    for key in ("usdCost", "input_tokens", "output_tokens", "time_seconds", "num_turns"):
        dst[key] += src.get(key, 0)


def compute_phase_costs_recodeagent(data: dict) -> dict:
    """Compute per-phase costs (analyzer, planning, translator, validator) for a single project."""
    phases: dict[str, dict] = {}

    # Analyzer and planning phases
    for key, phase_name in [("analyzer_results", "analyzer"), ("planning_results", "planning")]:
        block = data.get(key)
        if not block:
            continue
        metrics = _init_metrics_dict()
        metrics["time_seconds"] += float(block.get("execution_time_seconds") or 0.0)
        agent_block = block.get("agent_output") or block
        last = get_last_json(agent_block)
        _accumulate_from_last_json(metrics, last)
        phases[phase_name] = metrics

    # Translator and validator phases (from iteration_results)
    trans = data.get("translator_results") or {}
    iteration_results = trans.get("iteration_results") or []
    if iteration_results:
        trans_metrics = _init_metrics_dict()
        val_metrics = _init_metrics_dict()
        for it in iteration_results:
            # Translator
            t_block = (it.get("translator_results") or {}).get("agent_output") or it.get("translator_results")
            last_t = get_last_json(t_block or {})
            _accumulate_from_last_json(trans_metrics, last_t)
            trans_metrics["time_seconds"] += float(it.get("translator_time") or 0.0)
            # Validator
            v_block = (it.get("validator_results") or {}).get("agent_output") or it.get("validator_results")
            last_v = get_last_json(v_block or {})
            _accumulate_from_last_json(val_metrics, last_v)
            val_metrics["time_seconds"] += float(it.get("validator_time") or 0.0)

        phases["translator"] = trans_metrics
        phases["validator"] = val_metrics

    return phases


def collect_costs_by_agent():
    """Scan trajectories dir and aggregate cost metrics per agent."""
    by_agent = defaultdict(
        lambda: {"usdCost": 0.0, "input_tokens": 0, "output_tokens": 0, "time_seconds": 0.0, "num_turns": 0, "count": 0}
    )

    if not TRAJECTORIES_DIR.is_dir():
        return dict(by_agent)

    for path in sorted(TRAJECTORIES_DIR.iterdir()):
        if not path.is_dir():
            continue
        # Only include trajectories for COST_TRAJECTORY_AGENT; second segment is tool
        parts = path.name.split(".", 2)
        if len(parts) < 2 or parts[0] != COST_TRAJECTORY_AGENT:
            continue
        tool = parts[1]
        if tool not in RECODEAGENT_TOOLS:
            continue
        details_file = path / "project_details.json"
        if not details_file.is_file():
            continue
        try:
            with open(details_file, "r") as f:
                data = json.load(f)
        except (json.JSONDecodeError, OSError):
            continue

        if "baseagent_results" in data:
            usd, inp, out, time_sec, turns = extract_baseagent_costs(data)
        else:
            usd, inp, out, time_sec, turns = extract_recodeagent_costs(data)

        by_agent[tool]["usdCost"] += usd
        by_agent[tool]["input_tokens"] += inp
        by_agent[tool]["output_tokens"] += out
        by_agent[tool]["time_seconds"] += time_sec
        by_agent[tool]["num_turns"] += turns
        by_agent[tool]["count"] += 1

    return dict(by_agent)


def collect_detailed_costs() -> dict:
    """Collect detailed per-project, per-tool, per-phase costs for COST_TRAJECTORY_AGENT."""
    detailed: dict[str, dict[str, dict[str, dict]]] = {}

    if not TRAJECTORIES_DIR.is_dir():
        return detailed

    for path in sorted(TRAJECTORIES_DIR.iterdir()):
        if not path.is_dir():
            continue
        parts = path.name.split(".", 2)
        if len(parts) < 2 or parts[0] != COST_TRAJECTORY_AGENT:
            continue
        tool = parts[1]
        if tool not in RECODEAGENT_TOOLS:
            continue
        details_file = path / "project_details.json"
        if not details_file.is_file():
            continue
        try:
            with open(details_file, "r") as f:
                data = json.load(f)
        except (json.JSONDecodeError, OSError):
            continue

        project_name = data.get("project_name") or parts[2] if len(parts) > 2 else path.name
        phase_costs = compute_phase_costs_recodeagent(data)
        if not phase_costs:
            continue
        tool_dict = detailed.setdefault(tool, {})
        tool_dict[project_name] = phase_costs

    return detailed


def print_detailed_costs(detailed: dict) -> None:
    """Print human-readable cost breakdowns for every project and tool."""
    if not detailed:
        return

    global_totals = _init_metrics_dict()
    # Phase ("agent") level breakdowns. These capture each project's per-phase metrics
    # before we sum across phases into the existing per-tool and global totals.
    phase_order = ["analyzer", "planning", "translator", "validator"]
    global_phase_project_totals: dict[str, list[dict]] = defaultdict(list)
    per_tool_phase_project_totals: dict[str, dict[str, list[dict]]] = defaultdict(lambda: defaultdict(list))
    # Per-project totals across all tools
    project_totals_list: list[dict] = []
    # Per-tool aggregates of per-project totals
    per_tool_totals: dict[str, dict] = {}
    per_tool_project_totals: dict[str, list[dict]] = {}

    for tool in RECODEAGENT_TOOLS:
        tool_projects = detailed.get(tool)
        if not tool_projects:
            continue
        print(f"=== {TOOL_DISPLAY_LABELS.get(tool, tool)} ===")
        for project in sorted(tool_projects.keys()):
            print(f"{project}:")
            phases = tool_projects[project]
            project_totals = _init_metrics_dict()
            for phase_key, phase_label in [
                ("analyzer", "analyzer"),
                ("planning", "planning"),
                ("translator", "translator"),
                ("validator", "validator"),
            ]:
                metrics = phases.get(phase_key)
                if not metrics:
                    continue
                print(f"  {phase_label} cost:")
                print(f"    total_input_tokens: {metrics['input_tokens']}")
                print(f"    total_output_tokens: {metrics['output_tokens']}")
                print(f"    total_usd: {metrics['usdCost']:.6f}")
                print(f"    total_time_seconds: {metrics['time_seconds']:.2f}")
                print(f"    total_num_turns: {metrics['num_turns']}")
                _add_metrics(project_totals, metrics)
                _add_metrics(global_totals, metrics)
                global_phase_project_totals[phase_label].append(metrics)
                per_tool_phase_project_totals[tool][phase_label].append(metrics)
            # Per-project totals across all phases
            project_totals_list.append(project_totals)
            # Track per-tool aggregates of project totals
            tool_totals = per_tool_totals.setdefault(tool, _init_metrics_dict())
            _add_metrics(tool_totals, project_totals)
            per_tool_project_totals.setdefault(tool, []).append(project_totals)
            print(f"  total cost:")
            print(f"    total_input_tokens: {project_totals['input_tokens']}")
            print(f"    total_output_tokens: {project_totals['output_tokens']}")
            print(f"    total_usd: {project_totals['usdCost']:.6f}")
            print(f"    total_time_seconds: {project_totals['time_seconds']:.2f}")
            print(f"    total_num_turns: {project_totals['num_turns']}")
            print()

    # Per-tool totals (min/avg/max/total across that tool's projects)
    if per_tool_totals:
        print("=== PER-TOOL TOTALS ===")
        for tool in RECODEAGENT_TOOLS:
            tool_totals = per_tool_totals.get(tool)
            projects = per_tool_project_totals.get(tool) or []
            if not tool_totals or not projects:
                continue
            n_projects = len(projects)
            label = TOOL_DISPLAY_LABELS.get(tool, tool)
            min_vals = {k: min(p[k] for p in projects) for k in tool_totals.keys()}
            max_vals = {k: max(p[k] for p in projects) for k in tool_totals.keys()}
            avg_vals = {k: tool_totals[k] / n_projects for k in tool_totals.keys()}

            print(f"== {label} ==")
            print(
                f"  total_input_tokens: "
                f"min={min_vals['input_tokens']} "
                f"avg={avg_vals['input_tokens']:.2f} "
                f"max={max_vals['input_tokens']} "
                f"total={tool_totals['input_tokens']}"
            )
            print(
                f"  total_output_tokens: "
                f"min={min_vals['output_tokens']} "
                f"avg={avg_vals['output_tokens']:.2f} "
                f"max={max_vals['output_tokens']} "
                f"total={tool_totals['output_tokens']}"
            )
            print(
                f"  total_usd: "
                f"min={min_vals['usdCost']:.6f} "
                f"avg={avg_vals['usdCost']:.6f} "
                f"max={max_vals['usdCost']:.6f} "
                f"total={tool_totals['usdCost']:.6f}"
            )
            print(
                f"  total_time_seconds: "
                f"min={min_vals['time_seconds']:.2f} "
                f"avg={avg_vals['time_seconds']:.2f} "
                f"max={max_vals['time_seconds']:.2f} "
                f"total={tool_totals['time_seconds']:.2f}"
            )
            print(
                f"  total_num_turns: "
                f"min={min_vals['num_turns']} "
                f"avg={avg_vals['num_turns']:.2f} "
                f"max={max_vals['num_turns']} "
                f"total={tool_totals['num_turns']}"
            )
            print()

    # Per-tool totals broken down by phase ("agent")
    if per_tool_phase_project_totals:
        print("=== PER-TOOL TOTALS BY AGENT ===")
        metric_keys = tuple(_init_metrics_dict().keys())
        for tool in RECODEAGENT_TOOLS:
            phase_lists = per_tool_phase_project_totals.get(tool) or {}
            if not phase_lists:
                continue
            label = TOOL_DISPLAY_LABELS.get(tool, tool)
            print(f"== {label} (by agent) ==")
            for phase_label in phase_order:
                projects = phase_lists.get(phase_label) or []
                if not projects:
                    continue
                n_projects = len(projects)
                min_vals = {k: min(p[k] for p in projects) for k in metric_keys}
                max_vals = {k: max(p[k] for p in projects) for k in metric_keys}
                totals = {k: sum(p[k] for p in projects) for k in metric_keys}
                avg_vals = {k: totals[k] / n_projects for k in metric_keys}

                print(f"  {phase_label} cost:")
                print(
                    f"    total_input_tokens: "
                    f"min={min_vals['input_tokens']} "
                    f"avg={avg_vals['input_tokens']:.2f} "
                    f"max={max_vals['input_tokens']} "
                    f"total={totals['input_tokens']}"
                )
                print(
                    f"    total_output_tokens: "
                    f"min={min_vals['output_tokens']} "
                    f"avg={avg_vals['output_tokens']:.2f} "
                    f"max={max_vals['output_tokens']} "
                    f"total={totals['output_tokens']}"
                )
                print(
                    f"    total_usd: "
                    f"min={min_vals['usdCost']:.6f} "
                    f"avg={avg_vals['usdCost']:.6f} "
                    f"max={max_vals['usdCost']:.6f} "
                    f"total={totals['usdCost']:.6f}"
                )
                print(
                    f"    total_time_seconds: "
                    f"min={min_vals['time_seconds']:.2f} "
                    f"avg={avg_vals['time_seconds']:.2f} "
                    f"max={max_vals['time_seconds']:.2f} "
                    f"total={totals['time_seconds']:.2f}"
                )
                print(
                    f"    total_num_turns: "
                    f"min={min_vals['num_turns']} "
                    f"avg={avg_vals['num_turns']:.2f} "
                    f"max={max_vals['num_turns']} "
                    f"total={totals['num_turns']}"
                )
            print()

    # Global totals across all tools and projects
    print("=== GLOBAL TOTAL ===")
    if project_totals_list:
        n_projects = len(project_totals_list)
        min_vals = {k: min(p[k] for p in project_totals_list) for k in global_totals.keys()}
        max_vals = {k: max(p[k] for p in project_totals_list) for k in global_totals.keys()}
        avg_vals = {k: global_totals[k] / n_projects for k in global_totals.keys()}

        print(
            f"  total_input_tokens: "
            f"min={min_vals['input_tokens']} "
            f"avg={avg_vals['input_tokens']:.2f} "
            f"max={max_vals['input_tokens']} "
            f"total={global_totals['input_tokens']}"
        )
        print(
            f"  total_output_tokens: "
            f"min={min_vals['output_tokens']} "
            f"avg={avg_vals['output_tokens']:.2f} "
            f"max={max_vals['output_tokens']} "
            f"total={global_totals['output_tokens']}"
        )
        print(
            f"  total_usd: "
            f"min={min_vals['usdCost']:.6f} "
            f"avg={avg_vals['usdCost']:.6f} "
            f"max={max_vals['usdCost']:.6f} "
            f"total={global_totals['usdCost']:.6f}"
        )
        print(
            f"  total_time_seconds: "
            f"min={min_vals['time_seconds']:.2f} "
            f"avg={avg_vals['time_seconds']:.2f} "
            f"max={max_vals['time_seconds']:.2f} "
            f"total={global_totals['time_seconds']:.2f}"
        )
        print(
            f"  total_num_turns: "
            f"min={min_vals['num_turns']} "
            f"avg={avg_vals['num_turns']:.2f} "
            f"max={max_vals['num_turns']} "
            f"total={global_totals['num_turns']}"
        )
    else:
        print(f"  total_input_tokens: {global_totals['input_tokens']}")
        print(f"  total_output_tokens: {global_totals['output_tokens']}")
        print(f"  total_usd: {global_totals['usdCost']:.6f}")
        print(f"  total_time_seconds: {global_totals['time_seconds']:.2f}")
        print(f"  total_num_turns: {global_totals['num_turns']}")

    # Global totals broken down by phase ("agent")
    if global_phase_project_totals:
        print("=== GLOBAL TOTAL BY AGENT ===")
        metric_keys = tuple(_init_metrics_dict().keys())
        for phase_label in phase_order:
            projects = global_phase_project_totals.get(phase_label) or []
            if not projects:
                continue
            n_projects = len(projects)
            min_vals = {k: min(p[k] for p in projects) for k in metric_keys}
            max_vals = {k: max(p[k] for p in projects) for k in metric_keys}
            totals = {k: sum(p[k] for p in projects) for k in metric_keys}
            avg_vals = {k: totals[k] / n_projects for k in metric_keys}

            print(f"  {phase_label} cost:")
            print(
                f"    total_input_tokens: "
                f"min={min_vals['input_tokens']} "
                f"avg={avg_vals['input_tokens']:.2f} "
                f"max={max_vals['input_tokens']} "
                f"total={totals['input_tokens']}"
            )
            print(
                f"    total_output_tokens: "
                f"min={min_vals['output_tokens']} "
                f"avg={avg_vals['output_tokens']:.2f} "
                f"max={max_vals['output_tokens']} "
                f"total={totals['output_tokens']}"
            )
            print(
                f"    total_usd: "
                f"min={min_vals['usdCost']:.6f} "
                f"avg={avg_vals['usdCost']:.6f} "
                f"max={max_vals['usdCost']:.6f} "
                f"total={totals['usdCost']:.6f}"
            )
            print(
                f"    total_time_seconds: "
                f"min={min_vals['time_seconds']:.2f} "
                f"avg={avg_vals['time_seconds']:.2f} "
                f"max={max_vals['time_seconds']:.2f} "
                f"total={totals['time_seconds']:.2f}"
            )
            print(
                f"    total_num_turns: "
                f"min={min_vals['num_turns']} "
                f"avg={avg_vals['num_turns']:.2f} "
                f"max={max_vals['num_turns']} "
                f"total={totals['num_turns']}"
            )
        print()


def _extract_tool_names_from_event(event: dict) -> list[str]:
    """Extract tool names from a single trajectory event object."""
    tool_names: list[str] = []

    # Top-level tool_use-style event
    if isinstance(event, dict) and event.get("type") == "tool_use":
        name = event.get("name")
        if isinstance(name, str) and name:
            tool_names.append(name)

    # Nested tool_use entries under message.content
    message = event.get("message")
    if isinstance(message, dict):
        content = message.get("content")
        if isinstance(content, list):
            for part in content:
                if not isinstance(part, dict):
                    continue
                if part.get("type") == "tool_use":
                    name = part.get("name")
                    if isinstance(name, str) and name:
                        tool_names.append(name)

    return tool_names


def _accumulate_tool_usage_from_file(jsonl_path: Path, counts: dict) -> None:
    """Accumulate tool-usage counts from a single JSONL trajectory file."""
    try:
        with open(jsonl_path, "r") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    event = json.loads(line)
                except json.JSONDecodeError:
                    continue
                for tool_name in _extract_tool_names_from_event(event):
                    counts[tool_name] += 1
    except OSError:
        # Ignore files that cannot be opened
        return


def collect_tool_usage_recodeagent() -> tuple[dict[str, int], set[str]]:
    """Collect tool-usage frequencies for COST_TRAJECTORY_AGENT trajectories.
    Returns (counts_by_tool_name, set of tool names that are MCP-derived).
    """
    if not TRAJECTORIES_DIR.is_dir():
        return {}, set()

    counts: dict[str, int] = defaultdict(int)

    for path in sorted(TRAJECTORIES_DIR.iterdir()):
        if not path.is_dir():
            continue
        parts = path.name.split(".", 2)
        if len(parts) < 2 or parts[0] != COST_TRAJECTORY_AGENT:
            continue

        workspace_dir = path / "-workspace"
        if not workspace_dir.is_dir():
            continue

        # Top-level workspace trajectories
        for jsonl_file in workspace_dir.glob("*.jsonl"):
            _accumulate_tool_usage_from_file(jsonl_file, counts)

        # Subagent trajectories nested under -workspace/**/subagents/*.jsonl
        for jsonl_file in workspace_dir.glob("**/subagents/*.jsonl"):
            _accumulate_tool_usage_from_file(jsonl_file, counts)

    # Post-process to deduplicate MCP tools by tool_name (last segment after "__").
    # Example: "mcp__server_name__tool_name" -> "tool_name"
    aggregated: dict[str, int] = defaultdict(int)
    mcp_tool_names: set[str] = set()
    for full_name, count in counts.items():
        if isinstance(full_name, str) and full_name.startswith("mcp__"):
            parts = full_name.split("__")
            if len(parts) >= 3:
                key = parts[-1]
                mcp_tool_names.add(key)
            else:
                key = full_name
        else:
            key = full_name
        aggregated[key] += count

    # Return (counts, set of tool names that are MCP-derived for star suffix in plot)
    return dict(aggregated), mcp_tool_names


def plot_and_save(by_agent, tool_usage_counts: dict, mcp_tool_names: set[str] | None = None):
    """Draw grouped bar chart with double y-axis and save to PDF."""
    # Preserve order: only include tools in RECODEAGENT_TOOLS that have data
    agents = [t for t in RECODEAGENT_TOOLS if t in by_agent]
    n_agents = len(agents)
    n_left = len(TOKEN_ITEMS)
    n_right = len(OTHER_ITEMS)
    n_total = n_left + n_right
    bar_width = 0.2
    group_width = n_total * bar_width
    # Space between groups: one bar width gap between tool groups
    group_spacing = group_width + bar_width if n_agents > 0 else bar_width
    x = [i * group_spacing for i in range(n_agents)]
    # Center 5 bars around each tick; tick is under the middle bar
    offsets = [bar_width * (j - (n_total - 1) / 2) for j in range(n_total)]

    # Two-row layout: top = cost bars, bottom = tool-usage frequency
    fig, (ax, ax_usage) = plt.subplots(
        nrows=2,
        ncols=1,
        figsize=(max(10, n_agents * 1.2), 6),
        gridspec_kw={"height_ratios": [1.5, 1], "hspace": 0.15},
    )
    ax2 = ax.twinx()
    colors = plt.cm.tab10.colors
    token_colors = [colors[1], colors[2]]
    other_colors = [colors[0], colors[3], colors[4]]
    colors_all = token_colors + other_colors

    all_items = TOKEN_ITEMS + OTHER_ITEMS
    all_labels = TOKEN_LABELS + OTHER_LABELS

    if n_agents > 0:
        for j, (key, label) in enumerate(zip(all_items, all_labels)):
            if j < n_left:
                # Tokens on left axis, normalized by number of trajectories
                vals = [by_agent[a][key] / by_agent[a]["count"] for a in agents]
                axis = ax
            else:
                # Other metrics on right axis, time converted to minutes and normalized
                if key == "time_seconds":
                    vals = [by_agent[a]["time_seconds"] / 60 / by_agent[a]["count"] for a in agents]
                else:
                    vals = [by_agent[a][key] / by_agent[a]["count"] for a in agents]
                axis = ax2
            pos = [xi + offsets[j] for xi in x]
            axis.bar(
                pos,
                vals,
                width=bar_width * 0.9,
                label=label,
                color=colors_all[j],
                edgecolor="black",
                linewidth=0.4,
            )

        # X-axis ticks and labels
        ax.set_xticks(x)
        ax.set_xticklabels([TOOL_DISPLAY_LABELS.get(a, a) for a in agents])

        # Axis labels with consistent font size
        ax.set_ylabel("Tokens (input / output)", fontsize=FONT_SIZE)
        ax2.set_ylabel("Cost ($), Time (min), # Turns", fontsize=FONT_SIZE)

        # Gridlines on primary (left) axis for readability
        ax.grid(axis="y", linestyle="--", linewidth=0.6, alpha=0.7)

        # Match tick font sizes on both axes
        ax.tick_params(axis="both", labelsize=FONT_SIZE)
        ax2.tick_params(axis="both", labelsize=FONT_SIZE)

        # Single combined legend for both y-axes
        handles1, labels1 = ax.get_legend_handles_labels()
        handles2, labels2 = ax2.get_legend_handles_labels()
        handles = handles1 + handles2
        labels = labels1 + labels2
        ax.legend(handles, labels, loc="upper right", fontsize=12)
    else:
        # If no by-agent data, hide the top axes while still keeping the figure layout.
        ax.axis("off")
        ax2.axis("off")

    # Bottom subplot: tool-usage frequency across COST_TRAJECTORY_AGENT trajectories
    if tool_usage_counts:
        mcp_set = mcp_tool_names or set()
        sorted_items = sorted(tool_usage_counts.items(), key=lambda kv: kv[1], reverse=True)
        # Filter out run_tests tool entirely
        sorted_items = [item for item in sorted_items if item[0] != "run_tests"]
        if sorted_items:
            # Show the most frequently used tools for readability
            top_n = min(25, len(sorted_items))
            sorted_items = sorted_items[:top_n]
            tools, counts = zip(*sorted_items)
            positions = range(len(tools))

            # Label with * for MCP-derived tool names and apply short aliases for some long names
            alias_map = {
                "get_file_structure": "file_structure",
                "get_directory_tree": "directory_tree",
                "rename_symbol": "rename_sym",
            }
            labels = []
            for t in tools:
                base = alias_map.get(t, t)
                if t in mcp_set:
                    labels.append(f"{base} *")
                else:
                    labels.append(base)

            ax_usage.bar(
                positions,
                counts,
                color="0.6",
                edgecolor="black",
                linewidth=0.4,
            )
            ax_usage.set_xticks(list(positions))
            ax_usage.set_xticklabels(labels, rotation=45, ha="right")
            # Nudge x-tick labels slightly to the right for better centering
            for tick_label in ax_usage.get_xticklabels():
                x, y = tick_label.get_position()
                tick_label.set_position((x + 0.1, y))
            ax_usage.set_ylabel("Tool Usage Count", fontsize=FONT_SIZE)
            # Log scale for counts; minimum at 1 to avoid log(0)
            ax_usage.set_yscale("log")
            ax_usage.set_ylim(1, 10000)

            ax_usage.grid(axis="y", linestyle="--", linewidth=0.6, alpha=0.7)
            ax_usage.tick_params(axis="both", labelsize=FONT_SIZE)
        else:
            ax_usage.axis("off")
    else:
        ax_usage.axis("off")

    fig.tight_layout()
    fig.savefig(OUTPUT_PDF, format="pdf", bbox_inches="tight", dpi=300)
    plt.close()


def main():
    detailed = collect_detailed_costs()
    print_detailed_costs(detailed)
    by_agent = collect_costs_by_agent()
    tool_usage_counts, mcp_tool_names = collect_tool_usage_recodeagent()
    # Print tool-usage counts to stdout (sorted, with * for MCP-derived tools)
    if tool_usage_counts:
        print("=== Tool usage (deduped by tool_name) ===")
        for name, count in sorted(tool_usage_counts.items(), key=lambda kv: kv[1], reverse=True):
            suffix = " *" if name in mcp_tool_names else ""
            print(f"{name}{suffix}: {count}")
    plot_and_save(by_agent, tool_usage_counts, mcp_tool_names)


if __name__ == "__main__":
    main()
