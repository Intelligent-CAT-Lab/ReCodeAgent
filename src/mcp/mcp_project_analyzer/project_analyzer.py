from typing import Dict, Any
from tree_sitter import Language, Parser
import tree_sitter_python as tsp
import tree_sitter_java as tsj
import tree_sitter_javascript as tsjs
import tree_sitter_go as tsgo
import tree_sitter_c as tsc
import tree_sitter_rust as tsr
from typing import Dict, List, Optional, Any, Tuple
import os
from fastmcp import FastMCP

mcp = FastMCP(name="Project Analyzer")


def extract_file_skeleton(language: str, file_path: str) -> Dict[str, Any]:
    """
    Extract a high-level language-agnostic skeleton of a file.

    Args:
        language: The programming language of the file ('python', 'java', 'javascript', 'go', 'c', 'rust')
        file_path: Path to the source code file

    Returns:
        A dictionary representing the high-level structure of the file
    """
    # Read file content
    content = ""
    with open(file_path, "r") as file:
        content = file.read()

    # Get the appropriate language parser
    language_obj = _get_language_by_name(language)
    parser = Parser(language_obj)

    # Parse the content
    tree = parser.parse(bytes(content, "utf8"))

    # Extract the skeleton
    skeleton = _extract_skeleton(tree.root_node, language)

    return {"file_path": file_path, "language": language, "skeleton": skeleton}


def _get_language_by_name(language: str) -> Language:
    """Get the Tree-sitter Language object for the specified language."""
    language = language.lower()

    if language == "python":
        return Language(tsp.language())
    elif language == "java":
        return Language(tsj.language())
    elif language == "javascript" or language == "js":
        return Language(tsjs.language())
    elif language == "go":
        return Language(tsgo.language())
    elif language == "c":
        return Language(tsc.language())
    elif language == "rust":
        return Language(tsr.language())
    else:
        raise ValueError(f"Unsupported language: {language}")


def _extract_skeleton(node, language: str) -> Dict[str, Any]:
    """
    Recursively extract the skeleton from a syntax tree node.

    This function identifies important structural elements like classes, functions,
    methods, and fields, creating a language-agnostic representation.

    Args:
        node: Tree-sitter syntax tree node
        language: The programming language being parsed

    Returns:
        A dictionary representing the structure of the code
    """
    if node is None:
        return {}

    result = {"type": node.type}

    # Add node start and end position
    result["start_line"] = node.start_point[0] + 1  # 0-indexed to 1-indexed
    result["end_line"] = node.end_point[0] + 1  # 0-indexed to 1-indexed

    # Process imports
    imports = _extract_imports(node, language)
    result["imports"] = imports  # Always include imports, even if empty

    # Process classes, functions, and global variables based on language
    structs = []  # For Go structs
    if language.lower() == "python":
        classes, functions, globals_dict = _extract_python_definitions(node)
    elif language.lower() == "java":
        classes, functions, globals_dict = _extract_java_definitions(node)
    elif language.lower() in ["javascript", "js"]:
        classes, functions, globals_dict = _extract_javascript_definitions(node)
    elif language.lower() == "go":
        classes, functions, globals_dict, structs = _extract_go_definitions(node)
    elif language.lower() == "c":
        classes, functions, globals_dict = _extract_c_definitions(node)
    elif language.lower() == "rust":
        classes, functions, globals_dict = _extract_rust_definitions(node)
    else:
        classes, functions, globals_dict = [], [], []

    # Post-process to ensure consistency across languages
    # Functions with the same name as a class are likely class-related methods
    if classes and functions:
        functions = _assign_class_related_functions(classes, functions)

    # Standardize the schema by ensuring all fields exist with proper defaults
    classes = _standardize_classes(classes)
    functions = _standardize_functions(functions)
    globals_list = _standardize_globals(globals_dict)

    # Always include classes, functions, and globals, even if empty
    result["classes"] = classes
    result["functions"] = functions
    result["globals"] = globals_list

    # Add structs for Go language
    if language.lower() == "go" and structs:
        result["structs"] = _standardize_classes(structs)  # Reuse class standardization for structs

    return result


def _standardize_globals(globals_dict: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """
    Standardize the schema for global variable entries to ensure consistency.

    Adds default values for missing fields.
    """
    standardized = []
    for glob in globals_dict:
        # Ensure standard global variable schema
        standardized_global = {
            "name": glob.get("name", ""),
            "start_line": glob.get("start_line", 0),
            "end_line": glob.get("end_line", 0),
            "type": glob.get("type", None),
            "value": glob.get("value", None),
        }

        # Copy any additional fields not in the standard schema
        for key, value in glob.items():
            if key not in standardized_global:
                standardized_global[key] = value

        standardized.append(standardized_global)

    return standardized


def _standardize_classes(classes: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """
    Standardize the schema for class entries to ensure consistency.

    Adds default values for missing fields.
    """
    standardized = []
    for cls in classes:
        # Ensure standard class schema
        standardized_class = {
            "name": cls.get("name", ""),
            "start_line": cls.get("start_line", 0),
            "end_line": cls.get("end_line", 0),
            "methods": _standardize_functions(cls.get("methods", [])),
            "fields": _standardize_fields(cls.get("fields", [])),
            "superclasses": cls.get("superclasses", []),
            "interfaces": cls.get("interfaces", []),
        }

        # Handle single superclass vs list of superclasses
        if "superclass" in cls and "superclasses" not in cls:
            standardized_class["superclasses"] = [cls["superclass"]]

        standardized.append(standardized_class)

    return standardized


def _standardize_functions(functions: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """
    Standardize the schema for function entries to ensure consistency.

    Adds default values for missing fields.
    """
    standardized = []
    for func in functions:
        # Ensure standard function schema
        standardized_func = {
            "name": func.get("name", ""),
            "start_line": func.get("start_line", 0),
            "end_line": func.get("end_line", 0),
            "parameters": func.get("parameters", []),
            "return_type": func.get("return_type", None),
        }

        # Include parameter types if available
        if "parameter_types" in func:
            standardized_func["parameter_types"] = func["parameter_types"]

        # For functions without parameter types but with parameters,
        # create a placeholder array of nulls
        elif "parameters" in func and func["parameters"]:
            standardized_func["parameter_types"] = [None] * len(func["parameters"])

        # Copy any additional fields not in the standard schema
        for key, value in func.items():
            if key not in standardized_func:
                standardized_func[key] = value

        standardized.append(standardized_func)

    return standardized


def _standardize_fields(fields: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """
    Standardize the schema for field entries to ensure consistency.

    Adds default values for missing fields.
    """
    standardized = []
    for field in fields:
        # Ensure standard field schema
        standardized_field = {
            "name": field.get("name", ""),
            "start_line": field.get("start_line", 0),
            "end_line": field.get("end_line", 0),
            "type": field.get("type", None),
        }

        # Copy any additional fields not in the standard schema
        for key, value in field.items():
            if key not in standardized_field:
                standardized_field[key] = value

        standardized.append(standardized_field)

    return standardized


def _assign_class_related_functions(classes, functions):
    """
    Assign functions to classes if they appear to be related to the class.

    This helps create a consistent structure across languages where some languages
    may parse class-related functions as standalone functions.
    """
    class_names = {cls["name"]: cls for cls in classes if "name" in cls}
    remaining_functions = []

    for func in functions:
        assigned = False
        if "name" in func:
            # Check if function name matches a class name (exact match or with numbers)
            for class_name, cls in class_names.items():
                # Match pattern: ClassName, ClassName0, ClassName1, etc.
                if func["name"] == class_name or (
                    func["name"].startswith(class_name) and func["name"][len(class_name) :].isdigit()
                ):
                    # This is likely a class-related method
                    if "methods" not in cls:
                        cls["methods"] = []
                    cls["methods"].append(func)
                    assigned = True
                    break

        if not assigned:
            remaining_functions.append(func)

    return remaining_functions


def _extract_imports(node, language: str) -> List[str]:
    """Extract import statements from the syntax tree."""
    imports = []

    # Different languages have different import node types
    import_node_types = {
        "python": ["import_statement", "import_from_statement", "future_import_statement"],
        "java": ["import_declaration"],
        "javascript": ["import_statement", "import_declaration"],
        "go": ["import_declaration", "import_spec"],
        "rust": ["use_declaration"],
        "c": ["preproc_include"],
    }

    node_types = import_node_types.get(language.lower(), [])

    def visit_node(node):
        if node.type in node_types:
            imports.append(node.text.decode("utf-8").strip())
        # Special case for Python's __future__ imports which might be parsed differently
        elif language.lower() == "python" and node.type == "expression_statement":
            text = node.text.decode("utf-8").strip()
            if text.startswith("from __future__") and "import" in text:
                imports.append(text)

        for child in node.children:
            visit_node(child)

    visit_node(node)
    return imports


def _extract_python_definitions(node) -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]], List[Dict[str, Any]]]:
    """Extract class, function, and global variable definitions from Python code."""
    classes = {}  # Using a dict to make it easier to find classes by name
    functions = []
    globals_list = []

    # First pass: Extract all classes, functions, and global variables
    def first_pass(node):
        if node.type == "class_definition":
            class_info = _extract_python_class(node)
            if class_info and "name" in class_info:
                classes[class_info["name"]] = class_info
        elif node.type == "function_definition":
            # Only include top-level functions (not methods inside class bodies)
            if (
                node.parent
                and node.parent.type != "block"
                or not any(parent.type == "class_definition" for parent in _get_parents(node))
            ):
                function_info = _extract_python_function(node)
                if function_info:
                    functions.append(function_info)
        elif node.type == "assignment" and _is_global_scope(node):
            # Global variable assignment
            global_var = _extract_python_global(node)
            if global_var:
                globals_list.append(global_var)

        for child in node.children:
            first_pass(child)

    # Second pass: Check for function names that match class names with 'classname0' or 'classname1' pattern
    # These are typically constructors or static methods that belong to classes
    def second_pass(funcs):
        remaining_funcs = []
        for func in funcs:
            assigned = False
            if "name" in func:
                # Check if function name matches pattern: ClassName0 or ClassName1
                for class_name, class_info in classes.items():
                    if func["name"] == class_name or (
                        func["name"].startswith(class_name) and func["name"][len(class_name) :].isdigit()
                    ):
                        # This is likely a constructor or static method
                        if "methods" not in class_info:
                            class_info["methods"] = []
                        class_info["methods"].append(func)
                        assigned = True
                        break

            if not assigned:
                remaining_funcs.append(func)
        return remaining_funcs

    first_pass(node)
    functions = second_pass(functions)

    return list(classes.values()), functions, globals_list


def _extract_python_global(node) -> Dict[str, Any]:
    """Extract information about a Python global variable."""
    variable_info = {}

    # Find variable name and value
    name = None
    value = None
    var_type = None

    try:
        # For simple assignments like x = 10
        if node.children[0].type == "identifier":
            name = node.children[0].text.decode("utf-8")

            # Try to infer type from the assigned value
            if len(node.children) > 2:
                value_node = node.children[2]
                value = value_node.text.decode("utf-8")

                if value_node.type == "integer":
                    var_type = "int"
                elif value_node.type == "float":
                    var_type = "float"
                elif value_node.type == "string":
                    var_type = "str"
                elif value_node.type == "true" or value_node.type == "false":
                    var_type = "bool"
    except (IndexError, AttributeError):
        pass

    if name:
        variable_info["name"] = name
        variable_info["value"] = value
        variable_info["type"] = var_type
        variable_info["start_line"] = node.start_point[0] + 1
        variable_info["end_line"] = node.end_point[0] + 1

    return variable_info


def _extract_python_class(node) -> Dict[str, Any]:
    """Extract information about a Python class."""
    # Find the class name
    class_name = None
    for child in node.children:
        if child.type == "identifier":
            class_name = child.text.decode("utf-8")
            break

    if not class_name:
        return {}

    class_info = {
        "name": class_name,
        "methods": [],
        "fields": [],
        "start_line": node.start_point[0] + 1,  # 0-indexed to 1-indexed
        "end_line": node.end_point[0] + 1,  # 0-indexed to 1-indexed
    }

    # Find superclasses
    superclasses = []
    for child in node.children:
        if child.type == "argument_list":
            for arg_child in child.children:
                if arg_child.type == "identifier":
                    superclasses.append(arg_child.text.decode("utf-8"))

    if superclasses:
        class_info["superclasses"] = superclasses

    # Process class body
    for child in node.children:
        if child.type == "block":
            for body_node in child.children:
                # Methods
                if body_node.type == "function_definition":
                    method_info = _extract_python_function(body_node)
                    if method_info:
                        class_info["methods"].append(method_info)

                # Fields (class variables)
                elif body_node.type == "expression_statement":
                    # This is a simplified approach, may need refinement
                    if "assignment" in [c.type for c in body_node.children]:
                        field_text = body_node.text.decode("utf-8").strip()
                        if field_text:
                            field_name = field_text.split("=")[0].strip()
                            class_info["fields"].append(
                                {
                                    "name": field_name,
                                    "start_line": body_node.start_point[0] + 1,
                                    "end_line": body_node.end_point[0] + 1,
                                }
                            )

    return class_info


def _extract_python_function(node) -> Dict[str, Any]:
    """Extract information about a Python function or method."""
    # Find the function name
    func_name = None
    for child in node.children:
        if child.type == "identifier":
            func_name = child.text.decode("utf-8")
            break

    if not func_name:
        return {}

    func_info = {
        "name": func_name,
        "start_line": node.start_point[0] + 1,  # 0-indexed to 1-indexed
        "end_line": node.end_point[0] + 1,  # 0-indexed to 1-indexed
    }

    # Get parameters
    params = []
    for child in node.children:
        if child.type == "parameters":
            for param_child in child.children:
                if param_child.type == "identifier":
                    params.append(param_child.text.decode("utf-8"))

    if params:
        func_info["parameters"] = params

    # Get return type annotation if it exists
    for child in node.children:
        if child.type == "type":
            func_info["return_type"] = child.text.decode("utf-8")

    return func_info


def _extract_java_definitions(node) -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]], List[Dict[str, Any]]]:
    """Extract class, function, and global variable definitions from Java code."""
    classes = []
    functions = []
    globals_list = []

    def visit_node(node):
        if node.type == "class_declaration":
            class_info = _extract_java_class(node)
            if class_info:
                classes.append(class_info)
        elif node.type == "method_declaration":
            # Skip if it's a method inside a class (handled by _extract_java_class)
            if not any(parent.type == "class_body" for parent in _get_parents(node)):
                function_info = _extract_java_method(node)
                if function_info:
                    functions.append(function_info)

        # Java doesn't really have true global variables, but we'll check for static fields
        elif node.type == "field_declaration" and _is_global_scope(node):
            global_var = _extract_java_global(node)
            if global_var:
                globals_list.append(global_var)

        for child in node.children:
            visit_node(child)

    visit_node(node)
    return classes, functions, globals_list


def _extract_java_global(node) -> Dict[str, Any]:
    """Extract information about a Java global variable (static field)."""
    # Java doesn't have true global variables in the same sense as other languages
    # This is just a placeholder for the API
    field_info = _extract_java_field(node)
    # Add static modifier indication
    field_info["is_static"] = True
    return field_info


def _extract_java_class(node) -> Dict[str, Any]:
    """Extract information about a Java class."""
    # Find the class name
    class_name = None
    for child in node.children:
        if child.type == "identifier":
            class_name = child.text.decode("utf-8")
            break

    if not class_name:
        return {}

    class_info = {
        "name": class_name,
        "methods": [],
        "fields": [],
        "start_line": node.start_point[0] + 1,  # 0-indexed to 1-indexed
        "end_line": node.end_point[0] + 1,  # 0-indexed to 1-indexed
    }

    # Check for superclass and interfaces
    for child in node.children:
        if child.type == "superclass":
            for sc_child in child.children:
                if sc_child.type == "type_identifier":
                    class_info["superclass"] = sc_child.text.decode("utf-8")

        elif child.type == "super_interfaces":
            interfaces = []
            for si_child in child.children:
                if si_child.type == "type_identifier":
                    interfaces.append(si_child.text.decode("utf-8"))
            if interfaces:
                class_info["interfaces"] = interfaces

    # Process class body
    for child in node.children:
        if child.type == "class_body":
            for body_node in child.children:
                # Methods
                if body_node.type == "method_declaration":
                    method_info = _extract_java_method(body_node)
                    if method_info:
                        class_info["methods"].append(method_info)

                # Constructors
                elif body_node.type == "constructor_declaration":
                    constructor_info = _extract_java_constructor(body_node, class_name)
                    if constructor_info:
                        class_info["methods"].append(constructor_info)

                # Fields
                elif body_node.type == "field_declaration":
                    field_info = _extract_java_field(body_node)
                    if field_info:
                        class_info["fields"].append(field_info)

    return class_info


def _extract_java_method(node) -> Dict[str, Any]:
    """Extract information about a Java method."""
    # Find the method name
    method_name = None
    for child in node.children:
        if child.type == "identifier":
            method_name = child.text.decode("utf-8")
            break

    if not method_name:
        return {}

    method_info = {
        "name": method_name,
        "start_line": node.start_point[0] + 1,  # 0-indexed to 1-indexed
        "end_line": node.end_point[0] + 1,  # 0-indexed to 1-indexed
    }

    # Get return type
    for child in node.children:
        if child.type == "type_identifier" or child.type == "primitive_type" or child.type == "void_type":
            method_info["return_type"] = child.text.decode("utf-8")
            break

    # Get parameters and their types
    params = []
    param_types = []
    for child in node.children:
        if child.type == "formal_parameters":
            for param_child in child.children:
                if param_child.type == "formal_parameter":
                    param_name = None
                    param_type = None

                    # Extract parameter name
                    for pc in param_child.children:
                        if pc.type == "identifier":
                            param_name = pc.text.decode("utf-8")
                            break

                    # Extract parameter type
                    for pc in param_child.children:
                        if pc.type == "type_identifier" or pc.type == "primitive_type":
                            param_type = pc.text.decode("utf-8")
                            break
                        elif pc.type == "array_type":
                            # Handle array types
                            for array_child in pc.children:
                                if array_child.type == "type_identifier" or array_child.type == "primitive_type":
                                    param_type = array_child.text.decode("utf-8") + "[]"
                                    break

                    if param_name:
                        params.append(param_name)
                        param_types.append(param_type)

    if params:
        method_info["parameters"] = params
        method_info["parameter_types"] = param_types

    return method_info


def _extract_java_constructor(node, class_name: str) -> Dict[str, Any]:
    """Extract information about a Java constructor."""
    # Constructor has the same name as the class
    constructor_info = {
        "name": class_name,
        "start_line": node.start_point[0] + 1,  # 0-indexed to 1-indexed
        "end_line": node.end_point[0] + 1,  # 0-indexed to 1-indexed
    }

    # Constructors return the instance of the class they're constructing
    constructor_info["return_type"] = class_name

    # Get parameters and their types
    params = []
    param_types = []
    for child in node.children:
        if child.type == "formal_parameters":
            for param_child in child.children:
                if param_child.type == "formal_parameter":
                    param_name = None
                    param_type = None

                    # Extract parameter name
                    for pc in param_child.children:
                        if pc.type == "identifier":
                            param_name = pc.text.decode("utf-8")
                            break

                    # Extract parameter type
                    for pc in param_child.children:
                        if pc.type == "type_identifier" or pc.type == "primitive_type":
                            param_type = pc.text.decode("utf-8")
                            break
                        elif pc.type == "array_type":
                            # Handle array types
                            for array_child in pc.children:
                                if array_child.type == "type_identifier" or array_child.type == "primitive_type":
                                    param_type = array_child.text.decode("utf-8") + "[]"
                                    break

                    if param_name:
                        params.append(param_name)
                        param_types.append(param_type)

    if params:
        constructor_info["parameters"] = params
        constructor_info["parameter_types"] = param_types

    return constructor_info


def _extract_java_field(node) -> Dict[str, Any]:
    """Extract information about a Java field."""
    field_info = {
        "start_line": node.start_point[0] + 1,  # 0-indexed to 1-indexed
        "end_line": node.end_point[0] + 1,  # 0-indexed to 1-indexed
    }

    # Get field name
    for child in node.children:
        if child.type == "variable_declarator":
            for vd_child in child.children:
                if vd_child.type == "identifier":
                    field_info["name"] = vd_child.text.decode("utf-8")

    # Get field type
    for child in node.children:
        if child.type == "type_identifier" or child.type == "primitive_type":
            field_info["type"] = child.text.decode("utf-8")
            break

    return field_info


def _extract_javascript_definitions(node) -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]], List[Dict[str, Any]]]:
    """Extract class, function, and global variable definitions from JavaScript code."""
    classes = []
    functions = []
    globals_list = []

    def visit_node(node):
        if node.type == "class_declaration":
            class_info = _extract_javascript_class(node)
            if class_info:
                classes.append(class_info)
        elif node.type == "function_declaration":
            function_info = _extract_javascript_function(node)
            if function_info:
                functions.append(function_info)
        elif node.type == "lexical_declaration" or node.type == "variable_declaration":
            # Check if this is a top-level declaration (global scope)
            if _is_javascript_global_scope(node):
                # Process variable declarations at the global scope
                for var_decl in node.children:
                    if var_decl.type == "variable_declarator":
                        function_info = _extract_javascript_variable_function(var_decl)
                        if function_info:
                            functions.append(function_info)
                        else:
                            # It's a global variable declaration
                            global_var = _extract_javascript_global(var_decl)
                            if global_var:
                                globals_list.append(global_var)
            # If not in global scope, we'll check for function expressions
            else:
                for var_decl in node.children:
                    if var_decl.type == "variable_declarator":
                        function_info = _extract_javascript_variable_function(var_decl)
                        if function_info:
                            functions.append(function_info)

        for child in node.children:
            visit_node(child)

    visit_node(node)
    return classes, functions, globals_list


def _extract_javascript_global(node) -> Dict[str, Any]:
    """
    Extract information about a JavaScript global variable.

    A true JavaScript global variable is one declared at the top level of a module,
    not inside any function, class, or block scope.
    """
    variable_info = {}

    # Check if we're actually in the global scope, not inside a function body
    if not _is_javascript_global_scope(node):
        return {}

    name = None
    value = None

    # Find the variable name
    for child in node.children:
        if child.type == "identifier":
            name = child.text.decode("utf-8")
            break

    # Try to get the value and infer type
    value_type = None
    for child in node.children:
        if child.type in ["string_literal", "number_literal", "true", "false", "null", "undefined"]:
            value = child.text.decode("utf-8")
            value_type = child.type
            break

    if name:
        variable_info["name"] = name
        variable_info["value"] = value
        variable_info["type"] = value_type
        variable_info["start_line"] = node.start_point[0] + 1
        variable_info["end_line"] = node.end_point[0] + 1

    return variable_info


def _is_javascript_global_scope(node):
    """
    Check if a node is in the global scope of a JavaScript file.
    This is more specific than _is_global_scope and checks specifically for
    being outside of any function body or block.
    """
    # Check each parent all the way up to the root
    current = node
    while current:
        parent = current.parent
        if not parent:
            break

        # If parent is a function declaration/expression/block, we're not in global scope
        if parent.type in [
            "function_declaration",
            "generator_function_declaration",  # Handle generator functions too
            "function_expression",
            "arrow_function",
            "method_definition",
            "block",  # Check for blocks like if/for/while
            "for_statement",
            "while_statement",
            "if_statement",
            "try_statement",
        ]:
            # If we're a direct child of the parameter list, we may be in global scope
            if current.type == "parameter_list":
                continue

            # We're inside a function body or block, not global scope
            return False

        current = parent

    # If we reach here, we're at the top level (global scope)
    return True


def _extract_javascript_class(node) -> Dict[str, Any]:
    """Extract information about a JavaScript class."""
    # Find the class name
    class_name = None
    for child in node.children:
        if child.type == "identifier":
            class_name = child.text.decode("utf-8")
            break

    if not class_name:
        return {}

    class_info = {
        "name": class_name,
        "methods": [],
        "fields": [],
        "start_line": node.start_point[0] + 1,
        "end_line": node.end_point[0] + 1,
    }

    # Check for superclass
    for child in node.children:
        if child.type == "class_heritage":
            for heritage_child in child.children:
                if heritage_child.type == "identifier":
                    class_info["superclass"] = heritage_child.text.decode("utf-8")
                    break

    # Extract methods and fields
    for child in node.children:
        if child.type == "class_body":
            for body_node in child.children:
                if body_node.type == "method_definition":
                    method_info = _extract_javascript_method(body_node)
                    if method_info:
                        class_info["methods"].append(method_info)
                elif body_node.type == "field_definition":
                    field_info = _extract_javascript_field(body_node)
                    if field_info:
                        class_info["fields"].append(field_info)

    return class_info


def _extract_javascript_method(node) -> Dict[str, Any]:
    """Extract information about a JavaScript class method."""
    # Find the method name
    method_name = None
    for child in node.children:
        if child.type == "property_identifier":
            method_name = child.text.decode("utf-8")
            break

    if not method_name:
        return {}

    method_info = {"name": method_name, "start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get parameters
    params = []
    for child in node.children:
        if child.type == "formal_parameters":
            for param_child in child.children:
                if param_child.type == "identifier":
                    params.append(param_child.text.decode("utf-8"))

    if params:
        method_info["parameters"] = params

    return method_info


def _extract_javascript_field(node) -> Dict[str, Any]:
    """Extract information about a JavaScript class field."""
    field_info = {"start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get field name
    for child in node.children:
        if child.type == "property_identifier":
            field_info["name"] = child.text.decode("utf-8")
            break

    return field_info


def _extract_javascript_function(node) -> Dict[str, Any]:
    """Extract information about a JavaScript function."""
    # Find the function name
    func_name = None
    for child in node.children:
        if child.type == "identifier":
            func_name = child.text.decode("utf-8")
            break

    if not func_name:
        return {}

    func_info = {"name": func_name, "start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get parameters
    params = []
    for child in node.children:
        if child.type == "formal_parameters":
            for param_child in child.children:
                if param_child.type == "identifier":
                    params.append(param_child.text.decode("utf-8"))

    if params:
        func_info["parameters"] = params

    return func_info


def _extract_javascript_variable_function(node) -> Dict[str, Any]:
    """Extract function information from variable declarations (arrow functions or function expressions)."""
    # Find the variable name (which becomes the function name)
    func_name = None
    for child in node.children:
        if child.type == "identifier":
            func_name = child.text.decode("utf-8")
            break

    if not func_name:
        return {}

    # Check if the value is a function
    is_function = False
    function_node = None
    for child in node.children:
        if child.type == "arrow_function" or child.type == "function":
            is_function = True
            function_node = child
            break

    if not is_function or not function_node:
        return {}

    func_info = {
        "name": func_name,
        "start_line": node.start_point[0] + 1,
        "end_line": node.end_point[0] + 1,
        "type": "arrow_function" if function_node.type == "arrow_function" else "function_expression",
    }

    # Get parameters
    params = []
    for child in function_node.children:
        if child.type == "formal_parameters":
            for param_child in child.children:
                if param_child.type == "identifier":
                    params.append(param_child.text.decode("utf-8"))

    if params:
        func_info["parameters"] = params

    return func_info


def _extract_go_definitions(
    node,
) -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]], List[Dict[str, Any]], List[Dict[str, Any]]]:
    """Extract struct, function, and global variable definitions from Go code.

    Returns:
        A tuple containing (empty_classes, functions, globals_list, structs)
        For Go, we keep classes empty and return structs separately
    """
    structs = []
    functions = []
    globals_list = []
    classes = []  # Empty list for classes - Go doesn't have classes

    def visit_node(node):
        if node.type == "type_declaration":
            # Check for struct types
            for child in node.children:
                if child.type == "type_spec":
                    struct_info = _extract_go_struct(child)
                    if struct_info:
                        structs.append(struct_info)
        elif node.type == "function_declaration":
            function_info = _extract_go_function(node)
            if function_info:
                functions.append(function_info)
        elif node.type == "method_declaration":
            method_info = _extract_go_method(node)
            if method_info:
                # Find the struct this method belongs to or create a new entry
                struct_name = _get_go_method_receiver_type(node)
                if struct_name:
                    for struct in structs:
                        if struct["name"] == struct_name:
                            if "methods" not in struct:
                                struct["methods"] = []
                            struct["methods"].append(method_info)
                            break
                    else:
                        # Create a placeholder struct
                        structs.append(
                            {
                                "name": struct_name,
                                "fields": [],
                                "methods": [method_info],
                                "start_line": node.start_point[0] + 1,
                                "end_line": node.end_point[0] + 1,
                            }
                        )
                else:
                    # If we can't determine the receiver, treat it as a function
                    functions.append(method_info)
        # Go global variables
        elif node.type in ["var_declaration", "const_declaration"] and _is_global_scope(node):
            global_vars = _extract_go_globals(node)
            globals_list.extend(global_vars)

        for child in node.children:
            visit_node(child)

    visit_node(node)
    return classes, functions, globals_list, structs


def _extract_go_globals(node) -> List[Dict[str, Any]]:
    """Extract global variables from Go var or const declarations."""
    globals_list = []

    # Handle var and const declarations which can define multiple variables
    for child in node.children:
        if child.type == "var_spec" or child.type == "const_spec":
            names = []
            var_type = None
            value = None

            # Get names (can be multiple in a declaration like var a, b, c int)
            for name_node in child.children:
                if name_node.type == "identifier":
                    names.append(name_node.text.decode("utf-8"))

            # Get type if specified
            for type_node in child.children:
                if type_node.type == "type_identifier" or type_node.type == "primitive_type":
                    var_type = type_node.text.decode("utf-8")
                    break

            # Get value if specified
            for expr_node in child.children:
                if expr_node.type in [
                    "interpreted_string_literal",
                    "raw_string_literal",
                    "int_literal",
                    "float_literal",
                ]:
                    value = expr_node.text.decode("utf-8")
                    break

            # Create a global var entry for each name
            for name in names:
                globals_list.append(
                    {
                        "name": name,
                        "type": var_type,
                        "value": value,
                        "start_line": node.start_point[0] + 1,
                        "end_line": node.end_point[0] + 1,
                    }
                )

    return globals_list


def _extract_go_struct(node) -> Dict[str, Any]:
    """Extract information about a Go struct."""
    # Find the struct name
    struct_name = None
    for child in node.children:
        if child.type == "type_identifier":
            struct_name = child.text.decode("utf-8")
            break

    if not struct_name:
        return {}

    # Check if it's a struct type
    is_struct = False
    struct_def_node = None
    for child in node.children:
        if child.type == "struct_type":
            is_struct = True
            struct_def_node = child
            break

    if not is_struct or not struct_def_node:
        return {}

    struct_info = {
        "name": struct_name,
        "fields": [],
        "methods": [],  # Will be populated by methods
        "start_line": node.start_point[0] + 1,
        "end_line": node.end_point[0] + 1,
    }

    # Extract fields
    for child in struct_def_node.children:
        if child.type == "field_declaration_list":
            for field_node in child.children:
                if field_node.type == "field_declaration":
                    field_info = _extract_go_field(field_node)
                    if field_info:
                        struct_info["fields"].append(field_info)

    return struct_info


def _extract_go_field(node) -> Dict[str, Any]:
    """Extract information about a Go struct field."""
    field_info = {"start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get field name
    for child in node.children:
        if child.type == "field_identifier":
            field_info["name"] = child.text.decode("utf-8")
            break

    # Get field type
    for child in node.children:
        if child.type == "type_identifier":
            field_info["type"] = child.text.decode("utf-8")
            break
        elif child.type == "pointer_type":
            for ptr_child in child.children:
                if ptr_child.type == "type_identifier":
                    field_info["type"] = "*" + ptr_child.text.decode("utf-8")
                    break

    return field_info


def _extract_go_function(node) -> Dict[str, Any]:
    """Extract information about a Go function."""
    # Find the function name
    func_name = None
    for child in node.children:
        if child.type == "identifier":
            func_name = child.text.decode("utf-8")
            break

    if not func_name:
        return {}

    func_info = {"name": func_name, "start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get parameters
    params = []
    for child in node.children:
        if child.type == "parameter_list":
            for param_child in child.children:
                if param_child.type == "parameter_declaration":
                    for pc in param_child.children:
                        if pc.type == "identifier":
                            params.append(pc.text.decode("utf-8"))

    if params:
        func_info["parameters"] = params

    # Get return type
    for child in node.children:
        if child.type == "result":
            for rc in child.children:
                if rc.type == "type_identifier":
                    func_info["return_type"] = rc.text.decode("utf-8")
                    break

    return func_info


def _extract_go_method(node) -> Dict[str, Any]:
    """Extract information about a Go method."""
    # Find the method name - Go uses field_identifier for method names
    func_name = None
    for child in node.children:
        if child.type == "field_identifier":
            func_name = child.text.decode("utf-8")
            break

    if not func_name:
        return {}

    # Create a basic method info with name and positions
    method_info = {
        "name": func_name,
        "start_line": node.start_point[0] + 1,  # 0-indexed to 1-indexed
        "end_line": node.end_point[0] + 1,  # 0-indexed to 1-indexed
    }

    # Get parameters
    params = []
    for child in node.children:
        if (
            child.type == "parameter_list" and child != node.children[1]
        ):  # Skip the first parameter_list which is the receiver
            for param_child in child.children:
                if param_child.type == "parameter_declaration":
                    for pc in param_child.children:
                        if pc.type == "identifier":
                            params.append(pc.text.decode("utf-8"))

    if params:
        method_info["parameters"] = params

    # Get return type
    for child in node.children:
        if child.type == "type_identifier" or child.type == "primitive_type":
            method_info["return_type"] = child.text.decode("utf-8")
            break
        elif child.type == "parameter_list" and child != node.children[1]:  # Return parameter list
            # If it's a multi-value return like (int, error)
            return_types = []
            for param_child in child.children:
                if param_child.type == "parameter_declaration":
                    for pc in param_child.children:
                        if pc.type == "type_identifier" or pc.type == "primitive_type":
                            return_types.append(pc.text.decode("utf-8"))

            if len(return_types) > 1:
                method_info["return_type"] = f"({', '.join(return_types)})"
            elif return_types:
                method_info["return_type"] = return_types[0]

    # Add receiver information
    receiver_name = _get_go_method_receiver(node)
    if receiver_name:
        method_info["receiver"] = receiver_name

    return method_info


def _get_go_method_receiver(node) -> Optional[str]:
    """Get the name of a Go method's receiver parameter."""
    for child in node.children:
        if child.type == "parameter_list":
            # The first parameter in a method declaration is the receiver
            for param_child in child.children:
                if param_child.type == "parameter_declaration":
                    for pc in param_child.children:
                        if pc.type == "identifier":
                            return pc.text.decode("utf-8")
    return None


def _get_go_method_receiver_type(node) -> Optional[str]:
    """Get the type of a Go method's receiver parameter."""
    for child in node.children:
        if child.type == "parameter_list":
            # The first parameter in a method declaration is the receiver
            for param_child in child.children:
                if param_child.type == "parameter_declaration":
                    for pc in param_child.children:
                        if pc.type == "type_identifier":
                            return pc.text.decode("utf-8")
                        elif pc.type == "pointer_type":
                            for ptr_child in pc.children:
                                if ptr_child.type == "type_identifier":
                                    return ptr_child.text.decode("utf-8")
    return None


def _extract_c_definitions(node) -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]], List[Dict[str, Any]]]:
    """Extract struct, function, and global variable definitions from C code."""
    structs = []
    functions = []
    globals_list = []

    def visit_node(node):
        if node.type == "struct_specifier":
            struct_info = _extract_c_struct(node)
            if struct_info:
                structs.append(struct_info)
        elif node.type == "function_definition":
            function_info = _extract_c_function(node)
            if function_info:
                functions.append(function_info)
        elif node.type == "declaration" and _is_global_scope(node):
            # Global variable declaration
            global_vars = _extract_c_globals(node)
            globals_list.extend(global_vars)

        for child in node.children:
            visit_node(child)

    visit_node(node)
    return structs, functions, globals_list


def _extract_c_globals(node) -> List[Dict[str, Any]]:
    """Extract global variables from C declarations."""
    globals_list = []

    # Check if this is a variable declaration and not a function prototype
    is_variable = True
    for child in node.children:
        if child.type == "function_declarator":
            is_variable = False
            break

    if not is_variable:
        return globals_list

    # Get the type of the variable
    var_type = ""
    type_qualifiers = []
    base_type = None

    for child in node.children:
        if child.type == "type_qualifier":
            type_qualifiers.append(child.text.decode("utf-8"))
        elif child.type == "primitive_type" or child.type == "type_identifier":
            base_type = child.text.decode("utf-8")

    if base_type:
        var_type = " ".join(type_qualifiers + [base_type])

    # Get the variables
    for child in node.children:
        if child.type == "init_declarator" or child.type == "declarator":
            name = None
            value = None

            # Get variable name
            if child.type == "init_declarator":
                for decl_child in child.children:
                    if decl_child.type == "declarator":
                        for id_child in decl_child.children:
                            if id_child.type == "identifier":
                                name = id_child.text.decode("utf-8")
                                break
                    elif decl_child.type == "=":
                        # There's an initializer
                        pass
                    elif decl_child.type in ["number_literal", "string_literal", "character_literal"]:
                        value = decl_child.text.decode("utf-8")
            else:  # simple declarator
                for id_child in child.children:
                    if id_child.type == "identifier":
                        name = id_child.text.decode("utf-8")
                        break

            if name:
                globals_list.append(
                    {
                        "name": name,
                        "type": var_type,
                        "value": value,
                        "start_line": node.start_point[0] + 1,
                        "end_line": node.end_point[0] + 1,
                    }
                )

    return globals_list


def _extract_c_struct(node) -> Dict[str, Any]:
    """Extract information about a C struct."""
    # Find the struct name
    struct_name = None
    for child in node.children:
        if child.type == "type_identifier":
            struct_name = child.text.decode("utf-8")
            break

    if not struct_name:
        return {}

    struct_info = {
        "name": struct_name,
        "fields": [],
        "start_line": node.start_point[0] + 1,
        "end_line": node.end_point[0] + 1,
    }

    # Extract fields
    for child in node.children:
        if child.type == "field_declaration_list":
            for field_node in child.children:
                if field_node.type == "field_declaration":
                    field_info = _extract_c_field(field_node)
                    if field_info:
                        struct_info["fields"].append(field_info)

    return struct_info


def _extract_c_field(node) -> Dict[str, Any]:
    """Extract information about a C struct field."""
    field_info = {"start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get field name
    for child in node.children:
        if child.type == "declarator":
            for decl_child in child.children:
                if decl_child.type == "identifier":
                    field_info["name"] = decl_child.text.decode("utf-8")
                    break

    # Get field type
    for child in node.children:
        if child.type == "primitive_type" or child.type == "type_identifier":
            field_info["type"] = child.text.decode("utf-8")
            break

    return field_info


def _extract_c_function(node) -> Dict[str, Any]:
    """Extract information about a C function."""
    # Find the function name
    func_name = None
    for child in node.children:
        if child.type == "function_declarator":
            for decl_child in child.children:
                if decl_child.type == "identifier":
                    func_name = decl_child.text.decode("utf-8")
                    break

    if not func_name:
        return {}

    func_info = {"name": func_name, "start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get parameters
    params = []
    param_types = []
    for child in node.children:
        if child.type == "function_declarator":
            for decl_child in child.children:
                if decl_child.type == "parameter_list":
                    params, param_types = _extract_c_parameter_list(decl_child)
                    break

    if params:
        func_info["parameters"] = params

    if param_types:
        func_info["parameter_types"] = param_types

    # Get return type
    for child in node.children:
        if child.type == "primitive_type" or child.type == "type_identifier":
            func_info["return_type"] = child.text.decode("utf-8")
            break
        elif child.type == "storage_class_specifier" and child.text.decode("utf-8") == "const":
            # Handle const return types
            for sibling in node.children:
                if sibling.type == "primitive_type" or sibling.type == "type_identifier":
                    func_info["return_type"] = "const " + sibling.text.decode("utf-8")
                    break

    return func_info


def _extract_c_parameter_list(node) -> Tuple[List[str], List[str]]:
    """
    Extract parameter names and types from a C function parameter list.

    Args:
        node: The parameter_list node

    Returns:
        Tuple containing (parameter_names, parameter_types)
    """
    param_names = []
    param_types = []

    for child in node.children:
        if child.type == "parameter_declaration":
            param_name = None
            # No need to initialize param_type here as it's not used directly

            # Extract type qualifiers (const, etc.)
            type_qualifiers = []
            for qual_node in child.children:
                if qual_node.type == "type_qualifier":
                    type_qualifiers.append(qual_node.text.decode("utf-8"))

            # Extract base type
            base_type = None
            for type_node in child.children:
                if type_node.type == "primitive_type" or type_node.type == "type_identifier":
                    base_type = type_node.text.decode("utf-8")
                    break

            # Extract parameter name
            for id_node in child.children:
                if id_node.type == "identifier":
                    param_name = id_node.text.decode("utf-8")
                    break

            # Check for pointer types
            is_pointer = False
            pointer_qualifiers = []

            for decl_node in child.children:
                if decl_node.type == "pointer_declarator":
                    is_pointer = True
                    # Check for const pointer
                    for ptr_qual in decl_node.children:
                        if ptr_qual.type == "type_qualifier":
                            pointer_qualifiers.append(ptr_qual.text.decode("utf-8"))

                    # Get the identifier from the pointer declarator
                    for ptr_id in decl_node.children:
                        if ptr_id.type == "identifier":
                            param_name = ptr_id.text.decode("utf-8")
                            break
                        elif ptr_id.type == "pointer_declarator":
                            # Handle double pointers
                            is_pointer = "**"
                            for ptr_ptr_id in ptr_id.children:
                                if ptr_ptr_id.type == "identifier":
                                    param_name = ptr_ptr_id.text.decode("utf-8")
                                    break

            # Construct full type string
            if base_type:
                full_type = " ".join(type_qualifiers + [base_type])
                if is_pointer:
                    if is_pointer == "**":
                        full_type += " **"
                    else:
                        full_type += " *"
                    if pointer_qualifiers:
                        full_type += " " + " ".join(pointer_qualifiers)

                param_types.append(full_type.strip())

            if param_name:
                param_names.append(param_name)
            else:
                # If no name is found, use a placeholder
                param_names.append("_unnamed_param_" + str(len(param_names)))

    return param_names, param_types


def _extract_rust_definitions(node) -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]], List[Dict[str, Any]]]:
    """Extract struct, function, and global variable definitions from Rust code."""
    structs = []
    functions = []
    globals_list = []

    def visit_node(node):
        if node.type == "struct_item":
            struct_info = _extract_rust_struct(node)
            if struct_info:
                structs.append(struct_info)
        elif node.type == "function_item":
            function_info = _extract_rust_function(node)
            if function_info:
                functions.append(function_info)
        elif node.type == "impl_item":
            _extract_rust_impl(node, structs)
        elif node.type == "const_item" or node.type == "static_item":
            # Capture global constants and static variables
            global_var = _extract_rust_global(node)
            if global_var:
                globals_list.append(global_var)

        # Special case for searching global variables that are not within a function or struct
        elif node.type == "let_declaration" and _is_global_scope(node):
            global_var = _extract_rust_let_global(node)
            if global_var:
                globals_list.append(global_var)

        for child in node.children:
            visit_node(child)

    visit_node(node)
    return structs, functions, globals_list


def _is_global_scope(node):
    """Check if a node is in the global scope (not within a function, class, method, etc.)."""
    parent = node.parent
    while parent:
        if parent.type in [
            # Rust scopes
            "function_item",
            "struct_item",
            "impl_item",
            # Python scopes
            "function_definition",
            "class_definition",
            # Java scopes
            "method_declaration",
            "class_declaration",
            # JavaScript scopes
            "method_definition",
            "function_declaration",
            # C/Go scopes - already covered by function checks
        ]:
            return False
        parent = parent.parent
    return True


def _extract_rust_global(node) -> Dict[str, Any]:
    """Extract information about a Rust global constant or static variable."""
    name = None
    var_type = None
    value = None

    # Find the variable name
    for child in node.children:
        if child.type == "identifier":
            name = child.text.decode("utf-8")
            break

    # Find the type
    for child in node.children:
        if child.type == "type_identifier" or child.type == "primitive_type":
            var_type = child.text.decode("utf-8")
            break

    # Find the value (if any)
    for child in node.children:
        if child.type == "integer_literal" or child.type == "float_literal" or child.type == "string_literal":
            value = child.text.decode("utf-8")
            break

    if not name:
        return {}

    return {
        "name": name,
        "type": var_type,
        "value": value,
        "start_line": node.start_point[0] + 1,
        "end_line": node.end_point[0] + 1,
    }


def _extract_rust_let_global(node) -> Dict[str, Any]:
    """Extract information about a Rust let declaration in global scope."""
    name = None
    var_type = None
    value = None

    # Find the variable name
    for child in node.children:
        if child.type == "identifier":
            name = child.text.decode("utf-8")
            break

    # Find the type (if explicitly specified)
    for child in node.children:
        if child.type == "type_annotation":
            for type_child in child.children:
                if type_child.type == "type_identifier" or type_child.type == "primitive_type":
                    var_type = type_child.text.decode("utf-8")
                    break

    # Find the value (if any)
    has_value = False
    for child in node.children:
        if child.type == "=":
            has_value = True
        elif has_value and child.type in ["integer_literal", "float_literal", "string_literal", "boolean_literal"]:
            value = child.text.decode("utf-8")
            break

    if not name:
        return {}

    return {
        "name": name,
        "type": var_type,
        "value": value,
        "start_line": node.start_point[0] + 1,
        "end_line": node.end_point[0] + 1,
    }


def _extract_rust_struct(node) -> Dict[str, Any]:
    """Extract information about a Rust struct."""
    # Find the struct name
    struct_name = None
    for child in node.children:
        if child.type == "type_identifier":
            struct_name = child.text.decode("utf-8")
            break

    if not struct_name:
        return {}

    struct_info = {
        "name": struct_name,
        "fields": [],
        "methods": [],  # Will be populated by _extract_rust_impl
        "start_line": node.start_point[0] + 1,
        "end_line": node.end_point[0] + 1,
    }

    # Extract fields
    for child in node.children:
        if child.type == "field_declaration_list":
            for field_node in child.children:
                if field_node.type == "field_declaration":
                    field_info = _extract_rust_field(field_node)
                    if field_info:
                        struct_info["fields"].append(field_info)

    return struct_info


def _extract_rust_field(node) -> Dict[str, Any]:
    """Extract information about a Rust struct field."""
    field_info = {"start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get field name
    for child in node.children:
        if child.type == "field_identifier":
            field_info["name"] = child.text.decode("utf-8")
            break

    # Get field type
    for child in node.children:
        if child.type == "type_identifier" or child.type == "primitive_type":
            field_info["type"] = child.text.decode("utf-8")
            break

    return field_info


def _extract_rust_function(node) -> Dict[str, Any]:
    """Extract information about a Rust function."""
    # Find the function name
    func_name = None
    for child in node.children:
        if child.type == "identifier":
            func_name = child.text.decode("utf-8")
            break

    if not func_name:
        return {}

    func_info = {"name": func_name, "start_line": node.start_point[0] + 1, "end_line": node.end_point[0] + 1}

    # Get parameters and their types
    params = []
    param_types = []
    for child in node.children:
        if child.type == "parameters":
            for param_child in child.children:
                if param_child.type == "parameter":
                    param_name = None
                    param_type = None

                    # Extract parameter name and type from the parameter node
                    param_name, param_type = _extract_rust_parameter(param_child)

                    if param_name:
                        params.append(param_name)
                        param_types.append(param_type)

    if params:
        func_info["parameters"] = params
        func_info["parameter_types"] = param_types

    # Get return type
    for child in node.children:
        if child.type == "return_type":
            for rt_child in child.children:
                if rt_child.type == "type_identifier" or rt_child.type == "primitive_type":
                    func_info["return_type"] = rt_child.text.decode("utf-8")
                    break
                elif rt_child.type == "reference_type":
                    # Handle reference return types
                    ref_text = "&"
                    for ref_child in rt_child.children:
                        if ref_child.type == "mutable_specifier":
                            ref_text += " mut "
                        elif ref_child.type in ["type_identifier", "primitive_type"]:
                            ref_text += ref_child.text.decode("utf-8")
                    func_info["return_type"] = ref_text
                    break

    return func_info


def _extract_rust_parameter(node) -> Tuple[Optional[str], Optional[str]]:
    """
    Extract the name and type of a Rust function parameter.

    Args:
        node: The parameter node

    Returns:
        A tuple containing (parameter_name, parameter_type)
    """
    param_name = None
    param_type = None

    # Direct examination of the parameter node text for patterns like "result: f64"
    full_text = node.text.decode("utf-8")
    if ": " in full_text:
        parts = full_text.split(": ", 1)
        param_name = parts[0].strip()
        param_type = parts[1].strip()

        # Clean up reference types
        if param_type.startswith("&"):
            if "mut " in param_type:
                param_type = "&mut " + param_type.split("mut ")[1]
            # Leave simple references as they are: "&str", etc.

        return param_name, param_type

    # If the pattern matching fails, fall back to the node-based approach
    for child in node.children:
        if child.type == "identifier":
            param_name = child.text.decode("utf-8")

        elif child.type == "type_annotation":
            # Extract from type annotation
            for type_child in child.children:
                if type_child.type in ["type_identifier", "primitive_type"]:
                    param_type = type_child.text.decode("utf-8")
                    break

                elif type_child.type == "reference_type":
                    ref_text = "&"
                    # Check for mutability
                    for ref_child in type_child.children:
                        if ref_child.type == "mutable_specifier":
                            ref_text += "mut "
                        elif ref_child.type in ["type_identifier", "primitive_type"]:
                            ref_text += ref_child.text.decode("utf-8")
                    param_type = ref_text
                    break

    return param_name, param_type


def _extract_rust_impl(node, structs) -> None:
    """Extract methods from a Rust impl block and add them to the corresponding struct."""
    # Find the struct this impl is for
    impl_for = None
    for child in node.children:
        if child.type == "type_identifier":
            impl_for = child.text.decode("utf-8")
            break

    if not impl_for:
        return

    # Find the matching struct in our list
    target_struct = None
    for struct in structs:
        if struct["name"] == impl_for:
            target_struct = struct
            break

    if not target_struct:
        # Create a placeholder struct if we haven't seen it yet
        target_struct = {
            "name": impl_for,
            "fields": [],
            "methods": [],
            "start_line": node.start_point[0] + 1,
            "end_line": node.end_point[0] + 1,
        }
        structs.append(target_struct)

    # Extract methods
    for child in node.children:
        if child.type == "implementation_body":
            for body_child in child.children:
                if body_child.type == "function_item":
                    method_info = _extract_rust_function(body_child)
                    if method_info:
                        target_struct["methods"].append(method_info)


def _get_parents(node):
    """Get all parent nodes of a given node."""
    parents = []
    current = node.parent

    while current:
        parents.append(current)
        current = current.parent

    return parents


@mcp.tool
def get_file_structure(language: str, file_path: str) -> Dict[str, Any]:
    """Extract a high-level language-agnostic skeleton of a source code file.

    This tool analyzes source code files and creates a structured representation
    of their contents, identifying key elements like classes, functions, methods,
    imports, and global variables. For Go code, it also properly identifies structs.
    The analysis is performed using tree-sitter parsers for accurate language-specific parsing.

    Args:
        language (str): The programming language of the file. Must be one of:
                        'python', 'java', 'javascript' (or 'js'), 'go', 'c', or 'rust'.
        file_path (str): Path to the source code file to analyze. Can be relative or absolute.

    Returns:
        Dict[str, Any]: A hierarchical dictionary containing the file structure with:
            - file_path: The path to the analyzed file
            - language: The detected language
            - skeleton: The main structure containing:
                - imports: List of import statements
                - classes: List of classes with their methods and fields
                - functions: List of top-level functions with parameters and return types
                - globals: List of global variables
                - structs: (Go only) List of structs with their methods and fields

    Raises:
        ValueError: If an unsupported language is specified
        FileNotFoundError: If the specified file doesn't exist
        UnicodeDecodeError: If the file contains invalid encoding

    Example:
        >>> structure = get_file_structure("python", "/path/to/script.py")
        >>> print(f"Found {len(structure['skeleton']['functions'])} functions")
        Found 3 functions
        >>> print(f"Found {len(structure['skeleton']['classes'])} classes")
        Found 2 classes

    Note:
        - Each element includes source location (start_line, end_line)
        - Methods are associated with their respective classes
        - For Go, structs are separated from classes in their own section
        - Parameter types are included when available
    """
    result = extract_file_skeleton(language, file_path)
    return result


@mcp.tool
def get_directory_tree(path: str, print_dirs_only: bool) -> str:
    """Generate a visual tree representation of a directory structure.

    This tool recursively traverses the specified directory and creates
    a hierarchical tree view using ASCII characters. The output shows
    files and subdirectories in a visually organized format similar
    to the Unix 'tree' command.

    Args:
        path (str): The file system path to the directory to analyze.
                   Can be relative or absolute path. The path will be
                   converted to absolute path internally.
        print_dirs_only (bool): If True, only directories will be shown
                               in the tree structure, files will be excluded.
                               This parameter is required.

    Returns:
        str: A formatted string containing the directory tree structure
             using ASCII art characters (├──, └──, │). Returns an error
             message if the path doesn't exist, isn't a directory, or
             access is denied.

    Raises:
        No exceptions are raised directly. All errors are caught and
        returned as formatted error messages in the output string.

    Example:
        >>> get_directory_tree("/home/user/project")
        project/
        ├── README.md
        ├── src/
        │   ├── main.py
        │   └── utils/
        │       └── helpers.py
        └── tests/
            └── test_main.py

        >>> get_directory_tree("/home/user/project", print_dirs_only=True)
        project/
        ├── src/
        │   └── utils/
        └── tests/

    Note:
        - Directories are traversed recursively
        - Items are sorted alphabetically for consistent output
        - Permission errors are handled gracefully
        - Hidden files and directories (starting with '.') are excluded from output
        - When print_dirs_only=True, only directories are shown in the tree
    """
    import os

    def generate_tree(directory, prefix="", is_last=True):
        """Recursively generate tree structure."""
        if not os.path.exists(directory):
            return f"Error: Path '{directory}' does not exist"

        if not os.path.isdir(directory):
            return f"Error: Path '{directory}' is not a directory"

        tree_lines = []

        try:
            # Get all items in the directory
            items = sorted(os.listdir(directory))
            # Filter out hidden directories (starting with '.')
            items = [item for item in items if not item.startswith(".")]

            # Filter to only directories if print_dirs_only is True
            if print_dirs_only:
                items = [item for item in items if os.path.isdir(os.path.join(directory, item))]

            for i, item in enumerate(items):
                item_path = os.path.join(directory, item)
                is_last_item = i == len(items) - 1

                # Create the tree symbols
                current_prefix = "└── " if is_last_item else "├── "
                tree_lines.append(f"{prefix}{current_prefix}{item}")

                # If it's a directory, recursively add its contents
                if os.path.isdir(item_path):
                    extension_prefix = "    " if is_last_item else "│   "
                    subtree = generate_tree(item_path, prefix + extension_prefix, is_last_item)
                    if subtree:  # Only add if there's content
                        tree_lines.append(subtree)

        except PermissionError:
            tree_lines.append(f"{prefix}[Permission Denied]")
        except Exception as e:
            tree_lines.append(f"{prefix}[Error: {str(e)}]")

        return "\n".join(tree_lines)

    # Get the absolute path and directory name
    abs_path = os.path.abspath(path)
    dir_name = os.path.basename(abs_path)

    # Start the tree with the root directory
    result = f"{dir_name}/\n"
    tree_content = generate_tree(abs_path)

    if tree_content and not tree_content.startswith("Error:"):
        result += tree_content
    else:
        result = tree_content  # Return error message directly

    return result


if __name__ == "__main__":
    mcp.run(transport="stdio")
