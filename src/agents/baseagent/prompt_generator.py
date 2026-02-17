"""
Prompt Generator for the BaseAgent system

This module provides a prompt generator that handles two baseagent variants:
- baseagent-concat: Concatenates recodeagent subagent templates (analyzer,
  planning, translator, validator) into a single comprehensive prompt
- baseagent-condensed: Uses the dedicated condensed template from
  configs/prompt_templates.yaml for a streamlined single-agent approach
"""

import yaml
from typing import Dict, Any
from jinja2 import Template


class BasePromptGenerator:
    """
    Prompt generator for BaseAgent supporting concat and condensed variants.

    For baseagent-concat: Concatenates all recodeagent subagent templates
    (analyzer, planning, translator, validator) into a single unified prompt.

    For baseagent-condensed: Uses the dedicated condensed template from
    configs/prompt_templates.yaml for a streamlined approach.

    Attributes:
        configs (Dict[str, Any]): Configuration settings
        project_details (Dict[str, Any]): Details about the project
        prompt_templates (Dict[str, Any]): Loaded prompt templates from YAML
    """

    def __init__(self, configs: Dict[str, Any], project_details: Dict[str, Any]) -> None:
        """
        Initialize the base prompt generator.

        Args:
            configs (Dict[str, Any]): Configuration settings
            project_details (Dict[str, Any]): Details about the project
        """
        self.configs = configs
        self.project_details = project_details

        # Load prompt templates
        with open("configs/prompt_templates.yaml", "r") as file:
            self.prompt_templates = yaml.safe_load(file)

        # Format project details for template rendering
        self._format_project_details()

        self.prompt = ""

    def generate_prompt(self) -> str:
        """
        Generate the baseagent prompt from the template.

        For baseagent-concat: Concatenates recodeagent subagent templates
        For baseagent-condensed: Uses the dedicated condensed template

        Returns:
            str: The rendered baseagent prompt
        """
        agent_name = self.configs.get("agent_name", "baseagent-condensed")
        order = {"analyzer": "FIRST", "planning": "SECOND", "translator": "THIRD", "validator": "FOURTH"}

        if agent_name == "baseagent-concat":
            # Concatenate recodeagent subagent templates
            template_parts = []
            for subagent in order.keys():
                template_content = self.prompt_templates["templates"]["recodeagent"][subagent]
                template_parts.append(
                    80 * "=" + "\nYOU MUST DO THIS STEP " + order[subagent] + ":\n" + 80 * "=" + "\n\n"
                )
                template_parts.append(template_content)

            # Join with separators for clarity
            full_template = (
                "YOU MUST FOLLOW THE ORDER OF STEPS DETERMINED BY THE ORDER VARIABLE: "
                + str(order)
                + "\n\n ONLY TERMINATE WHEN YOU FINISH ALL FOUR AGENTS. DO NOT STOP AFTER ANALYZER, PLANNING, TRANSLATOR AGENTS. YOU MUST ONLY STOP AFTER VALIDATOR AGENT."
            )
            full_template += "\n\n".join(template_parts)
            template = Template(full_template)
        else:  # baseagent-condensed
            template_content = self.prompt_templates["templates"]["baseagent-condensed"]
            template = Template(template_content)

        context = self._get_template_context()
        self.prompt = template.render(context)
        return self.prompt

    def _get_template_context(self) -> Dict[str, Any]:
        """Build the context dict for Jinja2 template rendering."""
        return {
            "source_project_root": self.source_project_root,
            "target_translation_root": self.target_translation_root,
            "planning_dir": self.planning_dir,
            "source_language": self.configs["source_language"],
            "target_language": self.configs["target_language"],
            "translation_requirements": self.translation_requirements,
        }

    def _format_project_details(self) -> None:
        """Format and prepare project details for use in the prompt."""
        self.source_project_root = self.project_details.get("source_project_root", "./c/")
        self.target_translation_root = self.project_details.get("target_translation_root", "./rust/")
        self.planning_dir = self.project_details.get("planning_dir", "./planning/")
        self.translation_requirements = self.configs.get(
            "translation_requirements",
            self.project_details.get("translation_requirements", "1:1 translation with no modifications"),
        )
