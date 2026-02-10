"""BaseAgent - A single-agent code translation agent using concatenated prompts."""

from src.agents.baseagent.agent import BaseAgent
from src.agents.baseagent.prompt_generator import BasePromptGenerator

__all__ = ["BaseAgent", "BasePromptGenerator"]
