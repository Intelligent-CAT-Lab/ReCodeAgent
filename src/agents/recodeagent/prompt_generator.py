"""
Prompt Generator for the RecodeAgent system

This module provides a prompt generator for all agents in the RecodeAgent system.
The PromptGenerator can generate prompts for any of the specialized agents:
- Analyzer
- Planning
- Translator
- Validator

Each agent's prompt is based on a template defined in configs/prompt_templates.yaml.
"""

import yaml
from typing import Dict, Any
from jinja2 import Template


class PromptGenerator:
    """
    Prompt generator for all agents in the RecodeAgent system.

    This class generates prompts for all agents based on templates
    defined in the prompt_templates.yaml configuration file.

    Attributes:
        configs (Dict[str, Any]): Configuration settings
        project_details (Dict[str, Any]): Details about the project
        prompt_templates (Dict[str, Any]): Loaded prompt templates from YAML
        agent_type (str): The type of agent (analyzer, planning, translator, validator)
    """

    def __init__(self, configs: Dict[str, Any], project_details: Dict[str, Any], agent_type: str) -> None:
        """
        Initialize the prompt generator.

        Args:
            configs (Dict[str, Any]): Configuration settings
            project_details (Dict[str, Any]): Details about the project
            agent_type (str): The type of agent (analyzer, planning, translator, validator)
        """
        self.configs = configs
        self.project_details = project_details
        self.agent_type = agent_type

        # Load prompt templates
        with open("configs/prompt_templates.yaml", "r") as file:
            self.prompt_templates = yaml.safe_load(file)

        # Format project details for template rendering
        self.format_project_details()

        self.prompt = ""

    def generate_prompt(self) -> str:
        """
        Generate a prompt based on the agent type, configuration, and project details.

        Returns:
            str: The generated prompt
        """
        # Get the appropriate template for the agent type
        template_content = self.prompt_templates["templates"]["recodeagent"][self.agent_type]

        # Create a template object
        template = Template(template_content)

        # Create the context with appropriate variables based on agent type
        context = {
            "source_project_root": self.source_project_root,
            "target_translation_root": self.target_translation_root,
            "planning_dir": self.planning_dir,
            "source_language": self.configs["source_language"],
            "target_language": self.configs["target_language"],
        }

        # Render the template with the context
        self.prompt = template.render(context)
        return self.prompt

    def format_project_details(self) -> None:
        """
        Format and prepare project details for use in the prompt.
        """
        # Extract and format common details used by all agents
        self.source_project_root = self.project_details.get("source_project_root", "./c/")
        self.target_translation_root = self.project_details.get("target_translation_root", "./rust/")
        self.planning_dir = self.project_details.get("planning_dir", "./planning/")
