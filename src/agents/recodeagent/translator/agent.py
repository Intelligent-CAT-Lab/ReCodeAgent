"""
Translator Agent for the RecodeAgent system

This module provides the TranslatorAgent class that executes the implementation plan
created by the planning agent to translate source code from C to Rust.
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
    Agent that translates source code from C to Rust.

    This agent is responsible for:
    1. Executing the implementation plan
    2. Translating C functions and methods to Rust
    3. Creating project skeleton and structure
    4. Ensuring the translated code compiles and passes tests

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

    async def run(self, project_details: Dict[str, Any]) -> Tuple[bool, Dict[str, Any]]:
        """
        Run the translator agent to translate source code.

        Args:
            project_details (Dict[str, Any]): Details about the project to translate
                Must contain:
                - c_project_root: Path to the C project root
                - rust_translation_root: Path to the Rust translation root
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

        self.logger.debug("Generated prompt:")
        self.logger.debug(prompt)

        try:
            # Execute the model
            self.logger.info("Executing translator agent with Claude")
            model_utils = ModelUtils(configs=self.configs, logger=self.logger)
            status, agent_output = await model_utils.prompt_agent(
                prompt=prompt,
                feedback="",
                agent_name="translator",
                timeout=1000,  # 10 minutes timeout - translation takes longer
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
