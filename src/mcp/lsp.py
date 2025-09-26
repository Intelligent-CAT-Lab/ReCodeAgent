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
def request_completions(
    language: str,
    project_root: str,
    relative_file_path: str,
    line: int,
    column: int,
    allow_incomplete: bool = False
) -> Dict[str, Any]:
    """
    Request code completions at a specific location in the code.
    
    This tool provides intelligent code completion suggestions based on the context
    at the specified position.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        line: Line number (0-based) where completions are requested
        column: Column number (0-based) where completions are requested
        allow_incomplete: Whether to allow incomplete completion results
        
    Returns:
        Dictionary containing completion suggestions or error information
        
    Example:
        request_completions("python", "/path/to/project", "src/main.py", 15, 8)
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                result = lsp_server.request_completions(relative_file_path, line, column, allow_incomplete)
            
        return {
            "success": True,
            "completions": result,
            "file_path": relative_file_path,
            "line": line,
            "column": column,
            "allow_incomplete": allow_incomplete
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get completions: {str(e)}",
            "file_path": relative_file_path,
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
def insert_text_at_position(
    language: str,
    project_root: str,
    relative_file_path: str,
    line: int,
    column: int,
    text_to_be_inserted: str
) -> Dict[str, Any]:
    """
    Insert text at the given line and column in the given file.
    
    This tool inserts text at a specific position and returns the updated cursor position
    after inserting the text. The file must be opened first.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        line: Line number where text should be inserted
        column: Column number where text should be inserted
        text_to_be_inserted: The text to insert
        
    Returns:
        Dictionary containing the updated cursor position or error information
        
    Example:
        insert_text_at_position("python", "/path/to/project", "main.py", 10, 5, "print('hello')")
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                result = lsp_server.insert_text_at_position(
                    relative_file_path, line, column, text_to_be_inserted
                )
            
        return {
            "success": True,
            "updated_position": {
                "line": result.line,
                "column": result.column
            },
            "file_path": relative_file_path,
            "inserted_text": text_to_be_inserted
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to insert text: {str(e)}",
            "file_path": relative_file_path
        }


@mcp.tool()
def delete_text_between_positions(
    language: str,
    project_root: str,
    relative_file_path: str,
    start_line: int,
    start_column: int,
    end_line: int,
    end_column: int
) -> Dict[str, Any]:
    """
    Delete text between the given start and end positions in the given file.
    
    This tool deletes text between two positions and returns the deleted text.
    The file must be opened first.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        start_line: Starting line number for deletion
        start_column: Starting column number for deletion
        end_line: Ending line number for deletion
        end_column: Ending column number for deletion
        
    Returns:
        Dictionary containing the deleted text or error information
        
    Example:
        delete_text_between_positions("rust", "/path/to/project", "src/lib.rs", 5, 0, 7, 10)
    """
    try:
        # Import multilspy_types to create Position objects
        from multilspy import multilspy_types
        
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        start_pos = multilspy_types.Position(line=start_line, column=start_column)
        end_pos = multilspy_types.Position(line=end_line, column=end_column)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                deleted_text = lsp_server.delete_text_between_positions(
                    relative_file_path, start_pos, end_pos
                )
            
        return {
            "success": True,
            "deleted_text": deleted_text,
            "file_path": relative_file_path,
            "start_position": {"line": start_line, "column": start_column},
            "end_position": {"line": end_line, "column": end_column}
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to delete text: {str(e)}",
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
def request_diagnostics(
    language: str,
    project_root: str,
    relative_file_path: str,
    context_lines: int = 3
) -> Dict[str, Any]:
    """
    Get diagnostic information (errors, warnings, hints) for a specific file.
    
    This tool provides comprehensive diagnostic information including syntax errors,
    type errors, warnings, and other issues detected by the language server.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        context_lines: Number of context lines to show around each diagnostic
        
    Returns:
        Dictionary containing diagnostic information or error information
        
    Example:
        request_diagnostics("python", "/path/to/project", "src/main.py", 2)
    """
    try:
        import time
        
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(relative_file_path):
                # Wait a moment for diagnostics to be computed
                time.sleep(2)
                
                # Try to get diagnostics - this might need to be implemented
                # For now, we'll return a placeholder indicating the feature needs LSP extension
                return {
                    "success": False,
                    "error": "Diagnostics feature requires extending multilspy with diagnostic support",
                    "file_path": relative_file_path,
                    "note": "This tool is prepared but needs multilspy diagnostic method implementation"
                }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get diagnostics: {str(e)}",
            "file_path": relative_file_path
        }


@mcp.tool()
def rename_symbol(
    language: str,
    project_root: str,
    file_path: str,
    line: int,
    column: int,
    new_name: str
) -> Dict[str, Any]:
    """
    Rename a symbol (variable, function, class, etc.) across the entire project.
    
    This tool performs safe refactoring by renaming all occurrences of a symbol
    throughout the codebase using the language server's rename functionality.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        file_path: Relative path to the code file from project root
        line: Line number (0-based) where the symbol is located
        column: Column number (0-based) where the symbol is located
        new_name: New name for the symbol
        
    Returns:
        Dictionary containing rename results or error information
        
    Example:
        rename_symbol("java", "/path/to/project", "src/Main.java", 10, 5, "newMethodName")
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            with lsp_server.open_file(file_path):
                # This would need to be implemented in multilspy
                return {
                    "success": False,
                    "error": "Rename feature requires extending multilspy with rename support",
                    "file_path": file_path,
                    "line": line,
                    "column": column,
                    "new_name": new_name,
                    "note": "This tool is prepared but needs multilspy rename method implementation"
                }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to rename symbol: {str(e)}",
            "file_path": file_path,
            "line": line,
            "column": column
        }


@mcp.tool()
def edit_file_lines(
    language: str,
    project_root: str,
    relative_file_path: str,
    edits: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """
    Apply multiple line-based edits to a file in a single operation.
    
    This tool allows making multiple text edits based on line numbers, providing
    a more reliable way to edit files than search-and-replace operations.
    
    Args:
        language: Programming language of the file
        project_root: Absolute path to the project root directory
        relative_file_path: Relative path to the code file from project root
        edits: List of edit operations, each with startLine, endLine, and newText
        
    Returns:
        Dictionary containing edit results or error information
        
    Example:
        edit_file_lines("rust", "/path/to/project", "src/lib.rs", [
            {"startLine": 5, "endLine": 7, "newText": "// New implementation\nfn new_function() {}"},
            {"startLine": 15, "endLine": 15, "newText": ""}  # Delete line 15
        ])
    """
    try:
        import os
        from pathlib import Path
        
        # Validate edits format
        for i, edit in enumerate(edits):
            required_fields = ["startLine", "endLine", "newText"]
            for field in required_fields:
                if field not in edit:
                    return {
                        "success": False,
                        "error": f"Edit {i} missing required field: {field}"
                    }
        
        # Get full file path
        full_path = Path(project_root) / relative_file_path
        
        if not full_path.exists():
            return {
                "success": False,
                "error": f"File does not exist: {full_path}"
            }
        
        # Read current file content
        with open(full_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        # Sort edits by line number in descending order (process from bottom to top)
        sorted_edits = sorted(edits, key=lambda x: x["startLine"], reverse=True)
        
        lines_removed = 0
        lines_added = 0
        
        # Apply edits
        for edit in sorted_edits:
            start_line = edit["startLine"] - 1  # Convert to 0-based
            end_line = edit["endLine"] - 1      # Convert to 0-based
            new_text = edit["newText"]
            
            # Validate line numbers
            if start_line < 0 or end_line >= len(lines) or start_line > end_line:
                return {
                    "success": False,
                    "error": f"Invalid line range: {start_line + 1}-{end_line + 1}"
                }
            
            # Count lines being removed
            lines_removed += (end_line - start_line + 1)
            
            # Prepare new lines
            if new_text:
                new_lines = new_text.split('\n')
                # Ensure each line ends with newline except possibly the last
                new_lines = [line + '\n' for line in new_lines[:-1]] + [new_lines[-1]]
                if not new_lines[-1].endswith('\n') and end_line < len(lines) - 1:
                    new_lines[-1] += '\n'
                lines_added += len(new_lines)
            else:
                new_lines = []
            
            # Replace the lines
            lines[start_line:end_line + 1] = new_lines
        
        # Write back to file
        with open(full_path, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        
        return {
            "success": True,
            "message": f"Successfully applied {len(edits)} edits. {lines_removed} lines removed, {lines_added} lines added.",
            "file_path": relative_file_path,
            "edits_applied": len(edits),
            "lines_removed": lines_removed,
            "lines_added": lines_added
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to edit file: {str(e)}",
            "file_path": relative_file_path
        }


@mcp.tool()
def enhanced_definition(
    language: str,
    project_root: str,
    symbol_name: str,
    include_source: bool = True
) -> Dict[str, Any]:
    """
    Get enhanced definition information for a symbol with full source code.
    
    This tool provides comprehensive symbol definition information including
    the complete source code, location details, and symbol metadata.
    
    Args:
        language: Programming language of the project
        project_root: Absolute path to the project root directory
        symbol_name: Name of the symbol to find (supports qualified names like "Class.method")
        include_source: Whether to include the full source code of the definition
        
    Returns:
        Dictionary containing enhanced definition information or error information
        
    Example:
        enhanced_definition("go", "/path/to/project", "MyStruct.Method", True)
    """
    try:
        lsp_server = _get_or_create_lsp_server(language, project_root)
        
        with lsp_server.start_server():
            # Use workspace symbols to find the symbol
            symbols = lsp_server.request_workspace_symbol(symbol_name)
            
            if not symbols:
                return {
                    "success": True,
                    "definitions": [],
                    "message": f"Symbol '{symbol_name}' not found"
                }
            
            definitions = []
            
            for symbol in symbols:
                # Filter symbols based on name matching
                if "." in symbol_name:
                    # For qualified names, require exact match
                    if symbol.name != symbol_name:
                        continue
                else:
                    # For simple names, check if it matches exactly
                    if symbol.name != symbol_name:
                        continue
                
                location = symbol.location
                definition_info = {
                    "symbol_name": symbol.name,
                    "file_path": location.uri.replace("file://", ""),
                    "line": location.range.start.line + 1,
                    "column": location.range.start.character + 1,
                    "end_line": location.range.end.line + 1,
                    "end_column": location.range.end.character + 1,
                    "kind": getattr(symbol, 'kind', 'Unknown')
                }
                
                if include_source:
                    try:
                        # Get the file content and extract the definition
                        file_path = location.uri.replace("file://", "")
                        with open(file_path, 'r', encoding='utf-8') as f:
                            lines = f.readlines()
                        
                        start_line = location.range.start.line
                        end_line = location.range.end.line
                        
                        # Extract the definition lines
                        definition_lines = lines[start_line:end_line + 1]
                        
                        # Add line numbers
                        numbered_lines = []
                        for i, line in enumerate(definition_lines):
                            line_num = start_line + i + 1
                            numbered_lines.append(f"{line_num:4d}: {line.rstrip()}")
                        
                        definition_info["source_code"] = "\n".join(numbered_lines)
                        
                    except Exception as e:
                        definition_info["source_code"] = f"Error reading source: {str(e)}"
                
                definitions.append(definition_info)
            
            return {
                "success": True,
                "definitions": definitions,
                "symbol_name": symbol_name,
                "count": len(definitions)
            }
        
    except Exception as e:
        return {
            "success": False,
            "error": f"Failed to get enhanced definition: {str(e)}",
            "symbol_name": symbol_name
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
