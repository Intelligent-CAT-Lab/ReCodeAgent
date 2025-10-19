import os
import json
import asyncio
import logging
from typing import Dict, Tuple, Any, Optional

from src.utils.cmd_utils import run_claude_code, prompt_claude


class ModelUtils:
    """
    A wrapper class to handle model operations for the Anthropic model.

    This class provides unified interfaces for prompting models,
    abstracting away the implementation details of the API.

    Attributes:
        configs (dict): Configuration settings
        logger (logging.Logger, optional): Logger to use
    """

    def __init__(self, configs: Dict[str, Any] = None, logger: Optional[logging.Logger] = None) -> None:
        """
        Initialize the ModelUtils with configuration.

        Args:
            configs (dict): Configuration settings
            logger (logging.Logger, optional): Logger to use. If None, logs to console only.
        """
        self.configs = configs or {}
        self.logger = logger

        if self.logger:
            self.logger.info(f"Initialized ModelUtils")

    async def prompt_agent(
        self,
        prompt: str,
        feedback: str = "",
        agent_name: str = None,
        sub_agent_name: str = None,
        timeout: int = None,
    ) -> Tuple[bool, Dict]:
        """
        Execute model command via Claude Code API with the given prompt.

        This is a wrapper for run_claude_code.

        Args:
            prompt (str): The prompt to send to the model
            feedback (str): Optional feedback to append to the prompt for retries
            agent_name (str): The name of the agent running the command
            sub_agent_name (str, optional): The name of the sub-agent running the command
            timeout (int, optional): Maximum time in seconds to wait for model's response.
                               If None, no timeout will be applied.

        Returns:
            tuple[bool, dict]: (success_status, captured_output)
                - success_status: True for both normal completions and timeouts
                - captured_output: Dictionary containing model response details

        Raises:
            ValueError: If agent_name is not provided
        """
        return await run_claude_code(
            prompt=prompt,
            feedback=feedback,
            configs=self.configs,
            logger=self.logger,
            agent_name=agent_name,
            sub_agent_name=sub_agent_name,
            timeout=timeout,
        )

    async def prompt_model(
        self,
        prompt: str,
        feedback: str = "",
        agent_name: str = None,
        sub_agent_name: str = None,
    ) -> Tuple[bool, Dict]:
        """
        Execute model via Claude API with the given prompt.

        This is a wrapper for prompt_claude.

        Args:
            prompt (str): The prompt to send to the model
            feedback (str): Optional feedback to append to the prompt for retries
            agent_name (str): The name of the agent running the command
            sub_agent_name (str, optional): The name of the sub-agent running the command

        Returns:
            tuple[bool, dict]: (success_status, parsed_output)
                - success_status: True if command executed successfully and output was valid
                - parsed_output: The parsed output from the model, or None if unsuccessful

        Raises:
            ValueError: If agent_name is not provided
        """
        return await prompt_claude(
            prompt=prompt,
            feedback=feedback,
            configs=self.configs,
            logger=self.logger,
            agent_name=agent_name,
            sub_agent_name=sub_agent_name,
        )
