"""
Validator Agent for the RecodeAgent system

This module provides the ValidatorAgent class that validates the translated code
and writes a detailed report of any issues found for the Translator to repair.

The validator is responsible for:
1. Checking all stubs have been implemented (no unimplemented!(), pass, etc.)
2. Checking there are no TODO comments remaining
3. Checking functions are functionally equivalent to source
4. Checking tests are translated correctly:
   - Same number of assertions as source
   - Same assertions as source
5. Generating tests for functions without coverage (in both source and target languages)
6. Writing a validation report for issues that need repair

The agent participates in a translation-validation loop:
- Translator translates code
- Validator checks for issues and writes validation-report.md
- If issues found, Translator repairs them
- Loop continues until validation passes (no issues)
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
    Agent that validates translated code and writes validation reports.

    This agent is responsible for:
    1. Checking all stubs have been implemented
    2. Checking there are no TODO comments
    3. Checking functions are functionally equivalent
    4. Checking tests are translated correctly (same assertions)
    5. Generating tests for uncovered functions (in both languages)
    6. Writing validation-report.md with issues for the Translator to repair

    The validation loop:
    - If issues are found: Write validation-report.md with FAIL status
    - If no issues: Write validation-summary.md and delete validation-report.md

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

        The validator checks for:
        - Unimplemented stubs
        - TODO comments
        - Function equivalence issues
        - Test translation issues (assertion count/content mismatches)
        - Coverage gaps (generates tests for uncovered functions)

        If issues are found, writes validation-report.md for the Translator to repair.
        If no issues, writes validation-summary.md to signal completion.

        Args:
            project_details (Dict[str, Any]): Details about the project to validate
                Must contain:
                - source_project_root: Path to the source project root
                - target_translation_root: Path to the target translation root
                - planning_dir: Path with planning documents

        Returns:
            Tuple[bool, Dict[str, Any]]: (success_status, results)
                - success_status: True if validation was successfully completed
                - results: The validation results including issues found and test outcomes
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
                timeout=self.configs["validator_timeout"],
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
