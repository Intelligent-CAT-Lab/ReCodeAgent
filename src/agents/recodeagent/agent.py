"""
RecodeAgent for code translation between programming languages

This module provides a RecodeAgent class that serves as the base for
the specialized agents in the translation pipeline, and includes a main
entry point for direct execution.

The agent:
1. Takes configuration and handles logging
2. Provides common functionality for all agents in the pipeline
3. Can be invoked directly with configuration parameters
"""

import os
import sys
import uuid
import yaml
import asyncio
import logging
import argparse
import glob
import time
from pathlib import Path
from typing import Dict, Any, Tuple, List, Optional

# ModelUtils is imported in subclasses but not used directly in this base class


class RecodeAgent:
    """
    Base agent class for the recodeagent system.

    This class provides common functionality for all agents in the translation pipeline.

    Attributes:
        configs (Dict): Configuration parameters for the agent
        session_id (str): Unique identifier for this agent session
        logger (logging.Logger): Logger instance for this agent
    """

    def __init__(self, configs: Dict[str, Any]) -> None:
        """
        Initialize the recode agent with configuration.

        Args:
            configs (Dict[str, Any]): Configuration settings
        """
        self.configs = configs
        self.session_id = str(uuid.uuid4())

        # Set up logging
        agent_name = self.__class__.__name__.lower()
        log_dir = Path(f"logs/{agent_name}")
        log_dir.mkdir(parents=True, exist_ok=True)

        log_file = log_dir / f"{self.session_id}.log"

        self.logger = logging.getLogger(f"{agent_name}.{self.session_id}")
        self.logger.setLevel(logging.DEBUG)
        self.logger.propagate = False  # Prevent propagation to parent loggers

        # File handler
        file_handler = logging.FileHandler(log_file)
        file_handler.setLevel(logging.DEBUG)

        # Console handler
        console_handler = logging.StreamHandler()
        console_handler.setLevel(logging.INFO)

        # Formatter
        formatter = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
        file_handler.setFormatter(formatter)
        console_handler.setFormatter(formatter)

        # Add handlers
        self.logger.addHandler(file_handler)
        self.logger.addHandler(console_handler)

        self.logger.info(f"{agent_name} initialized with session ID: {self.session_id}")

    async def run(self, project_details: Dict[str, Any]) -> Tuple[bool, Dict[str, Any]]:
        """
        Run the agent with the given project details.

        This method should be implemented by each subclass.

        Args:
            project_details (Dict[str, Any]): Details about the project to process

        Returns:
            Tuple[bool, Dict[str, Any]]: (success_status, results)
        """
        raise NotImplementedError("Subclasses must implement this method")

    def _rename_log_file(self, old_session_id: str, new_session_id: str, agent_name: str) -> None:
        """
        Rename a log file from using old_session_id to new_session_id in the filename.

        Args:
            old_session_id (str): The original session ID in the filename
            new_session_id (str): The new session ID to use in the filename
            agent_name (str): The name of the agent (used for the log directory)
        """
        # Skip if session IDs are the same
        if old_session_id == new_session_id:
            return

        # Define log file paths
        log_dir = Path(f"logs/{agent_name}")
        original_log_file = log_dir / f"{old_session_id}.log"
        new_log_dir = log_dir / new_session_id
        new_log_file = new_log_dir / f"{new_session_id}.log"

        # Only proceed if the original file exists
        if not original_log_file.exists():
            return

        os.makedirs(new_log_dir, exist_ok=True)

        # Close logger handlers first
        logger = logging.getLogger(f"{agent_name}.{old_session_id}")
        for handler in logger.handlers[:]:
            handler.close()
            logger.removeHandler(handler)

        # Copy the log file and remove original
        import shutil

        shutil.copy2(original_log_file, new_log_file)
        original_log_file.unlink(missing_ok=True)

        # Create a new logger with the new session ID
        new_logger = logging.getLogger(f"{agent_name}.{new_session_id}")
        new_logger.setLevel(logging.DEBUG)
        new_logger.propagate = False

        # Add handlers to the new logger
        file_handler = logging.FileHandler(new_log_file)
        file_handler.setLevel(logging.DEBUG)
        console_handler = logging.StreamHandler()
        console_handler.setLevel(logging.INFO)
        formatter = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
        file_handler.setFormatter(formatter)
        console_handler.setFormatter(formatter)
        new_logger.addHandler(file_handler)
        new_logger.addHandler(console_handler)

        # Log the file rename operation
        new_logger.info(f"Log file renamed from {original_log_file} to {new_log_file}")


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


async def run_agents(
    config: Dict[str, Any], logger: logging.Logger, skip_agents: List[str] = None, only_agents: List[str] = None
) -> bool:
    """
    Run the agents in sequence.

    Args:
        config (Dict[str, Any]): The configuration for the agents
        logger (logging.Logger): The logger to use
        skip_agents (List[str], optional): List of agents to skip
        only_agents (List[str], optional): List of agents to run (ignores skip_agents if provided)

    Returns:
        bool: True if all agents ran successfully, False otherwise
    """
    from src.agents.recodeagent.analyzer.agent import AnalyzerAgent
    from src.agents.recodeagent.planning.agent import PlanningAgent
    from src.agents.recodeagent.translator.agent import TranslatorAgent
    from src.agents.recodeagent.validator.agent import ValidatorAgent

    logger.info("Starting RecodeAgent orchestrator")

    skip_agents = skip_agents or []
    only_agents = only_agents or []

    # Project details to pass to agents
    project_details = {
        "project_name": config.get("project_name", "unknown"),
        "source_project_root": config.get("source_project_root", "./c/"),
        "target_translation_root": config.get("target_translation_root", "./rust/"),
        "planning_dir": config.get("planning_dir", "./planning/"),
    }

    # Ensure the planning directory exists
    planning_dir = Path(project_details["planning_dir"])
    planning_dir.mkdir(exist_ok=True, parents=True)

    logger.info(f"Project details: {project_details}")

    # Run analyzer agent if not skipped
    if (not only_agents or "analyzer" in only_agents) and "analyzer" not in skip_agents:
        logger.info("Running Analyzer Agent")
        analyzer_agent = AnalyzerAgent(config)

        # Start timing the execution
        start_time = time.time()
        analyzer_success, analyzer_results = await analyzer_agent.run(project_details)
        # Record execution time in seconds
        execution_time = time.time() - start_time

        if not analyzer_success:
            logger.error("Analyzer Agent failed. Aborting.")
            return False

        logger.info(f"Analyzer Agent completed successfully in {execution_time:.2f} seconds")

        # Add execution time to the results dictionary
        analyzer_results["execution_time_seconds"] = execution_time
        project_details["analyzer_results"] = analyzer_results
    else:
        logger.info("Skipping Analyzer Agent")

    # Run planning agent if not skipped
    if (not only_agents or "planning" in only_agents) and "planning" not in skip_agents:
        logger.info("Running Planning Agent")
        planning_agent = PlanningAgent(config)

        # Start timing the execution
        start_time = time.time()
        planning_success, planning_results = await planning_agent.run(project_details)
        # Record execution time in seconds
        execution_time = time.time() - start_time

        if not planning_success:
            logger.error("Planning Agent failed. Aborting.")
            return False

        logger.info(f"Planning Agent completed successfully in {execution_time:.2f} seconds")

        # Add execution time to the results dictionary
        planning_results["execution_time_seconds"] = execution_time
        project_details["planning_results"] = planning_results
    else:
        logger.info("Skipping Planning Agent")

    # Run translator-validator loop if not skipped
    # The loop continues until validator reports no issues
    run_translator = (not only_agents or "translator" in only_agents) and "translator" not in skip_agents
    run_validator = (not only_agents or "validator" in only_agents) and "validator" not in skip_agents

    if run_translator or run_validator:
        max_iterations = config.get("max_translation_validation_iterations", 5)
        iteration = 0
        validation_passed = False

        # Track cumulative execution times
        total_translator_time = 0.0
        total_validator_time = 0.0
        iteration_results = []

        while iteration < max_iterations and not validation_passed:
            iteration += 1
            logger.info(f"=== Translation-Validation Loop: Iteration {iteration}/{max_iterations} ===")

            iteration_result = {"iteration": iteration}

            # Run translator agent
            if run_translator:
                logger.info(f"Running Translator Agent (Iteration {iteration})")
                translator_agent = TranslatorAgent(config)

                # Start timing the execution
                start_time = time.time()
                translator_success, translator_results = await translator_agent.run(project_details)
                # Record execution time in seconds
                execution_time = time.time() - start_time
                total_translator_time += execution_time

                if not translator_success:
                    logger.error("Translator Agent failed. Aborting.")
                    return False

                logger.info(f"Translator Agent completed in {execution_time:.2f} seconds")
                iteration_result["translator_time"] = execution_time
                iteration_result["translator_results"] = translator_results

            # Run validator agent
            if run_validator:
                logger.info(f"Running Validator Agent (Iteration {iteration})")
                validator_agent = ValidatorAgent(config)

                # Start timing the execution
                start_time = time.time()
                validator_success, validator_results = await validator_agent.run(project_details)
                # Record execution time in seconds
                execution_time = time.time() - start_time
                total_validator_time += execution_time

                if not validator_success:
                    logger.error("Validator Agent failed. Aborting.")
                    return False

                logger.info(f"Validator Agent completed in {execution_time:.2f} seconds")
                iteration_result["validator_time"] = execution_time
                iteration_result["validator_results"] = validator_results

                # Check if validation passed by looking for validation-report.md
                # If the file doesn't exist or has PASS status, validation is complete
                validation_report_path = Path(project_details["planning_dir"]) / "validation-report.md"
                validation_summary_path = Path(project_details["planning_dir"]) / "validation-summary.md"

                if validation_summary_path.exists():
                    # Validator created summary, meaning validation passed
                    logger.info("Validation summary found - validation passed!")
                    validation_passed = True
                elif validation_report_path.exists():
                    # Check if report indicates PASS or FAIL
                    try:
                        with open(validation_report_path, "r") as f:
                            report_content = f.read()
                        if "## Status: PASS" in report_content:
                            logger.info("Validation report shows PASS status")
                            validation_passed = True
                        else:
                            logger.info("Validation report shows FAIL status - issues need repair")
                            logger.info("Continuing to next iteration for repairs...")
                    except Exception as e:
                        logger.warning(f"Could not read validation report: {e}")
                        # Assume there are issues if we can't read the report
                else:
                    # No validation report means either first run or all issues resolved
                    # Check if this is not the first iteration
                    if iteration > 1:
                        logger.info("No validation report found after iteration - assuming passed")
                        validation_passed = True
            else:
                # If validator is skipped, exit loop after one translator run
                validation_passed = True

            iteration_results.append(iteration_result)

        if not validation_passed:
            logger.warning(f"Translation-validation loop did not converge after {max_iterations} iterations")
        else:
            logger.info(f"Translation-validation loop completed successfully in {iteration} iteration(s)")

        # Store final results
        project_details["translator_results"] = {
            "execution_time_seconds": total_translator_time,
            "iterations": iteration,
            "iteration_results": iteration_results,
        }
        project_details["validator_results"] = {
            "execution_time_seconds": total_validator_time,
            "iterations": iteration,
            "validation_passed": validation_passed,
            "iteration_results": iteration_results,
        }

        logger.info(f"Total Translator time: {total_translator_time:.2f} seconds")
        logger.info(f"Total Validator time: {total_validator_time:.2f} seconds")
    else:
        logger.info("Skipping Translator and Validator Agents")

    # Save the final project details for reference
    try:
        with open(planning_dir / "project_details.json", "w") as f:
            import json

            json.dump(project_details, f, indent=2)
    except Exception as e:
        logger.error(f"Failed to save project details: {e}")

    logger.info("RecodeAgent orchestrator completed successfully")
    return True
