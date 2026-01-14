import os
import subprocess
from typing import Dict, Any
from fastmcp import FastMCP

mcp = FastMCP(name="Crust Test Runner")


@mcp.tool
def run_tests(project_name: str) -> Dict[str, Any]:
    """Run cargo tests in a Rust project and return the raw output.

    This tool executes the `cargo test` command within the specified Crust
    project and returns the raw output.

    Args:
        project_name (str): The name of the Crust project, which must be one of the projects
                          in the data/tool_projects/crust directory.

    Returns:
        Dict[str, Any]: A dictionary containing:
            - success (bool): True if the command executed successfully, False otherwise
            - output (str): The complete raw output of the cargo test command
            - error (str): Error message if any
    """
    return run(project_name)


def run(project_name: str) -> Dict[str, Any]:
    """Run cargo tests in a Rust project and return the raw output.

    Implementation function that can be called directly from Python code.
    """
    # Construct the project path
    project_path = f"/workspace/data/tool_projects/crust/{project_name}/rust"

    # Validate the project path
    if not os.path.exists(project_path):
        return {
            "success": False,
            "output": "",
            "error": f"Project '{project_name}' does not exist in data/tool_projects/crust",
        }

    cargo_toml_path = os.path.join(project_path, "Cargo.toml")
    if not os.path.exists(cargo_toml_path):
        return {
            "success": False,
            "output": "",
            "error": f"No Cargo.toml found in '{project_path}'. Not a valid Rust project.",
        }

    # Restore the original test file before running tests
    subprocess.run(["git", "restore", "src/bin/test.rs"], cwd=project_path)

    # Run the cargo test command
    cmd = ["cargo", "test", "--", "--show-output"]

    try:
        result = subprocess.run(
            cmd,
            cwd=project_path,
            capture_output=True,
            text=True,
            check=False,
        )
        output = result.stdout + result.stderr
        return_code = result.returncode

        return {
            "success": return_code == 0,
            "output": output,
            "error": "" if return_code == 0 else f"Command exited with status {return_code}",
        }
    except Exception as e:
        return {"success": False, "output": "", "error": f"Failed to execute cargo test: {str(e)}"}


if __name__ == "__main__":
    mcp.run(transport="stdio")
