"""
Analyzer Agent for the RecodeAgent system

This module provides the AnalyzerAgent class that analyzes the source project structure,
understands dependencies, and creates a high-level design for the translation.
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


class AnalyzerAgent(RecodeAgent):
    """
    Agent that analyzes source code and creates a high-level design for translation.

    This agent is responsible for:
    1. Analyzing the source project structure
    2. Understanding dependencies and requirements
    3. Creating a high-level design for the translation
    4. Creating research documents for future agents

    Attributes:
        Inherits all attributes from RecodeAgent
    """

    def __init__(self, configs: Dict[str, Any]) -> None:
        """
        Initialize the analyzer agent with configuration.

        Args:
            configs (Dict[str, Any]): Configuration settings
        """
        super().__init__(configs)
        self.logger.info("AnalyzerAgent initialized")

    async def run(self, project_details: Dict[str, Any]) -> Tuple[bool, Dict[str, Any]]:
        """
        Run the analyzer agent to analyze source code and create design documents.

        Args:
            project_details (Dict[str, Any]): Details about the project to analyze
                Must contain:
                - c_project_root: Path to the C project root
                - rust_translation_root: Path to the Rust translation root
                - planning_dir: Path to store planning documents
                - translation_requirements: Optional specific requirements for translation

        Returns:
            Tuple[bool, Dict[str, Any]]: (success_status, results)
                - success_status: True if analysis was successfully completed
                - results: The analysis results including paths to created documents
        """
        self.logger.info(f"Starting analysis for project: {project_details.get('project_name', 'unknown')}")

        # Generate the prompt for the analyzer
        prompt_generator = PromptGenerator(configs=self.configs, project_details=project_details, agent_type="analyzer")
        prompt = prompt_generator.generate_prompt()

        self.logger.debug("Generated prompt:")
        self.logger.debug(prompt)

        try:
            # Execute the model
            self.logger.info("Executing analyzer agent with Claude")
            model_utils = ModelUtils(configs=self.configs, logger=self.logger)
            status, agent_output = await model_utils.prompt_agent(
                prompt=prompt,
                feedback="",
                agent_name="analyzer",
                timeout=self.configs["analyzer_timeout"],
            )

            # Process the result
            if not status:
                self.logger.error("Analyzer agent execution failed")
                return False, {"error": "Analyzer agent execution failed"}

            # Extract the final response
            result = agent_output.get("result", "")
            if not result:
                if "last_json" in agent_output and "result" in agent_output["last_json"]:
                    result = agent_output["last_json"]["result"]
                else:
                    self.logger.error("No result found in agent output")
                    return False, {"error": "No result found in agent output"}

            # Log the final status
            self.logger.info("Analysis completed successfully")

            # Generate a session ID for logs
            final_session_id = f"analyzer.{project_details.get('project_name', 'unknown')}"
            self._rename_log_file(self.session_id, final_session_id, "analyzer")

            return True, {"agent_output": agent_output}

        except Exception as e:
            self.logger.error(f"Error during analyzer agent execution: {str(e)}")
            return False, {"error": str(e)}
