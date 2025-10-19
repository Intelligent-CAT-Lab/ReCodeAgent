"""
Validator Agent for the RecodeAgent system

This module provides the ValidatorAgent class that validates the translated code by
generating tests and comparing the behavior between C and Rust implementations.
"""

import os
import json
import asyncio
import re
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple

from src.agents.recodeagent.agent import RecodeAgent
from src.agents.recodeagent.prompt_generator import PromptGenerator
from src.utils.model_utils import ModelUtils


class ValidatorAgent(RecodeAgent):
    """
    Agent that validates translated code.

    This agent is responsible for:
    1. Generating tests for both C and Rust implementations
    2. Running the tests to compare behavior
    3. Identifying discrepancies between implementations
    4. Suggesting fixes for functional equivalence issues

    Attributes:
        Inherits all attributes from RecodeAgent
    """

    def __init__(self, configs: Dict[str, Any]) -> None:
        """
        Initialize the validator agent with configuration.

        Args:
            configs (Dict[str, Any]): Configuration settings
        """
        super().__init__(configs)
        self.logger.info("ValidatorAgent initialized")

    async def run(self, project_details: Dict[str, Any]) -> Tuple[bool, Dict[str, Any]]:
        """
        Run the validator agent to validate translated code.

        Args:
            project_details (Dict[str, Any]): Details about the project to validate
                Must contain:
                - c_project_root: Path to the C project root
                - rust_translation_root: Path to the Rust translation root
                - planning_dir: Path with planning documents

        Returns:
            Tuple[bool, Dict[str, Any]]: (success_status, results)
                - success_status: True if validation was successfully completed
                - results: The validation results including test outcomes and fixes
        """
        self.logger.info(f"Starting validation for project: {project_details.get('project_name', 'unknown')}")

        # Generate the prompt for the validator
        prompt_generator = PromptGenerator(
            configs=self.configs, project_details=project_details, agent_type="validator"
        )
        prompt = prompt_generator.generate_prompt()

        self.logger.debug("Generated prompt:")
        self.logger.debug(prompt)

        try:
            # Execute the model
            self.logger.info("Executing validator agent with Claude")
            model_utils = ModelUtils(configs=self.configs, logger=self.logger)
            status, agent_output = await model_utils.prompt_agent(
                prompt=prompt,
                feedback="",
                agent_name="validator",
                timeout=1000,  # 5 minutes timeout
            )

            # Process the result
            if not status:
                self.logger.error("Validator agent execution failed")
                return False, {"error": "Validator agent execution failed"}

            # Extract the final response
            result = agent_output.get("result", "")
            if not result:
                if "last_json" in agent_output and "result" in agent_output["last_json"]:
                    result = agent_output["last_json"]["result"]
                else:
                    self.logger.error("No result found in agent output")
                    return False, {"error": "No result found in agent output"}

            # Log the final status
            self.logger.info("Validation completed successfully")

            # Generate a session ID for logs
            final_session_id = f"validator.{project_details.get('project_name', 'unknown')}"
            self._rename_log_file(self.session_id, final_session_id, "validator")

            return True, {"agent_output": agent_output}

        except Exception as e:
            self.logger.error(f"Error during validator agent execution: {str(e)}")
            return False, {"error": str(e)}
