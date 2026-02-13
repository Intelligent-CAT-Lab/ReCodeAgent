#!/usr/bin/env python3
"""
Run script for RecodeAgent

This script parses a configuration file and initializes the appropriate agent.
It is designed to be used with the scripts/run.sh shell script.

Usage:
    python src/run.py --config_file=configs/crust/recodeagent_2dpartint_c_rust.yaml
"""

import os
import sys
import yaml
import asyncio
import argparse
import logging
import time
from pathlib import Path
from typing import Dict, Any, Optional


def setup_logging(log_level: str = "INFO") -> logging.Logger:
    """
    Set up logging for the main orchestrator.

    Args:
        log_level (str): The logging level to use (DEBUG, INFO, WARNING, ERROR)

    Returns:
        logging.Logger: The configured logger
    """
    log_dir = Path("logs")
    log_dir.mkdir(exist_ok=True)

    logger = logging.getLogger("orchestrator")
    logger.setLevel(getattr(logging, log_level))
    logger.propagate = False

    # File handler
    file_handler = logging.FileHandler(log_dir / "orchestrator.log")
    file_handler.setLevel(logging.DEBUG)

    # Console handler
    console_handler = logging.StreamHandler()
    console_handler.setLevel(getattr(logging, log_level))

    # Formatter
    formatter = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
    file_handler.setFormatter(formatter)
    console_handler.setFormatter(formatter)

    # Add handlers
    logger.addHandler(file_handler)
    logger.addHandler(console_handler)

    return logger


async def run_agent(config_file: str, log_level: str = "INFO") -> int:
    """
    Run the agent with the specified configuration file.

    Args:
        config_file (str): Path to the configuration file
        log_level (str): Logging level (DEBUG, INFO, WARNING, ERROR)

    Returns:
        int: Exit code (0 for success, non-zero for failure)
    """
    logger = setup_logging(log_level)
    logger.info(f"Starting RecodeAgent with configuration file: {config_file}")

    try:
        # Load configuration from YAML file
        with open(config_file, "r") as f:
            config = yaml.safe_load(f)
            logger.info(f"Loaded configuration from {config_file}")
    except Exception as e:
        logger.error(f"Failed to load configuration: {e}")
        return 1

    # Extract basic configuration for logging
    agent_name = config.get("agent_name", "recodeagent")
    project_name = config.get("project_name", "unknown")
    source_language = config.get("source_language", "unknown")
    target_language = config.get("target_language", "unknown")

    logger.info(f"Agent: {agent_name}")
    logger.info(f"Project: {project_name}")
    logger.info(f"Source language: {source_language}")
    logger.info(f"Target language: {target_language}")

    # Create necessary directories based on configuration
    source_project_root = config.get("source_project_root", "./c/")
    target_translation_root = config.get("target_translation_root", "./rust/")
    planning_dir = config.get("planning_dir", "./planning/")

    for directory in [source_project_root, target_translation_root, planning_dir, "logs"]:
        try:
            Path(directory).mkdir(parents=True, exist_ok=True)
            logger.info(f"Ensured directory exists: {directory}")
        except Exception as e:
            logger.warning(f"Failed to create directory {directory}: {e}")

    # Import and run the appropriate agent
    try:
        if agent_name in ("baseagent-concat", "baseagent-condensed"):
            from src.agents.baseagent.agent import BaseAgent

            project_details = {
                "project_name": project_name,
                "source_project_root": source_project_root,
                "target_translation_root": target_translation_root,
                "planning_dir": planning_dir,
            }
            logger.info(f"Project details: {project_details}")
            logger.info(f"Running {agent_name}")

            base_agent = BaseAgent(config)
            start_time = time.time()
            success, baseagent_results = await base_agent.run(project_details)
            execution_time = baseagent_results.get("execution_time_seconds", time.time() - start_time)

            if success:
                logger.info(f"{agent_name} completed successfully in {execution_time:.2f} seconds")
            else:
                logger.error(f"{agent_name} failed after {execution_time:.2f} seconds")
        else:
            from src.agents.recodeagent.agent import run_agents

            skip_agents = config.get("skip_agents", [])
            only_agents = config.get("only_agents", [])

            success = await run_agents(config=config, logger=logger, skip_agents=skip_agents, only_agents=only_agents)

        if success:
            logger.info(f"{agent_name} completed successfully")
            return 0
        else:
            logger.error(f"{agent_name} failed")
            return 1

    except Exception as e:
        logger.error(f"Unexpected error running RecodeAgent: {e}")
        import traceback

        logger.error(traceback.format_exc())
        return 1


def main():
    """
    Main entry point for the run script.
    """
    parser = argparse.ArgumentParser(description="Run the RecodeAgent with a configuration file.")
    parser.add_argument("--config_file", type=str, required=True, help="Path to the configuration YAML file")
    parser.add_argument(
        "--log_level", type=str, default="INFO", choices=["DEBUG", "INFO", "WARNING", "ERROR"], help="Logging level"
    )

    args = parser.parse_args()

    # Run the agent with the configuration file
    exit_code = asyncio.run(run_agent(args.config_file, args.log_level))

    # Exit with the appropriate exit code
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
