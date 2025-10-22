"""
Planning Agent for the RecodeAgent system

This module provides the PlanningAgent class that creates a detailed plan for translating
source code based on the high-level design created by the analyzer agent.
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


class PlanningAgent(RecodeAgent):
    """
    Agent that creates a detailed plan for translating source code.

    This agent is responsible for:
    1. Creating a list of C functions and methods to translate
    2. Creating a call graph for the functions and methods
    3. Creating a step-by-step implementation plan for the translation
    4. Prioritizing the implementation order based on dependencies

    Attributes:
        Inherits all attributes from RecodeAgent
    """

    def __init__(self, configs: Dict[str, Any]) -> None:
        """
        Initialize the planning agent with configuration.

        Args:
            configs (Dict[str, Any]): Configuration settings
        """
        super().__init__(configs)
        self.logger.info("PlanningAgent initialized")

    async def run(self, project_details: Dict[str, Any]) -> Tuple[bool, Dict[str, Any]]:
        """
        Run the planning agent to create a detailed translation plan.

        Args:
            project_details (Dict[str, Any]): Details about the project to plan
                Must contain:
                - c_project_root: Path to the C project root
                - rust_translation_root: Path to the Rust translation root
                - planning_dir: Path to store planning documents

        Returns:
            Tuple[bool, Dict[str, Any]]: (success_status, results)
                - success_status: True if planning was successfully completed
                - results: The planning results including paths to created documents
        """
        self.logger.info(f"Starting planning for project: {project_details.get('project_name', 'unknown')}")

        # Generate the prompt for the planner
        prompt_generator = PromptGenerator(configs=self.configs, project_details=project_details, agent_type="planning")
        prompt = prompt_generator.generate_prompt()

        self.logger.debug("Generated prompt:")
        self.logger.debug(prompt)

        try:
            # Execute the model
            self.logger.info("Executing planning agent with Claude")
            model_utils = ModelUtils(configs=self.configs, logger=self.logger)
            status, agent_output = await model_utils.prompt_agent(
                prompt=prompt,
                feedback="",
                agent_name="planning",
                timeout=self.configs["planning_timeout"],
            )

            # Process the result
            if not status:
                self.logger.error("Planning agent execution failed")
                return False, {"error": "Planning agent execution failed"}

            # Extract the final response
            result = agent_output.get("result", "")
            if not result:
                if "last_json" in agent_output and "result" in agent_output["last_json"]:
                    result = agent_output["last_json"]["result"]
                else:
                    self.logger.error("No result found in agent output")
                    return False, {"error": "No result found in agent output"}

            # Log the final status
            self.logger.info("Planning completed successfully")

            # Generate a session ID for logs
            final_session_id = f"planning.{project_details.get('project_name', 'unknown')}"
            self._rename_log_file(self.session_id, final_session_id, "planning")

            return True, {"agent_output": agent_output}

        except Exception as e:
            self.logger.error(f"Error during planning agent execution: {str(e)}")
            return False, {"error": str(e)}
