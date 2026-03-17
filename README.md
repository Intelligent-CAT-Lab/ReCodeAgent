# ReCodeAgent: A Multi-Agent Workflow for Language-Agnostic Repository-Level Code Translation

ReCodeAgent is a language-agnostic framework for autonomous repository-level code translation and validation. ReCodeAgent leverages static analysis combined with Large Language Model (LLM) agents to perform high-quality code translation across multiple programming languages efficiently. To simplify the translation task, ReCodeAgent employs a multi-agent pipeline approach with specialized agents (Analyzer, Planner, Translator, Validator) that work together to understand source code structure, plan the translation strategy, implement the translation, and validate functional equivalence.

## Docker Container

For re-running and evaluating ReCodeAgent on more projects, we recommend using our [`Dockerfile`](./docker-env/Dockerfile) to build a docker image. All required dependencies are installed during `docker build`, making it easier for users to interact with ReCodeAgent.

To build a new image from scratch and run it inside a container:

```bash
bash scripts/start_docker.sh <agent_name> <tool_name> <project_name> <source_language> <target_language>
bash docker_shell.sh <agent_name>.<tool_name>.<project_name>.<source_language>.<target_language>
```

## Zenodo

We provide the results of ReCodeAgent from our experiments on [Zenodo](https://zenodo.org):

- `results.zip`: Translations of ReCodeAgent for all projects, agent trajectories and graphectory analysis.
- `results.xlsx`: More detailed results of ReCodeAgent, including per tool and project results.

## Credentials

### Claude Code

The main experiments in ReCodeAgent use [Claude Code](https://github.com/anthropics/claude-code) as its LLM agent powered by [Claude Sonnet](https://www.anthropic.com/claude/sonnet) model. We use [AWS Amazon Bedrock](https://aws.amazon.com/bedrock/) as provider to interact with the Claude model. To reproduce our results, you are required to make sure your AWS account has Claude model enabled in your desired region. Please configure your credentials by running `aws configure` and pasting your `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `region`, and `model` information. You might need to install the [`awscli`](https://github.com/aws/aws-cli) package if you haven't already.

This assumes you have enabled access to Claude Sonnet model in your chosen region. Please see [AWS Amazon Bedrock](https://aws.amazon.com/bedrock/) for more information on how to get AWS credentials and enable model access.

### Alternative LLM Agents

ReCodeAgent can be configured with other LLM agents and models. Please configure your agent in [`src/utils/model_utils.py`](./src/utils/model_utils.py) and [`src/utils/cmd_utils.py`](./src/utils/cmd_utils.py).

## Project Structure

ReCodeAgent supports multiple translation tools and language pairs:

- **crust**: C to Rust translation
- **alphatrans**: Java to Python translation
- **skel**: Python to JavaScript translation
- **oxidizer**: Go to Rust translation

The project is organized as follows:

```
recodeagent/
├── .claude/                    # Claude Code configuration
├── configs/                    # Configuration files and prompt templatesfor experiments
├── data/
│   └── tool_projects/          # Source and target projects organized by tool
├── docker-env/                 # Docker build and configuration scripts
├── results/                    # Experiment results and trajectories
├── scripts/                    # Shell scripts for running experiments
├── src/
│   ├── agents/                 # Agent implementations
│   │   ├── recodeagent/        # Multi-agent pipeline
│   │   └── baseagent/          # Baseline agent
│   ├── analysis/               # Result analysis tools
│   ├── mcp/                    # MCP servers for language analysis
│   └── utils/                  # Utility functions
├── CLAUDE.md                   # Claude Code memory
└── README.md
```

## Reproducing Results

### RQ1

```bash
bash scripts/start_docker.sh <agent_name> <tool_name> <project_name> <source_language> <target_language>
```

After translation and validation is complete, you can run tests in the target PL to reproduce RQ1 results. Please refer to the `results.zip` file we provide for ReCodeAgent translations.

### RQ2

### RQ3

### RQ4

## Building on ReCodeAgent

### Evaluating More Projects

You can evaluate ReCodeAgent on other projects. Please make sure you have:

- Source projects stored under `data/tool_projects/<tool>/<project>/<source_language>/`
- Appropriate build systems (Makefile, Maven, npm, Cargo, etc.)

### Extending Supported Languages

If you need to experiment with other programming languages not supported by ReCodeAgent:

1. Add MCP language server configuration in `configs/claude_mcp_config.json` under [`Dockerfile`](./docker-env/Dockerfile)
2. Update the Dockerfile to install language-specific tools and compilers
3. Add language-specific analysis tools if needed
4. Update prompt templates in `configs/prompt_templates.yaml`

### MCP Servers

ReCodeAgent uses several MCP servers.

- **Language Servers**: For each supported language (rust-analyzer, gopls, pylsp, jdtls, clangd, typescript-language-server)
- **project-analyzer**: Custom MCP server for analyzing project structure
- **crust-test-runner**: Custom MCP server for running C tests against Rust translations in CRUST tool

To add more MCP servers:

1. Implement the server in `src/mcp/`
2. Configure it in `configs/claude_mcp_config.json` under [`Dockerfile`](./docker-env/Dockerfile)
3. Update the Dockerfile if system dependencies are needed
4. Update agent prompts to utilize the new server capabilities

## Contact

For questions or issues, please open an issue on our GitHub repository.
