import csv
import math
from pathlib import Path
from typing import Iterable, Mapping

import matplotlib.pyplot as plt
import numpy as np
from scipy import stats

try:
    import seaborn as sns

    _HAS_SEABORN = True
except ImportError:
    _HAS_SEABORN = False


plt.rcParams.update(
    {
        "font.family": "serif",
        "font.serif": [
            "Times New Roman",
            "Times",
            "Liberation Serif",
            "DejaVu Serif",
            "serif",
        ],
        "font.size": 12,
    }
)


REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "results" / "ablation_study" / "ablation-study-effectiveness.csv"
GRAPHECTORY_DIR = REPO_ROOT / "results" / "ablation_study" / "graphectory_analysis"
PDF_PATH = REPO_ROOT / "ablation.pdf"

TOOL_ORDER = ["oxidizer", "alphatrans", "skel", "swe-agent"]
TOOL_TITLES = {
    "oxidizer": "Oxidizer",
    "alphatrans": "AlphaTrans",
    "skel": "Skel",
    "swe-agent": "SWE-agent",
}
AGENT_ORDER = [
    "RecodeAgent",
    "NoAnalyzer",
    "NoPlanning",
    "NoValidator",
    "BaseAgentCondense",
    "BaseAgentConcat",
]
AGENT_LABELS = {
    "RecodeAgent": "RA",
    "NoAnalyzer": "NoA",
    "NoPlanning": "NoP",
    "NoValidator": "NoV",
    "BaseAgentCondense": r"BA-$\alpha$",
    "BaseAgentConcat": r"BA-$\beta$",
}

# Mapping from graphectory directory agent slug to ablation agent key
GRAPHECTORY_AGENT_MAP: dict[str, str] = {
    "recodeagent": "RecodeAgent",
    "noanalyzer": "NoAnalyzer",
    "noplanning": "NoPlanning",
    "novalidator": "NoValidator",
    "baseagent-condensed": "BaseAgentCondense",
    "baseagent-concat": "BaseAgentConcat",
}

# Metrics to extract from graphectory CSVs (columns):
# node_count, exec_edge_count, hier_edge_count, loop_count, avg_loop_length
# and their short labels:
# NC, TEC, SEC, LC, ALL (in that order).
HEATMAP_METRICS = [
    "node_count",
    "exec_edge_count",
    "hier_edge_count",
    "loop_count",
    "avg_loop_length",
]
HEATMAP_METRIC_LABELS = ["NC", "TEC", "SEC", "LC", "ALL"]


def _parse_number(value: str) -> float:
    try:
        result = float(value)
        return 1000.0 if result == 10000.0 else result
    except (TypeError, ValueError):
        return 0.0


def _read_rows() -> Iterable[Mapping[str, str]]:
    with open(CSV_PATH, newline="") as csv_file:
        return list(csv.DictReader(csv_file))


def _collect_per_project_rates(
    rows: Iterable[Mapping[str, str]],
) -> dict[str, dict[str, list[float]]]:
    """Collect per-project test validation rates for each tool/agent combination."""
    agent_tp_percent_columns = {
        "RecodeAgent": "TP - RecodeAgent (%)",
        "NoAnalyzer": "TP - NoAnalyzer (%)",
        "NoPlanning": "TP - NoPlanning (%)",
        "NoValidator": "TP - NoValidator (%)",
        "BaseAgentCondense": "TP - BaseAgentCondense (%)",
        "BaseAgentConcat": "TP - BaseAgentConcat (%)",
    }

    # Structure: tool -> agent -> list of percentages
    data: dict[str, dict[str, list[float]]] = {}

    for row in rows:
        tool = row.get("tool", "").strip()
        if not tool:
            continue
        if tool.lower() == "total":
            continue

        if tool not in data:
            data[tool] = {agent: [] for agent in AGENT_ORDER}

        for agent, percent_col in agent_tp_percent_columns.items():
            percent = _parse_number(row.get(percent_col))
            data[tool][agent].append(percent)

    return data


def _aggregate_rates(
    rows: Iterable[Mapping[str, str]],
) -> tuple[dict[str, dict[str, dict[str, float]]], dict[str, float], dict[str, float]]:
    """Keep aggregation for summary printing."""
    agent_tp_columns = {
        "RecodeAgent": "TP - RecodeAgent",
        "NoAnalyzer": "TP - NoAnalyzer",
        "NoPlanning": "TP - NoPlanning",
        "NoValidator": "TP - NoValidator",
        "BaseAgentCondense": "TP - BaseAgentCondense",
        "BaseAgentConcat": "TP - BaseAgentConcat",
    }

    summary: dict[str, dict[str, dict[str, float]]] = {}
    rows_by_tool: dict[str, list[Mapping[str, str]]] = {}
    for row in rows:
        tool = row.get("tool", "").strip()
        if not tool:
            continue
        if tool.lower() == "total":
            continue
        rows_by_tool.setdefault(tool, []).append(row)

    tool_fragments: dict[str, float] = {}
    tool_tests: dict[str, float] = {}
    for tool, group in rows_by_tool.items():
        total_tests = sum(_parse_number(row.get("# tests")) for row in group)
        tool_tests[tool] = total_tests
        total_frags = sum(_parse_number(row.get("# fragments")) for row in group)
        counts_for_tool: dict[str, dict[str, float]] = {}
        for agent, tp_col in agent_tp_columns.items():
            tp_total = sum(_parse_number(row.get(tp_col)) for row in group)
            tp_rate = 0.0 if math.isclose(total_tests, 0) else 100 * tp_total / total_tests
            counts_for_tool[agent] = {
                "tp_rate": tp_rate,
                "tp_total": tp_total,
            }
        summary[tool] = counts_for_tool
        tool_fragments[tool] = total_frags
    return summary, tool_fragments, tool_tests


def _collect_graphectory_metrics(
    base_dir: Path,
) -> tuple[dict[str, dict[str, dict[str, float]]], float | None, float | None]:
    """Aggregate graphectory metrics per (tool, agent).

    Aggregation is:
    1) For each CSV (one project for an agent/tool), average each metric over rows.
    2) For each (tool, agent), average those per-CSV means across projects.
    """
    aggregated: dict[str, dict[str, dict[str, float]]] = {}
    # Temporary storage: tool -> agent -> metric -> list[float]
    per_tool_agent: dict[str, dict[str, dict[str, list[float]]]] = {}

    if not base_dir.exists():
        return aggregated, None, None

    all_values: list[float] = []

    for csv_path in base_dir.glob("*/*.csv"):
        parent_name = csv_path.parent.name
        parts = parent_name.split(".")
        if len(parts) < 2:
            continue

        agent_slug = parts[0]
        tool = parts[1]

        # Map directory naming ('crust') to ablation naming ('swe-agent') for tools.
        if tool == "crust":
            tool = "swe-agent"

        agent = GRAPHECTORY_AGENT_MAP.get(agent_slug)
        if agent is None:
            continue

        try:
            with csv_path.open(newline="") as f:
                reader = csv.DictReader(f)
                rows = list(reader)
        except OSError:
            continue

        if not rows:
            continue

        # Compute per-CSV averages for each metric
        csv_means: dict[str, float] = {}
        for metric in HEATMAP_METRICS:
            values = [_parse_number(row.get(metric)) for row in rows]
            if not values:
                continue
            mean_val = float(sum(values) / len(values))
            csv_means[metric] = mean_val

        if not csv_means:
            continue

        for metric, mean_val in csv_means.items():
            tool_dict = per_tool_agent.setdefault(tool, {})
            agent_dict = tool_dict.setdefault(agent, {})
            metric_list = agent_dict.setdefault(metric, [])
            metric_list.append(mean_val)
            all_values.append(mean_val)

    for tool, agents_dict in per_tool_agent.items():
        for agent, metrics_dict in agents_dict.items():
            for metric, values in metrics_dict.items():
                if not values:
                    continue
                mean_val = float(sum(values) / len(values))
                aggregated.setdefault(tool, {}).setdefault(agent, {})[metric] = mean_val

    if not all_values:
        return aggregated, None, None
    global_min = float(min(all_values))
    global_max = float(max(all_values))
    return aggregated, global_min, global_max


def _plot_test_validation(
    per_project_data: dict[str, dict[str, list[float]]],
    graphectory_metrics: dict[str, dict[str, dict[str, float]]],
    heatmap_vmin: float | None,
    heatmap_vmax: float | None,
) -> None:
    """Plot Test Validation ridgelines (top row) and graphectory heatmaps (bottom row)."""
    agents = AGENT_ORDER
    tools = [tool for tool in TOOL_ORDER if (tool in per_project_data) or (tool in graphectory_metrics)]
    n_agents = len(agents)

    # 2xN: top row ridgelines, bottom row heatmaps (equal height ratios)
    fig, axes = plt.subplots(2, len(tools), figsize=(4 * len(tools), 6))
    if len(tools) == 1:
        # axes is shape (2, 1)
        axes_top = [axes[0, 0]]
        axes_bottom = [axes[1, 0]]
    else:
        axes_top = list(axes[0, :])
        axes_bottom = list(axes[1, :])

    title_mapping = {
        "oxidizer": "Oxidizer",
        "alphatrans": "AlphaTrans",
        "skel": "Skel",
        "swe-agent": "SWE-agent",
    }

    # X axis: Test Validation (%); smooth density (KDE) per ridge
    x_min, x_max = -5.0, 105.0
    x_grid = np.linspace(x_min, x_max, 200)
    ridge_scale = 0.85  # height of each ridge (so they don't overlap)

    for ax, tool in zip(axes_top, tools):
        # Tools like CRUST may have graphectory metrics but no test-validation data.
        data_for_tool = per_project_data.get(tool)
        if data_for_tool is None:
            ax.set_title(title_mapping.get(tool, tool), fontsize=14)
            ax.axis("off")
            continue
        # Bottom-to-top: BA-β, BA-α, NoV, NoP, NoA, RA
        agents_reversed = list(reversed(agents))

        for k, agent in enumerate(agents_reversed):
            values = np.asarray(data_for_tool[agent])
            y_base = k

            if len(values) == 0:
                ax.fill_between(
                    [x_min, x_max],
                    y_base,
                    y_base + ridge_scale,
                    facecolor="#FFB366",
                    alpha=0.3,
                    edgecolor="black",
                    linewidth=0.5,
                )
            else:
                # Kernel density estimate; clip to x range for support
                values_clip = np.clip(values, x_min, x_max)
                bw = "scott" if len(values_clip) > 1 else 15.0  # wide band if n=1
                try:
                    kde = stats.gaussian_kde(values_clip, bw_method=bw)
                    density = kde(x_grid)
                except np.linalg.LinAlgError:
                    density = np.zeros_like(x_grid)
                    density[np.argmin(np.abs(x_grid - np.median(values_clip)))] = 1.0
                # Scale so ridge height is comparable across agents (normalize by max)
                d_max = density.max() if density.max() > 0 else 1.0
                density_scaled = (density / d_max) * ridge_scale
                ax.fill_between(
                    x_grid,
                    y_base,
                    y_base + density_scaled,
                    facecolor="#FFB366",
                    alpha=0.7,
                    edgecolor="black",
                    linewidth=0.5,
                )
                # Dots placed in ascending order: sort values, spread y linearly by rank
                sort_idx = np.argsort(values)
                sorted_values = values[sort_idx]
                n = len(sorted_values)
                if n == 1:
                    y_dot_positions = np.array([ridge_scale / 2])
                else:
                    y_dot_positions = np.linspace(0.05, ridge_scale - 0.05, n)
                ax.scatter(
                    sorted_values,
                    y_base + y_dot_positions,
                    color="black",
                    s=14,
                    alpha=1.0,
                    zorder=3,
                    edgecolors="none",
                    linewidths=0,
                )
                # Dashed vertical line at mean of per-project validation rates
                mean_val = float(np.mean(values))
                ax.plot(
                    [mean_val, mean_val],
                    [y_base, y_base + ridge_scale],
                    "k--",
                    linewidth=1.0,
                    zorder=2,
                )

            # Agent labels only on the leftmost subplot
            if ax is axes_top[0]:
                ax.text(
                    x_min - 3,
                    y_base + ridge_scale / 2,
                    AGENT_LABELS.get(agent, agent),
                    fontsize=11,
                    ha="right",
                    va="center",
                )

        ax.set_xlim(x_min, x_max)
        ax.set_ylim(0, n_agents)
        ax.set_yticks([])
        ax.set_title(title_mapping.get(tool, tool), fontsize=14)
        ax.grid(axis="x", linestyle="--", linewidth=0.6, alpha=0.7)
        ax.tick_params(axis="both", labelsize=12)

    # X label directly under the upper subplots
    for ax in axes_top:
        ax.set_xlabel("Test Validation (%)")

    # Bottom row: graphectory heatmaps
    for col_idx, (ax_hm, tool) in enumerate(zip(axes_bottom, tools)):
        metrics_for_tool = graphectory_metrics.get(tool, {})
        matrix = np.full((n_agents, len(HEATMAP_METRICS)), np.nan, dtype=float)

        for i, agent in enumerate(agents):
            metrics_for_agent = metrics_for_tool.get(agent, {})
            for j, metric_name in enumerate(HEATMAP_METRICS):
                value = metrics_for_agent.get(metric_name)
                if value is not None:
                    matrix[i, j] = value

        mask = np.isnan(matrix)
        has_data = not np.all(mask)

        if has_data:
            vmin = heatmap_vmin if heatmap_vmin is not None else np.nanmin(matrix)
            vmax = heatmap_vmax if heatmap_vmax is not None else np.nanmax(matrix)
        else:
            vmin = vmax = None

        if _HAS_SEABORN:
            # Prepare string annotations with 1 decimal, empty where NaN
            annot = np.empty_like(matrix, dtype=object)
            for i in range(n_agents):
                for j in range(len(HEATMAP_METRICS)):
                    if mask[i, j]:
                        annot[i, j] = ""
                    else:
                        annot[i, j] = f"{matrix[i, j]:.1f}"

            sns.heatmap(
                matrix,
                ax=ax_hm,
                cmap="magma_r",
                vmin=vmin,
                vmax=vmax,
                cbar=False,
                linewidths=0.2,
                linecolor="white",
                xticklabels=HEATMAP_METRIC_LABELS,
                yticklabels=[AGENT_LABELS[a] for a in agents],
                mask=mask,
                annot=annot,
                annot_kws={"family": "monospace"},
                fmt="",
            )
        else:
            if has_data:
                im = ax_hm.imshow(
                    matrix,
                    cmap="magma_r",
                    vmin=vmin,
                    vmax=vmax,
                    aspect="auto",
                )

                # Text annotations with 1 decimal, color depending on value
                threshold = (vmin + vmax) / 2.0 if vmin is not None and vmax is not None else None
                for i in range(n_agents):
                    for j in range(len(HEATMAP_METRICS)):
                        val = matrix[i, j]
                        if np.isnan(val):
                            continue
                        color = "white" if threshold is not None and val >= threshold else "black"
                        ax_hm.text(
                            j,
                            i,
                            f"{val:.1f}",
                            ha="center",
                            va="center",
                            color=color,
                            fontsize=8,
                            fontfamily="monospace",
                        )
            else:
                # Empty heatmap: just draw the grid / ticks, no colored cells
                ax_hm.set_xlim(-0.5, len(HEATMAP_METRICS) - 0.5)
                ax_hm.set_ylim(n_agents - 0.5, -0.5)

            ax_hm.set_xticks(range(len(HEATMAP_METRICS)))
            ax_hm.set_xticklabels(HEATMAP_METRIC_LABELS, rotation=45, ha="right")
            ax_hm.set_yticks(range(n_agents))
            ax_hm.set_yticklabels([AGENT_LABELS[a] for a in agents])
            # Thin grid lines between cells
            ax_hm.set_xticks(np.arange(-0.5, len(HEATMAP_METRICS), 1), minor=True)
            ax_hm.set_yticks(np.arange(-0.5, n_agents, 1), minor=True)
            ax_hm.grid(which="minor", color="white", linestyle="-", linewidth=0.2)
            ax_hm.tick_params(which="minor", bottom=False, left=False)

        # Only show y tick labels on left-most heatmap
        if col_idx != 0:
            ax_hm.set_yticklabels([])

        # Heatmap tick styling: match upper agent label size and remove y tick marks
        ax_hm.tick_params(axis="both", labelsize=11)
        ax_hm.tick_params(axis="y", which="both", length=0)
        if col_idx == 0:
            # Add a bit more left padding so row labels don't hug the heatmap
            ax_hm.tick_params(axis="y", pad=7)

        # Add border around the heatmap subplot (axes frame)
        ax_hm.set_frame_on(True)
        for spine in ax_hm.spines.values():
            spine.set_visible(True)
            spine.set_linewidth(1.0)
            spine.set_color("black")

    plt.tight_layout()
    plt.subplots_adjust(bottom=0.12)
    fig.savefig(PDF_PATH, dpi=300, bbox_inches="tight")


def _print_summary(
    tool_rates: dict[str, dict[str, dict[str, float]]],
    tool_fragments: dict[str, float],
    tool_tests: dict[str, float],
) -> None:
    print("Aggregated counts per tool (TP=Test Validation):")
    for tool in TOOL_ORDER:
        if tool not in tool_rates:
            continue
        title = TOOL_TITLES.get(tool, tool)
        fragments = tool_fragments.get(tool, 0.0)
        tests = tool_tests.get(tool, 0.0)
        print(f"{title} (Fragments: {int(fragments)}, Tests: {int(tests)})")
        for agent in AGENT_ORDER:
            data = tool_rates[tool][agent]
            label = AGENT_LABELS.get(agent, agent)
            tp_total = int(round(data["tp_total"]))
            print(f"  {label}: Test Validation {tp_total} ({data['tp_rate']:.1f}%)")
        print()


def _print_graphectory_summary(
    graphectory_metrics: dict[str, dict[str, dict[str, float]]],
) -> None:
    """Print aggregated graphectory metrics per tool, similar in spirit to test validation."""
    if not graphectory_metrics:
        return

    # Compute totals over projects for each (tool, agent, metric) by re-reading the CSVs.
    tool_agent_metric_project_totals: dict[str, dict[str, dict[str, float]]] = {}
    if GRAPHECTORY_DIR.exists():
        for csv_path in GRAPHECTORY_DIR.glob("*/*.csv"):
            parent_name = csv_path.parent.name
            parts = parent_name.split(".")
            if len(parts) < 2:
                continue

            agent_slug = parts[0]
            tool = parts[1]

            # Map directory naming ('crust') to ablation naming ('swe-agent') for tools.
            if tool == "crust":
                tool = "swe-agent"

            agent = GRAPHECTORY_AGENT_MAP.get(agent_slug)
            if agent is None:
                continue

            try:
                with csv_path.open(newline="") as f:
                    reader = csv.DictReader(f)
                    rows = list(reader)
            except OSError:
                continue

            if not rows:
                continue

            # Per-CSV means for each metric, then add to tool/agent-level totals
            for metric in HEATMAP_METRICS:
                values = [_parse_number(row.get(metric)) for row in rows]
                if not values:
                    continue
                mean_val = float(sum(values) / len(values))
                totals_for_agent = tool_agent_metric_project_totals.setdefault(tool, {}).setdefault(agent, {})
                totals_for_agent[metric] = totals_for_agent.get(metric, 0.0) + mean_val

    print("Graphectory metrics per tool (averaged over projects):")
    for tool in TOOL_ORDER:
        if tool not in graphectory_metrics:
            continue
        title = TOOL_TITLES.get(tool, tool)
        print(title)

        tool_data = graphectory_metrics[tool]

        # Per-agent metric lines
        for agent in AGENT_ORDER:
            agent_metrics = tool_data.get(agent)
            if not agent_metrics:
                continue
            label = AGENT_LABELS.get(agent, agent)
            parts: list[str] = []
            for metric_name, short_label in zip(HEATMAP_METRICS, HEATMAP_METRIC_LABELS):
                val = agent_metrics.get(metric_name)
                if val is None:
                    continue
                parts.append(f"{short_label}={val:.1f}")
            if parts:
                print(f"  {label}: " + ", ".join(parts))

            # Totals over projects for this agent (sum of per-project means)
            agent_totals = tool_agent_metric_project_totals.get(tool, {}).get(agent, {})
            if agent_totals:
                total_parts: list[str] = []
                for metric_name, short_label in zip(HEATMAP_METRICS, HEATMAP_METRIC_LABELS):
                    if metric_name not in agent_totals:
                        continue
                    total_val = agent_totals[metric_name]
                    total_parts.append(f"{short_label}={total_val:.1f}")
                if total_parts:
                    print("    Total over projects: " + ", ".join(total_parts))
        print()


def main() -> None:
    rows = _read_rows()
    tool_rates, tool_fragments, tool_tests = _aggregate_rates(rows)
    _print_summary(tool_rates, tool_fragments, tool_tests)
    per_project_data = _collect_per_project_rates(rows)
    graphectory_metrics, heatmap_vmin, heatmap_vmax = _collect_graphectory_metrics(GRAPHECTORY_DIR)
    _print_graphectory_summary(graphectory_metrics)
    _plot_test_validation(per_project_data, graphectory_metrics, heatmap_vmin, heatmap_vmax)


if __name__ == "__main__":
    main()
