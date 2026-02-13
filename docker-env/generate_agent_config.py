#!/usr/bin/env python3
"""Generate agent config YAML based on AGENT_NAME and other build args."""
import os
import yaml

agent = os.environ.get("AGENT_NAME", "recodeagent")
tool = os.environ.get("TOOL_NAME")
proj = os.environ.get("PROJECT_NAME")
src = os.environ.get("SOURCE_LANGUAGE")
tgt = os.environ.get("TARGET_LANGUAGE")

extra_agent_args = [
    "--output-format",
    "stream-json",
    "--verbose",
    "--settings",
    "/workspace/.claude/settings.local.json",
]
extra_translator_agent_args = (
    []
    if agent in ("baseagent-condensed")
    else [
        "--agents",
        '{"source-translator-executor": {"description": "Translates and executes source code from PART A of implementation-plan.md", "prompt": "You are an agent which performs source translation and execution from PART A of implementation-plan.md. This agent will ONLY translate and execute one source file and then terminates. The execution involves compiling the source file to ensure it is valid."}, "test-translator-executor": {"description": "Translates and executes tests from PART B of implementation-plan.md", "prompt": "You are an agent which performs test translation and execution from PART B of implementation-plan.md. This agent will ONLY translate and execute one test class and then terminates. CRITICAL REQUIREMENTS: 1) First execute the source test in the source language to verify it passes and count the number of tests executed. 2) Translate the test to the target language. 3) Execute the translated test in the target language. 4) Verify that the same number of tests are executed in both source and target languages. 5) Verify that all tests pass in both languages. 6) If source tests pass but target tests fail, fix the target implementation or translation. The execution involves compiling and running the test file in BOTH languages to ensure functional equivalence."}}',
    ]
)
extra_validator_agent_args = (
    []
    if agent in ("baseagent-condensed")
    else [
        "--agents",
        '{"structure-validator": {"description": "Validates that the translated project directory structure matches the design document", "prompt": "You are a structure validation agent. Your task is to verify the directory structure in target_translation_root matches the expected structure defined in the overall design document. Compare actual vs expected and report: missing files, missing directories, extra files (except *_generated.* test files), wrong locations, and incorrect file names. Return a structured list of all directory structure issues found. This agent terminates after completing the structure validation."}, "name-validator": {"description": "Validates that all identifier names are preserved exactly between source and target files", "prompt": "You are a name validation agent. Your task is to verify that ALL identifier names (classes, methods, functions, variables, constants) are EXACTLY the same in source and target files. DO NOT use name-mapping.json - compare files directly. For each source file, extract identifiers and compare with corresponding target file. Report any name changes including: snake_case to camelCase conversions, abbreviated names, translated names, added/removed prefixes/suffixes. Names MUST be identical strings. Return a structured list of all name preservation issues. This agent terminates after completing name validation."}, "stub-todo-validator": {"description": "Checks for unimplemented stubs and TODO comments in translated code", "prompt": "You are a stub and TODO validation agent. Your task is to search all target files for: 1) Stub markers (unimplemented!(), pass, raise NotImplementedError, throw new Error, empty function bodies). 2) TODO comments (TODO, FIXME, XXX, HACK - case insensitive). Return a structured list with file path, function name, line number, and type for each issue found. This agent terminates after completing the search."}, "rust-safety-validator": {"description": "Validates that all Rust code is safe with no unsafe blocks or raw pointers", "prompt": "You are a Rust safety validation agent. Your task is to verify ALL Rust code is safe Rust with NO unsafe blocks, raw pointer declarations, or raw pointer dereferences. Search for: unsafe keyword (blocks, functions, traits), raw pointer types (*const T, *mut T), raw pointer operations, unsafe function calls (std::ptr::*, std::mem::transmute). Return a structured list of all safety violations with file, line, violation type, and code snippet. This agent terminates after completing safety validation."}, "test-validator": {"description": "Validates that test translations have correct assertion counts and content", "prompt": "You are a test validation agent. Your task is to verify test translation correctness by comparing source and target test files. For each test: 1) Count assertions in source test (assert, assertEquals, assertTrue, expect().toBe(), etc.). 2) Count assertions in target test. 3) Compare assertion counts - must match. 4) Compare assertion content - must test same conditions. Return a structured list of issues: assertion count mismatches and wrong assertions. This agent terminates after completing test validation."}, "test-executor": {"description": "Executes all translated tests in both source and target languages", "prompt": "You are a test execution agent. Your task is to execute ALL translated tests in both source and target languages. For Python: run pytest from target_translation_root (create pytest.ini if needed). For Rust: run cargo test from target_translation_root. For other languages: use appropriate test runners. Execute tests in source language for comparison. Return structured results: test file, language, tests run, passed, failed, and error messages for failures. This agent terminates after executing all tests."}, "coverage-analyzer": {"description": "Builds a function-to-test coverage map to identify uncovered functions", "prompt": "You are a coverage analysis agent. Your task is to build a function-to-test coverage map. 1) Load all functions from source_language-functions.md. 2) Analyze each test file to identify which functions are called/tested. 3) Create coverage-map.md with: covered functions (function, file, tested by, test method), uncovered functions (function, file, reason), and summary (total, covered, uncovered, percentage). Return the list of uncovered functions. This agent terminates after creating the coverage map."}, "test-generator-executor": {"description": "Generates and executes tests for a single class in both source and target languages to verify functional equivalence", "prompt": "You are an agent which generates tests for a SINGLE CLASS to verify functional equivalence between the source and target implementations. For the given class: 1) Identify all uncovered methods/functions in that class. 2) Generate a test file named ClassNameTest_generated in the source language that tests all uncovered methods. 3) Generate an identical test file named ClassNameTest_generated in the target language with the same test logic. 4) Execute tests in BOTH languages and compare results. 5) If tests pass in source but fail in target, fix the target implementation. 6) Report pass/fail status for both languages. This agent terminates after completing tests for ONE class."}}',
    ]
)

if agent in ("baseagent-concat", "baseagent-condensed"):
    cfg = {
        "agent_name": agent,
        "mcp_config_file": "configs/claude_mcp_config.json",
        "project_name": proj,
        "source_language": src,
        "target_language": tgt,
        "tool_name": tool,
        "source_project_root": f"data/tool_projects/{tool}/{proj}/{src}",
        "target_translation_root": f"data/tool_projects/{tool}/{proj}/{tgt}",
        "planning_dir": f"data/tool_projects/{tool}/{proj}/planning",
        "extra_agent_args": extra_agent_args,
        "extra_translator_agent_args": extra_translator_agent_args,
        "extra_validator_agent_args": extra_validator_agent_args,
        "baseagent_timeout": 10000,
        "vendor": "anthropic",
    }
else:
    cfg = {
        "agent_name": "recodeagent",
        "mcp_config_file": "configs/claude_mcp_config.json",
        "project_name": proj,
        "source_language": src,
        "target_language": tgt,
        "tool_name": tool,
        "source_project_root": f"data/tool_projects/{tool}/{proj}/{src}",
        "target_translation_root": f"data/tool_projects/{tool}/{proj}/{tgt}",
        "planning_dir": f"data/tool_projects/{tool}/{proj}/planning",
        "extra_agent_args": extra_agent_args,
        "extra_translator_agent_args": extra_translator_agent_args,
        "extra_validator_agent_args": extra_validator_agent_args,
        "analyzer_timeout": 5000,
        "planning_timeout": 5000,
        "translator_timeout": 10000,
        "validator_timeout": 5000,
        "only_agents": ["analyzer", "planning", "translator", "validator"],
        "skip_agents": {
            "noplanning": ["planning"],
            "noanalyzer": ["analyzer"],
            "novalidator": ["validator"],
        }.get(agent, []),
        "vendor": "anthropic",
    }


# Use block style for top-level, flow style for lists, quoted string values (int stays unquoted)
class FlowListDumper(yaml.SafeDumper):
    pass


def _str_style(s):
    """Use single quote for JSON-like strings, double quote otherwise."""
    return "'" if isinstance(s, str) and s.strip().startswith("{") else '"'


def _represent_list(dumper, data):
    nodes = [
        (
            yaml.ScalarNode("tag:yaml.org,2002:str", x, style=_str_style(x))
            if isinstance(x, str)
            else dumper.represent_data(x)
        )
        for x in data
    ]
    return yaml.SequenceNode("tag:yaml.org,2002:seq", nodes, flow_style=True)


def _represent_mapping(dumper, data):
    """Quote string values, leave int and keys unquoted."""
    nodes = []
    for k, v in data.items():
        node_key = dumper.represent_data(k)
        node_value = dumper.represent_data(v)
        if isinstance(v, str):
            node_value = yaml.ScalarNode("tag:yaml.org,2002:str", v, style=_str_style(v))
        nodes.append((node_key, node_value))
    return yaml.MappingNode("tag:yaml.org,2002:map", nodes, flow_style=False)


FlowListDumper.add_representer(list, _represent_list)
FlowListDumper.add_representer(dict, _represent_mapping)

out_dir = os.environ.get("OUTPUT_DIR", "/workspace/configs")
out = os.path.join(out_dir, f"{agent}_{tool}_{proj}_{src}_{tgt}.yaml")
with open(out, "w") as f:
    yaml.dump(cfg, f, Dumper=FlowListDumper, default_flow_style=False, sort_keys=False, width=65536)
