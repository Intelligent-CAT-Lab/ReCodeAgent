import os
import json
import asyncio
import logging
from typing import Dict, Tuple, Any, Optional


async def run_claude_code(
    prompt: str,
    feedback: str,
    configs: dict,
    logger=None,
    agent_name: str = None,
    sub_agent_name: str = None,
    timeout: int = None,
) -> tuple[bool, dict]:
    """
    Execute Claude CLI command with the given prompt and optional timeout.

    Runs Claude via CLI subprocess with stream-json output format, intercepts and parses
    the streamed JSONs. Utility function that can be imported by different agents.

    Args:
        prompt (str): The prompt to send to Claude
        feedback (str): Optional feedback to append to the prompt for retries
        configs (dict): Configuration settings
        logger (logging.Logger, optional): Logger to use. If None, logs to console only.
        agent_name (str): The name of the agent running the command
        sub_agent_name (str, optional): The name of the sub-agent running the command
        timeout (int, optional): Maximum time in seconds to wait for Claude's response.
                               If None, no timeout will be applied.

    Returns:
        tuple[bool, dict]: (success_status, captured_output)
            - success_status: True for both normal completions and timeouts
            - captured_output: A dictionary containing:
                - For normal completions: first and last intercepted JSONs in 'first_json' and 'last_json'
                - For timeouts or errors: only the first intercepted JSON in 'first_json'

    Raises:
        ValueError: If agent_name is not provided
    """
    if not agent_name:
        raise ValueError("Agent name must be provided to run Claude command")

    if feedback != "" and logger:
        logger.info(f"Feedback provided: {feedback}")
        prompt += f"\\n\\nFeedback: {feedback}"

    process = None
    captured_jsons = []
    try:
        if logger:
            logger.info(f"Executing Claude CLI command{' with timeout: ' + str(timeout) + 's' if timeout else ''}...")
        # Use asyncio.create_subprocess_exec for true async operation
        cmd = ["claude", "-p", prompt]
        if configs.get("mcp_config_file"):
            cmd += ["--mcp-config", configs["mcp_config_file"]]
        if configs.get("extra_agent_args"):
            cmd += configs["extra_agent_args"]
        if agent_name == "translator" and configs.get("extra_translator_agent_args"):
            cmd += configs["extra_translator_agent_args"]
        if agent_name == "validator" and configs.get("extra_validator_agent_args"):
            cmd += configs["extra_validator_agent_args"]

        process = await asyncio.create_subprocess_exec(
            *cmd,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )

        # Process the output stream in fixed-size chunks
        async def process_stream():
            buffer = b""
            chunk_size = 32_768  # 32 KiB
            while True:
                chunk = await process.stdout.read(chunk_size)
                if not chunk:
                    break  # End of stream
                buffer += chunk
                while b"\n" in buffer:
                    line, buffer = buffer.split(b"\n", 1)
                    line_str = line.decode("utf-8").strip()
                    if not line_str:
                        continue
                    try:
                        json_obj = json.loads(line_str)
                        captured_jsons.append(json_obj)
                        if logger:
                            logger.debug(f"Captured JSON: {json_obj}")
                            logger.debug(f"Captured JSON stream: {len(captured_jsons)} items so far")
                    except json.JSONDecodeError as e:
                        if logger:
                            logger.warning(f"Failed to parse stream JSON: {e}")
                            logger.debug(f"Raw line: {line_str}")
                            exit()

        # Run with or without timeout
        if timeout is not None:
            try:
                # Create a task for stream processing
                stream_task = asyncio.create_task(process_stream())
                # Wait for the task to complete with timeout
                await asyncio.wait_for(stream_task, timeout=timeout)
                # Wait for process to finish
                await process.wait()
            except asyncio.TimeoutError:
                # Timeout occurred
                if logger:
                    logger.warning(f"Claude command timed out after {timeout} seconds")

                # Cancel the stream processing task
                stream_task.cancel()
                try:
                    await asyncio.wait_for(stream_task, timeout=1.0)
                except (asyncio.TimeoutError, asyncio.CancelledError):
                    pass

                # Try to terminate the process gracefully
                if process and process.returncode is None:
                    try:
                        process.terminate()
                        # Wait a short time for it to terminate
                        await asyncio.sleep(0.5)
                        # Force kill if still running
                        if process.returncode is None:
                            process.kill()
                    except Exception as e:
                        if logger:
                            logger.error(f"Error terminating subprocess during timeout: {str(e)}")

                # Return the first captured JSON if available with timeout flag
                result = {"timeout": True}  # Add explicit timeout flag
                if captured_jsons:
                    result["first_json"] = captured_jsons[0]
                    if logger:
                        logger.info(
                            f"Returning first captured JSON from {len(captured_jsons)} total received before timeout"
                        )

                # Return with success=True as requested
                return True, result
        else:
            # No timeout, process the entire stream
            await process_stream()
            # Wait for process to finish
            await process.wait()

        # Handle process completion
        if process.returncode != 0:
            stderr_content = await process.stderr.read()
            if logger:
                logger.error(f"Claude failed with exit code {process.returncode}")
                logger.error(f"Error details: {stderr_content.decode(errors='ignore')}")

            # Return the first captured JSON if available, with success=True
            result = {"timeout": False}  # Not a timeout, but process error
            if captured_jsons:
                result["first_json"] = captured_jsons[0]
            return True, result

        # Normal completion
        if logger:
            logger.info(f"Claude command completed successfully. Captured {len(captured_jsons)} JSON objects")

        # Return first and last JSON for normal termination
        result = {"timeout": False}  # Explicitly mark as not timeout
        if captured_jsons:
            result["first_json"] = captured_jsons[0]
            result["last_json"] = captured_jsons[-1]

            # Store the result for compatibility with older code
            if "result" in captured_jsons[-1]:
                result["result"] = captured_jsons[-1]["result"]

        return True, result

    except asyncio.CancelledError:
        # Properly handle task cancellation (e.g., due to timeout)
        if process and process.returncode is None:
            try:
                process.terminate()
                await asyncio.sleep(0.5)
                if process.returncode is None:
                    process.kill()
            except Exception as e:
                if logger:
                    logger.error(f"Error terminating subprocess during cancellation: {str(e)}")
        raise  # Re-raise the CancelledError to be handled by the caller

    except Exception as e:
        # Handle all other exceptions
        if logger:
            logger.error(f"Error executing Claude command: {str(e)}")

        # Try to terminate the process if it's still running
        if process and process.returncode is None:
            try:
                process.terminate()
                await asyncio.sleep(0.5)
                if process.returncode is None:
                    process.kill()
            except Exception as term_error:
                if logger:
                    logger.error(f"Error terminating subprocess during exception handling: {str(term_error)}")

        # Return failure with error information
        return False, {"error": str(e), "timeout": False}


async def prompt_claude(
    prompt: str,
    feedback: str,
    configs: dict,
    logger=None,
    agent_name: str = None,
    sub_agent_name: str = None,
) -> tuple[bool, dict]:
    """
    Execute Claude CLI command with the given prompt for regular text responses.

    Args:
        prompt (str): The prompt to send to Claude
        feedback (str): Optional feedback to append to the prompt for retries
        configs (dict): Configuration settings
        logger (logging.Logger, optional): Logger to use. If None, logs to console only.
        agent_name (str): The name of the agent running the command
        sub_agent_name (str, optional): The name of the sub-agent running the command

    Returns:
        tuple[bool, dict]: (success_status, captured_output)

    Raises:
        ValueError: If agent_name is not provided
    """
    if not agent_name:
        raise ValueError("Agent name must be provided to run Claude command")

    if feedback != "" and logger:
        logger.info(f"Feedback provided: {feedback}")
        prompt += f"\\n\\nFeedback: {feedback}"

    try:
        if logger:
            logger.info("Executing Claude CLI command...")
        # Use asyncio.create_subprocess_exec for true async operation
        cmd = ["claude", "-p", prompt]
        if configs.get("mcp_config_file"):
            cmd += ["--mcp-config", configs["mcp_config_file"]]
        if configs.get("extra_agent_args"):
            cmd += configs["extra_agent_args"]

        process = await asyncio.create_subprocess_exec(
            *cmd,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )

        stdout, stderr = await process.communicate()

        if process.returncode != 0:
            if logger:
                logger.error(f"Claude failed with exit code {process.returncode}")
                logger.error(f"Error details: {stderr.decode(errors='ignore')}")
            return False, {"error": stderr.decode(errors="ignore")}

        # Process completed successfully
        output = stdout.decode("utf-8")
        return True, {"result": output}

    except Exception as e:
        if logger:
            logger.error(f"Error executing Claude command: {str(e)}")
        return False, {"error": str(e)}
