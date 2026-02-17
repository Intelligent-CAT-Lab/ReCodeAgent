"""
BaseAgent for code translation between programming languages

This module provides a BaseAgent class that performs the entire translation
workflow using a single agent. Supports two variants:
- baseagent-concat: Uses concatenation of all RecodeAgent subagent prompts
- baseagent-condensed: Uses a dedicated condensed prompt template
"""

import time
import uuid
import logging
import json
from pathlib import Path
from typing import Dict, Any, Tuple

from src.agents.recodeagent.agent import RecodeAgent
from src.agents.baseagent.prompt_generator import BasePromptGenerator
from src.utils.model_utils import ModelUtils


class BaseAgent(RecodeAgent):
    """
    Single agent that performs the entire translation workflow.

    Unlike RecodeAgent which uses 4 specialized agents (analyzer, planning,
    translator, validator), BaseAgent uses one agent with either:
    - baseagent-concat: Concatenated prompt with all subagent instructions
    - baseagent-condensed: Condensed single-prompt approach

    The agent is responsible for:
    1. Analyzing the source project and creating high-level design
    2. Creating the translation plan
    3. Translating code and tests
    4. Validating and repairing issues

    Attributes:
        Inherits all attributes from RecodeAgent
    """

    def __init__(self, configs: Dict[str, Any]) -> None:
        """
        Initialize the base agent with configuration.

        Args:
            configs (Dict[str, Any]): Configuration settings
        """
        # Set configs and session_id (like RecodeAgent does)
        self.configs = configs
        self.session_id = str(uuid.uuid4())

        # Use the agent variant name from config (e.g., "baseagent-concat" or "baseagent-condensed")
        # instead of the class name
        agent_name = self.configs.get("agent_name", "baseagent-condensed")
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
        Run the base agent to perform the entire translation workflow.

        Args:
            project_details (Dict[str, Any]): Details about the project
                Must contain:
                - source_project_root: Path to the source project root
                - target_translation_root: Path to the target translation root
                - planning_dir: Path to store planning documents
                - project_name: Name of the project

        Returns:
            Tuple[bool, Dict[str, Any]]: (success_status, results)
                - success_status: True if translation was successfully completed
                - results: The agent output and execution details
        """
        agent_name = self.configs.get("agent_name", "baseagent-condensed")
        start_time = time.time()
        self.logger.info(f"Starting translation for project: {project_details.get('project_name', 'unknown')}")

        # Generate the prompt (either concatenated or condensed based on variant)
        prompt_generator = BasePromptGenerator(configs=self.configs, project_details=project_details)
        prompt = prompt_generator.generate_prompt()

        self.logger.debug("Generated prompt:")
        self.logger.debug(prompt)

        try:
            # Execute the single agent with the full concatenated prompt
            self.logger.info(f"Executing {agent_name} with Claude")
            model_utils = ModelUtils(configs=self.configs, logger=self.logger)
            status, agent_output = await model_utils.prompt_agent(
                prompt=prompt,
                feedback="",
                agent_name=agent_name,
                timeout=self.configs["baseagent_timeout"],
            )

            if not status:
                execution_time = time.time() - start_time
                self.logger.error(f"{agent_name} execution failed")
                return False, {"error": f"{agent_name} execution failed", "execution_time_seconds": execution_time}

            result = agent_output.get("result", "")
            if not result:
                if "last_json" in agent_output and "result" in agent_output["last_json"]:
                    result = agent_output["last_json"]["result"]
                else:
                    execution_time = time.time() - start_time
                    self.logger.error("No result found in agent output")
                    return False, {"error": "No result found in agent output", "execution_time_seconds": execution_time}

            execution_time = time.time() - start_time
            self.logger.info(f"{agent_name} completed successfully")

            # Save the final project details for reference (same pattern as recodeagent)
            try:
                planning_dir = Path(project_details.get("planning_dir", "./planning/"))
                planning_dir.mkdir(exist_ok=True, parents=True)
                project_details["baseagent_results"] = {
                    "agent_output": agent_output,
                    "execution_time_seconds": execution_time,
                }
                with open(planning_dir / "project_details.json", "w") as f:
                    json.dump(project_details, f, indent=2)
            except Exception as e:
                self.logger.error(f"Failed to save project details: {e}")

            return True, {"agent_output": agent_output, "execution_time_seconds": execution_time}

        except Exception as e:
            execution_time = time.time() - start_time
            self.logger.error(f"Error during {agent_name} execution: {str(e)}")
            return False, {"error": str(e), "execution_time_seconds": execution_time}
