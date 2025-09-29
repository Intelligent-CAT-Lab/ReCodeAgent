#!/usr/bin/env python3
"""
MCP Server for Language Server Protocol operations using multilspy.

This server provides language-agnostic LSP functionality through multilspy,
supporting multiple programming languages including Java, Python, Rust, C#,
TypeScript, JavaScript, Go, Dart, and Ruby.
"""

import asyncio
import json
from typing import Any, Dict, List, Optional, Union
from pathlib import Path

try:
    from multilspy import SyncLanguageServer
    from multilspy.multilspy_config import MultilspyConfig
    from multilspy.multilspy_logger import MultilspyLogger
    from multilspy import multilspy_types
except ImportError:
    raise ImportError(
        "multilspy is required. Install with: pip install multilspy"
    )

from fastmcp import FastMCP

# Initialize the MCP server
mcp = FastMCP("Language Server Protocol")

# Global variables to maintain LSP server state
_lsp_servers: Dict[str, SyncLanguageServer] = {}
_project_roots: Dict[str, str] = {}


def _get_or_create_lsp_server(
    language: str, project_root: str
) -> SyncLanguageServer:
    """
    Get existing LSP server or create a new one for the given language and project.
    
    Args:
        language: Programming language (java, python, rust, csharp, typescript, javascript, go, dart, ruby)
        project_root: Absolute path to the project root directory
        
    Returns:
        SyncLanguageServer instance
    """
    key = f"{language}:{project_root}"
    
    if key not in _lsp_servers:
        config = MultilspyConfig.from_dict({"code_language": language})
        logger = MultilspyLogger()
        _lsp_servers[key] = SyncLanguageServer.create(config, logger, project_root)
        _project_roots[key] = project_root
    
    return _lsp_servers[key]


@mcp.tool()
def initialize_lsp_server(
    language: str,
    project_root: str
) -> Dict[str, Any]:
    """
    Initialize a language server for a specific programming language and project.
    
    This tool sets up a language server instance that can be used for subsequent
    LSP operations like go-to-definition, completions, etc.
    
    Args:
        language: Programming language. Supported values: "java", "python", "rust", 
                 "csharp", "typescript", "javascript", "go", "dart", "ruby"
        project_root: Absolute path to the project root directory
        
    Returns:
        Dictionary containing initialization status and server information
        
    Example:
        initialize_lsp_server("python", "/path/to/my/python/project")
    """
    try:
        # Validate language
        supported_languages = [
            "java", "python", "rust", "csharp", "typescript", 
            "javascript", "go", "dart", "ruby"
        ]
        if language not in supported_languages:
            return {
                "success": False,
                "error": f"Unsupported language: {language}. Supported: {supported_languages}"
            }
        
        # Validate project root exists
        if not Path(project_root).exists():
            return {
                "success": False,
                "error": f"Project root does not exist: {project_root}"
            }
        
        # Create or get existing server
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        return {
            "success": True,
            "language": language,
            "project_root": project_root,
            "message": f"LSP server initialized for {language} at {project_root}"
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to initialize LSP server: {str(e)}"
        }


@mcp.tool()
def request_definition(
    language: str,
    project_root: str,
    file_path: str,
    line: int,
    column: int
) -> Dict[str, Any]:
    """
    Request go-to-definition information for a symbol at a specific location.
    
    This tool finds the definition location of a symbol (variable, function, class, etc.)
    at the specified position in the code.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        file_path: Relative path to the code file from project root
        line: Line number (0-based) where the symbol is located
        column: Column number (0-based) where the symbol is located
        
    Returns:
        Dictionary containing definition locations or error information
        
    Example:
        request_definition("java", "/path/to/project", "src/Main.java", 10, 5)
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(file_path):
                result = lsp_server.request_definition(file_path, line, column)
            
        return {
            "success": True,
            "definitions": result,
            "file_path": file_path,
            "line": line,
            "column": column
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get definition: {str(e)}",
            "file_path": file_path,
            "line": line,
            "column": column
        }


@mcp.tool()
def request_references(
    language: str,
    project_root: str,
    file_path: str,
    line: int,
    column: int
) -> Dict[str, Any]:
    """
    Find all references to a symbol at a specific location.
    
    This tool finds all locations in the codebase where a symbol is referenced.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        file_path: Relative path to the code file from project root
        line: Line number (0-based) where the symbol is located
        column: Column number (0-based) where the symbol is located
        
    Returns:
        Dictionary containing reference locations or error information
        
    Example:
        request_references("typescript", "/path/to/project", "src/utils.ts", 5, 12)
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(file_path):
                result = lsp_server.request_references(file_path, line, column)
            
        return {
            "success": True,
            "references": result,
            "file_path": file_path,
            "line": line,
            "column": column
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get references: {str(e)}",
            "file_path": file_path,
            "line": line,
            "column": column
        }


@mcp.tool()
def request_document_symbols(
    language: str,
    project_root: str,
    relative_file_path: str
) -> Dict[str, Any]:
    """
    Get all symbols (functions, classes, variables, etc.) in a document.
    
    This tool provides a hierarchical list of all symbols defined in the specified file,
    useful for code navigation and understanding file structure.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        
    Returns:
        Dictionary containing document symbols or error information
        
    Example:
        request_document_symbols("rust", "/path/to/project", "src/lib.rs")
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                result = lsp_server.request_document_symbols(relative_file_path)
            
        return {
            "success": True,
            "symbols": result,
            "file_path": relative_file_path
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get document symbols: {str(e)}",
            "file_path": relative_file_path
        }


@mcp.tool()
def request_hover(
    language: str,
    project_root: str,
    relative_file_path: str,
    line: int,
    column: int
) -> Dict[str, Any]:
    """
    Get hover information for a symbol at a specific location.
    
    This tool provides detailed information about a symbol when hovering over it,
    including type information, documentation, and signatures.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        line: Line number (0-based) where the symbol is located
        column: Column number (0-based) where the symbol is located
        
    Returns:
        Dictionary containing hover information or error information
        
    Example:
        request_hover("go", "/path/to/project", "main.go", 20, 10)
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                result = lsp_server.request_hover(relative_file_path, line, column)
            
        return {
            "success": True,
            "hover_info": result,
            "file_path": relative_file_path,
            "line": line,
            "column": column
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get hover info: {str(e)}",
            "file_path": relative_file_path,
            "line": line,
            "column": column
        }


@mcp.tool()
def request_workspace_symbol(
    language: str,
    project_root: str,
    query: str
) -> Dict[str, Any]:
    """
    Search for symbols across the entire workspace.
    
    This tool searches for symbols (functions, classes, variables, etc.) across
    all files in the workspace, filtered by a query string.
    
    Args:
        language: Programming language of the project
        project_root: Absolute path to the project root directory
        query: Search query to filter symbols
        
    Returns:
        Dictionary containing workspace symbols or error information
        
    Example:
        request_workspace_symbol("csharp", "/path/to/project", "MyClass")
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            result = lsp_server.request_workspace_symbol(query)
            
        return {
            "success": True,
            "symbols": result,
            "query": query
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get workspace symbols: {str(e)}",
            "query": query
        }


@mcp.tool()
def open_file(
    language: str,
    project_root: str,
    relative_file_path: str
) -> Dict[str, Any]:
    """
    Open a file in the Language Server.
    
    This is required before making any requests to the Language Server for a specific file.
    The file remains open until the server is cleaned up or restarted.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        
    Returns:
        Dictionary containing operation status
        
    Example:
        open_file("java", "/path/to/project", "src/Main.java")
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                # File is now open in the language server
                pass
            
        return {
            "success": True,
            "message": f"File {relative_file_path} opened successfully",
            "file_path": relative_file_path
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to open file: {str(e)}",
            "file_path": relative_file_path
        }


@mcp.tool()
def get_open_file_text(
    language: str,
    project_root: str,
    relative_file_path: str
) -> Dict[str, Any]:
    """
    Get the contents of the given opened file as per the Language Server.
    
    This tool retrieves the current content of a file that has been opened in the
    language server, which may include any modifications made through the LSP.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        
    Returns:
        Dictionary containing the file content or error information
        
    Example:
        get_open_file_text("go", "/path/to/project", "main.go")
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                content = lsp_server.get_open_file_text(relative_file_path)
            
        return {
            "success": True,
            "content": content,
            "file_path": relative_file_path
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get file content: {str(e)}",
            "file_path": relative_file_path
        }


@mcp.tool()
def cleanup_lsp_servers() -> Dict[str, Any]:
    """
    Clean up all active LSP server instances.
    
    This tool properly shuts down all active language server instances and clears
    the internal cache. Useful for freeing resources or resetting the server state.
    
    Returns:
        Dictionary containing cleanup status information
        
    Example:
        cleanup_lsp_servers()
    """
    try:
        global _lsp_servers, _project_roots
        
        servers_count = len(_lsp_servers)
        
        # Clear the dictionaries
        _lsp_servers.clear()
        _project_roots.clear()
        
        return {
            "success": True,
            "message": f"Cleaned up {servers_count} LSP server instances"
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to cleanup LSP servers: {str(e)}"
        }


@mcp.tool()
def list_active_servers() -> Dict[str, Any]:
    """
    List all currently active LSP server instances.
    
    This tool provides information about all active language server instances,
    including their languages and project roots.
    
    Returns:
        Dictionary containing information about active servers
        
    Example:
        list_active_servers()
    """
    try:
        servers_info = []
        
        for key, project_root in _project_roots.items():
            language = key.split(':')[0]
            servers_info.append({
                "language": language,
                "project_root": project_root,
                "key": key
            })
        
        return {
            "success": True,
            "active_servers": servers_info,
            "count": len(servers_info)
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to list active servers: {str(e)}"
        }


if __name__ == "__main__":
    mcp.run(transport="stdio")
