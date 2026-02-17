"""
Translator Agent for the RecodeAgent system

This module provides the TranslatorAgent class that executes the implementation plan
created by the planning agent to translate source code between programming languages.

The translator is responsible for:
1. Translating ALL functions from source to target language
2. Translating ALL tests from source to target language
3. Executing tests to ensure they pass
4. Repairing issues identified in validation reports (in the translation-validation loop)
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


class TranslatorAgent(RecodeAgent):
    """
    Agent that translates source code between programming languages.

    This agent is responsible for:
    1. Executing the implementation plan (translating functions and tests)
    2. Ensuring the translated code compiles and passes tests
    3. Repairing issues identified by the Validator agent

    The agent participates in a translation-validation loop:
    - Translator translates code
    - Validator checks for issues and writes a report
    - Translator repairs issues based on the report
    - Loop continues until no issues remain

    Attributes:
        Inherits all attributes from RecodeAgent
    """

    def __init__(self, configs: Dict[str, Any]) -> None:
        """
        Initialize the translator agent with configuration.

        Args:
            configs (Dict[str, Any]): Configuration settings
        """
        super().__init__(configs)
        self.logger.info("TranslatorAgent initialized")

    def _get_validation_feedback(self, planning_dir: str) -> str:
        """
        Read the validation report if it exists and return its content as feedback.

        Args:
            planning_dir (str): Path to the planning directory

        Returns:
            str: The validation report content, or empty string if no report exists
        """
        validation_report_path = Path(planning_dir) / "validation-report.md"

        if validation_report_path.exists():
            try:
                with open(validation_report_path, "r") as f:
                    report_content = f.read()

                # Check if this is a FAIL report (has issues to fix)
                if "## Status: FAIL" in report_content or "Total issues found:" in report_content:
                    self.logger.info("Found validation report with issues - will include as feedback")
                    return f"""
## VALIDATION FEEDBACK - ISSUES TO REPAIR

The Validator agent has identified the following issues that MUST be repaired:

{report_content}

**IMPORTANT:** You MUST address ALL issues listed above before proceeding.
After fixing all issues, delete {validation_report_path} to signal completion.
"""
            except Exception as e:
                self.logger.warning(f"Could not read validation report: {e}")

        return ""

    async def run(self, project_details: Dict[str, Any]) -> Tuple[bool, Dict[str, Any]]:
        """
        Run the translator agent to translate source code.

        Args:
            project_details (Dict[str, Any]): Details about the project to translate
                Must contain:
                - source_project_root: Path to the source project root
                - target_translation_root: Path to the target translation root
                - planning_dir: Path with planning documents

        Returns:
            Tuple[bool, Dict[str, Any]]: (success_status, results)
                - success_status: True if translation was successfully completed
                - results: The translation results including paths to created files
        """
        self.logger.info(f"Starting translation for project: {project_details.get('project_name', 'unknown')}")

        # Generate the prompt for the translator
        prompt_generator = PromptGenerator(
            configs=self.configs, project_details=project_details, agent_type="translator"
        )
        prompt = prompt_generator.generate_prompt()

        # Check for validation feedback (issues from previous validation that need repair)
        planning_dir = project_details.get("planning_dir", "./planning/")
        validation_feedback = self._get_validation_feedback(planning_dir)

        self.logger.debug("Generated prompt:")
        self.logger.debug(prompt)

        if validation_feedback:
            self.logger.info("Including validation feedback in prompt")
            self.logger.debug("Validation feedback:")
            self.logger.debug(validation_feedback)

        try:
            # Execute the model
            self.logger.info("Executing translator agent with Claude")
            model_utils = ModelUtils(configs=self.configs, logger=self.logger)
            status, agent_output = await model_utils.prompt_agent(
                prompt=prompt,
                feedback=validation_feedback,
                agent_name="translator",
                timeout=self.configs["translator_timeout"],
            )

            # Process the result
            if not status:
                self.logger.error("Translator agent execution failed")
                return False, {"error": "Translator agent execution failed"}

            # Extract the final response
            result = agent_output.get("result", "")
            if not result:
                if "last_json" in agent_output and "result" in agent_output["last_json"]:
                    result = agent_output["last_json"]["result"]
                else:
                    self.logger.error("No result found in agent output")
                    return False, {"error": "No result found in agent output"}

            # Log the final status
            self.logger.info("Translation completed successfully")

            # Generate a session ID for logs
            final_session_id = f"translator.{project_details.get('project_name', 'unknown')}"
            self._rename_log_file(self.session_id, final_session_id, "translator")

            return True, {"agent_output": agent_output}

        except Exception as e:
            self.logger.error(f"Error during translator agent execution: {str(e)}")
            return False, {"error": str(e)}
