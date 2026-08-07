# REQUIREMENTS

## Architecture

| Item | Requirement |
|------|-------------|
| CPU architecture | **x86_64 / amd64** (`docker-env/Dockerfile` and language toolchains target linux-amd64) |
| ARM / Apple Silicon | Not officially packaged |
| GPU | **Not required** |

## Hardware (recommended)

| Use case | Recommendation |
|----------|----------------|
| Inspecting Zenodo `results.zip` / `results.xlsx` and running analysis scripts (`ablation.py`, `cost.py`, `run_test_comparison.sh`) | ≥ 4 CPU cores, ≥ 8 GB RAM, ≥ **20 GB** free disk |
| Building and running the Docker image (`scripts/start_docker.sh`) | ≥ 8 CPU cores, ≥ 16 GB RAM, ≥ **40 GB** free disk |
| Re-running all paper experiments (118 projects) | Multi-day wall-clock time and non-trivial LLM cost |

## Software

- **Docker** (Engine 24+ recommended) or a Podman-compatible Docker CLI — primary way to run ReCodeAgent (see `README.md` § Getting Started)
- Network access during `docker build` to fetch language toolchains
- Linux host recommended inside/alongside Docker; macOS or Windows with WSL2 may work for host-side scripts
- Python **3.10+** (3.12 recommended) for analysis scripts in `src/analysis/`
- `unzip` (or equivalent) to unpack Zenodo `results.zip`
- Optional host packages for analysis plots / RQ2 similarity: `matplotlib`, `numpy`, `scipy`, `seaborn`, `pandas` (and `scikit-learn` / `sentence-transformers` if embedding similarity is enabled)

## LLM credentials

Running agents requires an LLM backend configured by the user (keys are **not** included in this artifact):

- **AWS Bedrock + Claude** (paper setup): `aws configure` with access key, secret, and region; Claude model enabled in that region (`awscli`)
- **OpenRouter** or other providers: set the `env` block in `.claude/settings.local.json` (see `README.md` § Credentials)

Published experiment outputs are also available on Zenodo without re-running agents: [https://doi.org/10.5281/zenodo.19214481](https://doi.org/10.5281/zenodo.19214481).

## Machine-readable dependency definitions

- Container build: [`docker-env/Dockerfile`](./docker-env/Dockerfile)
- Python packages for the image: `pip install` section in the Dockerfile
- Agent / runtime configuration: [`.claude/`](./.claude/), [`configs/`](./configs/)

## Supported language pairs (in this package)

| Tool | Source → Target |
|------|-----------------|
| crust | C → Rust |
| oxidizer | Go → Rust |
| alphatrans | Java → Python |
| skel | Python → JavaScript |
