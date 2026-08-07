# STATUS

## Badges Sought

We apply for the following ACM artifact badges ([Artifact Review and Badging Version 1.1](https://www.acm.org/publications/policies/artifact-review-and-badging-current)):

1. **Artifacts Available**
2. **Artifacts Evaluated — Functional**
3. **Artifacts Evaluated — Reusable**

## Justification

### Artifacts Available

Author-created artifacts for this paper are publicly archived on Zenodo with a persistent DOI:

- **DOI:** [https://doi.org/10.5281/zenodo.19214481](https://doi.org/10.5281/zenodo.19214481)

The deposit includes `results.zip` (translations, ablation agents, trajectories, graphectory analysis, cost data) and `results.xlsx` (detailed per-tool and per-project metrics). This repository (source code, Docker packaging, scripts, and documentation) is intended to be obtained via the same archival deposit so the complete artifact is available under a DOI.

### Artifacts Evaluated — Functional

The artifact is documented, consistent with the paper, complete, and exercisable, as described in [`README.md`](./README.md):

- **Getting Started** — artifact description, Docker installation, Zenodo results, credentials, and project structure
- **Step-by-step instructions** — reproduce RQ1–RQ4 via `scripts/start_docker.sh`, `scripts/run_test_comparison.sh`, `src/analysis/ablation.py`, and `src/analysis/cost.py`, using either fresh runs or unpacked Zenodo `results.zip` under `results/`
- Docker packaging under `docker-env/` installs dependencies for the supported language pairs

LLM API keys are not included; users configure Bedrock or OpenRouter credentials as in `README.md` § Credentials. Empirical outputs matching the paper are also provided on Zenodo.

### Artifacts Evaluated — Reusable

The artifact is carefully structured for reuse and extension beyond minimal functionality:

- Modular multi-agent pipeline (`src/agents/recodeagent/`) and baselines (`src/agents/baseagent/`)
- MCP servers and analysis utilities (`src/mcp/`, `src/analysis/`)
- Docker environment covering six PLs and four PL pairs (crust, oxidizer, alphatrans, skel)
- Documented guidance for new projects, languages, and MCP servers (`README.md` § Building on ReCodeAgent)
