FROM python:3.12.9

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV NODE_PATH=/usr/local/lib/node_modules

# Install required system packages
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    build-essential \
    git \
    vim \
    nano \
    less \
    graphviz \
    tmux \
    bear \
    && rm -rf /var/lib/apt/lists/*

# Install Go 1.24.4
RUN wget https://go.dev/dl/go1.24.4.linux-amd64.tar.gz \
    && tar -C /usr/local -xzf go1.24.4.linux-amd64.tar.gz \
    && rm go1.24.4.linux-amd64.tar.gz \
    && ls -la /usr/local/go/bin/go || (echo "Go binary not found at /usr/local/go/bin/go" && exit 1)

# Install Rust 1.92.0 (nightly-2025-09-15)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain none \
    && . "$HOME/.cargo/env" \
    && rustup toolchain install nightly-2025-09-15 \
    && rustup default nightly-2025-09-15

# Install Java 21 (Amazon Corretto)
RUN wget https://corretto.aws/downloads/latest/amazon-corretto-21-x64-linux-jdk.tar.gz \
    && tar -C /usr/local -xzf amazon-corretto-21-x64-linux-jdk.tar.gz \
    && rm amazon-corretto-21-x64-linux-jdk.tar.gz \
    && mv /usr/local/amazon-corretto-* /usr/local/java \
    && ls -la /usr/local/java/bin/java || (echo "Java binary not found at /usr/local/java/bin/java" && exit 1)

# Install Maven 3.9.9
RUN wget https://archive.apache.org/dist/maven/maven-3/3.9.9/binaries/apache-maven-3.9.9-bin.tar.gz \
    && tar -C /usr/local -xzf apache-maven-3.9.9-bin.tar.gz \
    && rm apache-maven-3.9.9-bin.tar.gz \
    && mv /usr/local/apache-maven-3.9.9 /usr/local/maven \
    && ls -la /usr/local/maven/bin/mvn || (echo "Maven binary not found at /usr/local/maven/bin/mvn" && exit 1)

# Download & unpack exactly Node.js v22.16.0
RUN curl -fsSL https://nodejs.org/dist/v22.16.0/node-v22.16.0-linux-x64.tar.xz \
     | tar -xJ -C /usr/local --strip-components=1 \
     && npm install -g npm@11.4.0

# Update PATH to include all tools
ENV PATH="/usr/local/go/bin:/root/.cargo/bin:/usr/local/java/bin:/usr/local/maven/bin:/go/bin:${PATH}"
ENV JAVA_HOME="/usr/local/java"
ENV MAVEN_HOME="/usr/local/maven"
ENV GOPATH="/go"
ENV GOROOT="/usr/local/go"

# Create Go workspace
RUN mkdir -p /go/bin /go/src /go/pkg

# Verify installations
RUN node -v && npm -v \
    && go version \
    && rustc --version \
    && java -version \
    && mvn -version \
    && python --version \
    && gcc --version

# Install Node.js packages globally
RUN npm install -g \
    crc-32@1.2.2 \
    numjs@0.16.1 \
    @anthropic-ai/claude-code@2.1.19

# Install Python packages
RUN pip install --no-cache-dir \
    tree-sitter==0.24.0 \
    tree-sitter-languages==1.10.2 \
    tree-sitter-c==0.23.4 \
    tree-sitter-go==0.23.4 \
    tree-sitter-javascript==0.23.1 \
    tree-sitter-python==0.23.6 \
    tree-sitter-rust==0.23.2 \
    tree-sitter-java==0.23.5 \
    openai==1.98.0 \
    boto3==1.38.20 \
    pandas==2.2.3 \
    fastmcp==2.12.5 \
    graphviz==0.20.3 \
    matplotlib==3.10.5 \
    networkx==3.5 \
    seaborn==0.13.2 \
    scikit-learn==1.7.1 \
    pyyaml==6.0.2 \
    jinja2==3.1.6 \
    astor==0.8.1 \
    pydot==3.0.4 \
    tqdm==4.67.1 \
    pebble==5.1.3 \
    attrs==25.3.0 \
    pytest==9.0.1

# Define ARG parameters for tool_name and project_name
ARG TOOL_NAME
ARG PROJECT_NAME
ARG SOURCE_LANGUAGE
ARG TARGET_LANGUAGE

# Set labels for the image
LABEL tool_name=${TOOL_NAME}
LABEL project_name=${PROJECT_NAME}

# Copy project snapshot into image, but only specific data for the tool and project
COPY .claude /workspace/.claude
COPY configs/prompt_templates.yaml /workspace/configs/prompt_templates.yaml
# First copy all project files to a temporary directory
COPY data/tool_projects/${TOOL_NAME}/${PROJECT_NAME} /tmp/project_files

# Create the target directory
RUN mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}

# Handle project directories based on tool type
RUN if [ "${TOOL_NAME}" = "crust" ]; then \
        # For crust: Copy both source and target languages inside project dir (default behavior) \
        cp -r /tmp/project_files/* /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/; \
    elif [ "${TOOL_NAME}" = "alphatrans" ]; then \
        # For alphatrans: Copy only java and create an empty python dir \
        mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/java && \
        mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/python && \
        cp -r /tmp/project_files/java/* /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/java/ || true; \
    elif [ "${TOOL_NAME}" = "skel" ]; then \
        # For skel: Copy python and create empty javascript \
        mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/python && \
        mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/javascript && \
        cp -r /tmp/project_files/python/* /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/python/ || true; \
    elif [ "${TOOL_NAME}" = "oxidizer" ]; then \
        # For oxidizer: Copy go and create empty rust dir \
        mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/go && \
        mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/rust && \
        cp -r /tmp/project_files/go/* /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/go/ || true; \
    else \
        # Default behavior for any other tool: copy everything \
        cp -r /tmp/project_files/* /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/; \
    fi && \
    # Create planning directory for all tools \
    mkdir -p /workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/planning && \
    # Clean up temporary directory \
    rm -rf /tmp/project_files
COPY scripts /workspace/scripts
COPY src /workspace/src
COPY .gitignore /workspace/.gitignore
COPY .gitattributes /workspace/.gitattributes
COPY CLAUDE.md /workspace/CLAUDE.md
COPY README.md /workspace/README.md

# Generate configs directory
RUN mkdir -p /workspace/configs

# Generate configs/claude_mcp_config.json file
RUN echo '{\n  "mcpServers": {' > /workspace/configs/claude_mcp_config.json && \
    if [ "$SOURCE_LANGUAGE" = "rust" ] || [ "$TARGET_LANGUAGE" = "rust" ]; then \
        echo '    "rust-language-server": {\n\
      "command": "mcp-language-server",\n\
      "args": [\n\
        "--workspace",\n\
        "/workspace/data/tool_projects/'${TOOL_NAME}'/'${PROJECT_NAME}'/rust",\n\
        "--lsp",\n\
        "rust-analyzer"\n\
      ]\n\
    },' >> /workspace/configs/claude_mcp_config.json; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "c" ] || [ "$TARGET_LANGUAGE" = "c" ]; then \
        echo '    "c-language-server": {\n\
      "command": "mcp-language-server",\n\
      "args": [\n\
        "--workspace",\n\
        "/workspace/data/tool_projects/'${TOOL_NAME}'/'${PROJECT_NAME}'/c",\n\
        "--lsp",\n\
        "/usr/bin/clangd",\n\
        "--",\n\
        "--compile-commands-dir=/workspace/data/tool_projects/'${TOOL_NAME}'/'${PROJECT_NAME}'/c"\n\
      ]\n\
    },' >> /workspace/configs/claude_mcp_config.json; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "python" ] || [ "$TARGET_LANGUAGE" = "python" ]; then \
        echo '    "python-language-server": {\n\
      "command": "mcp-language-server",\n\
      "args": [\n\
        "--workspace",\n\
        "/workspace/data/tool_projects/'${TOOL_NAME}'/'${PROJECT_NAME}'/python",\n\
        "--lsp",\n\
        "pylsp"\n\
      ]\n\
    },' >> /workspace/configs/claude_mcp_config.json; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "javascript" ] || [ "$TARGET_LANGUAGE" = "javascript" ] || \
       [ "$SOURCE_LANGUAGE" = "typescript" ] || [ "$TARGET_LANGUAGE" = "typescript" ]; then \
        echo '    "javascript-language-server": {\n\
      "command": "mcp-language-server",\n\
      "args": [\n\
        "--workspace",\n\
        "/workspace/data/tool_projects/'${TOOL_NAME}'/'${PROJECT_NAME}'/'${TARGET_LANGUAGE}'",\n\
        "--lsp",\n\
        "typescript-language-server",\n\
        "--",\n\
        "--stdio"\n\
      ]\n\
    },' >> /workspace/configs/claude_mcp_config.json; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "java" ] || [ "$TARGET_LANGUAGE" = "java" ]; then \
        echo '    "java-language-server": {\n\
      "command": "mcp-language-server",\n\
      "args": [\n\
        "--workspace",\n\
        "/workspace/data/tool_projects/'${TOOL_NAME}'/'${PROJECT_NAME}'/java",\n\
        "--lsp",\n\
        "/usr/local/jdt-language-server/bin/jdtls",\n\
        "--",\n\
        "--stdio",\n\
        "-configuration",\n\
        "/usr/local/jdt-language-server/config_linux",\n\
        "-data",\n\
        "/.cache/jdtls-data"\n\
      ],\n\
      "env": {\n\
        "JAVA_HOME": "/usr/local/java"\n\
      }\n\
    },' >> /workspace/configs/claude_mcp_config.json; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "go" ] || [ "$TARGET_LANGUAGE" = "go" ]; then \
        echo '    "go-language-server": {\n\
      "command": "mcp-language-server",\n\
      "args": [\n\
        "--workspace",\n\
        "/workspace/data/tool_projects/'${TOOL_NAME}'/'${PROJECT_NAME}'/go",\n\
        "--lsp",\n\
        "gopls"\n\
      ],\n\
      "env": {\n\
        "PATH": "/usr/local/go/bin:/go/bin:${PATH}",\n\
        "GOPATH": "/go",\n\
        "GOCACHE": "/.cache/go-build",\n\
        "GOMODCACHE": "/go/pkg/mod"\n\
      }\n\
    },' >> /workspace/configs/claude_mcp_config.json; \
    fi && \
    echo '    "project-analyzer": {\n\
      "command": "python",\n\
      "args": ["/workspace/src/mcp/mcp_project_analyzer/project_analyzer.py"],\n\
      "env": {}\n\
    }' >> /workspace/configs/claude_mcp_config.json && \
    if [ "${TOOL_NAME}" = "crust" ]; then \
        echo ',\n\
    "crust-test-runner": {\n\
      "command": "python",\n\
      "args": ["/workspace/src/mcp/mcp_crust_test_runner/test_runner.py"],\n\
      "env": {}\n\
    }' >> /workspace/configs/claude_mcp_config.json; \
    fi && \
    echo '\n\
  }\n\
}' >> /workspace/configs/claude_mcp_config.json

# Generate recodeagent YAML config file with dynamic values
RUN echo "agent_name: \"recodeagent\"\n\
mcp_config_file: \"configs/claude_mcp_config.json\"\n\
project_name: \"${PROJECT_NAME}\"\n\
source_language: \"${SOURCE_LANGUAGE}\"\n\
target_language: \"${TARGET_LANGUAGE}\"\n\
tool_name: \"${TOOL_NAME}\"\n\
source_project_root: \"data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/${SOURCE_LANGUAGE}\"\n\
target_translation_root: \"data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/${TARGET_LANGUAGE}\"\n\
planning_dir: \"data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/planning\"\n\
extra_agent_args: [\"--output-format\", \"stream-json\", \"--verbose\", \"--settings\", \"/workspace/.claude/settings.local.json\"]\n\
extra_translator_agent_args: [\"--agents\", '{\"source-translator-executor\": {\"description\": \"Translates and executes source code from PART A of implementation-plan.md\", \"prompt\": \"You are an agent which performs source translation and execution from PART A of implementation-plan.md. This agent will ONLY translate and execute one source file and then terminates. The execution involves compiling the source file to ensure it is valid.\"}, \"test-translator-executor\": {\"description\": \"Translates and executes tests from PART B of implementation-plan.md\", \"prompt\": \"You are an agent which performs test translation and execution from PART B of implementation-plan.md. This agent will ONLY translate and execute one test class and then terminates. CRITICAL REQUIREMENTS: 1) First execute the source test in the source language to verify it passes and count the number of tests executed. 2) Translate the test to the target language. 3) Execute the translated test in the target language. 4) Verify that the same number of tests are executed in both source and target languages. 5) Verify that all tests pass in both languages. 6) If source tests pass but target tests fail, fix the target implementation or translation. The execution involves compiling and running the test file in BOTH languages to ensure functional equivalence.\"}}']\n\
extra_validator_agent_args: [\"--agents\", '{\"test-generator-executor\": {\"description\": \"Generates and executes tests for a single class in both source and target languages to verify functional equivalence\", \"prompt\": \"You are an agent which generates tests for a SINGLE CLASS to verify functional equivalence between the source and target implementations. For the given class: 1) Identify all uncovered methods/functions in that class. 2) Generate a test file named ClassNameTest_generated in the source language that tests all uncovered methods. 3) Generate an identical test file named ClassNameTest_generated in the target language with the same test logic. 4) Execute tests in BOTH languages and compare results. 5) If tests pass in source but fail in target, fix the target implementation. 6) Report pass/fail status for both languages. This agent terminates after completing tests for ONE class.\"}}']\n\
analyzer_timeout: 5000\n\
planning_timeout: 5000\n\
translator_timeout: 10000\n\
validator_timeout: 5000\n\
only_agents: [\"analyzer\", \"planning\", \"translator\", \"validator\"]\n\
skip_agents: []\n\
vendor: \"anthropic\"" > /workspace/configs/recodeagent_${TOOL_NAME}_${PROJECT_NAME}_${SOURCE_LANGUAGE}_${TARGET_LANGUAGE}.yaml


# Init Git repo with full project snapshot
WORKDIR /workspace
RUN git init --initial-branch=main \
    && git config user.email "agent@example.com" \
    && git config user.name "agent" \
    && git add . \
    && git commit -m "Initial commit with full project snapshot"

# Configure shell with basic coloring and PATH
RUN echo 'export TERM=xterm-256color' >> /root/.bashrc \
    && echo 'export PATH="/usr/local/go/bin:/root/.cargo/bin:/usr/local/java/bin:/usr/local/maven/bin:/go/bin:$PATH"' >> /root/.bashrc \
    && echo 'export JAVA_HOME="/usr/local/java"' >> /root/.bashrc \
    && echo 'export GOPATH="/go"' >> /root/.bashrc \
    && echo 'export GOROOT="/usr/local/go"' >> /root/.bashrc \
    && echo 'alias ls="ls --color=auto"' >> /root/.bashrc \
    && echo 'alias grep="grep --color=auto"' >> /root/.bashrc \
    && echo 'PS1="\[\033[1;36m\]\u\[\033[1;31m\]@\[\033[1;32m\]\h:\[\033[1;35m\]\w\[\033[1;31m\]\$\[\033[0m\] "' >> /root/.bashrc

# install mcp-language-server from src/mcp/mcp_language_server
RUN cd /workspace/src/mcp/mcp_language_server \
    && go install .

# Install language servers based on SOURCE_LANGUAGE and TARGET_LANGUAGE
RUN if [ "$SOURCE_LANGUAGE" = "go" ] || [ "$TARGET_LANGUAGE" = "go" ]; then \
        go install golang.org/x/tools/gopls@latest; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "rust" ] || [ "$TARGET_LANGUAGE" = "rust" ]; then \
        rustup component add rust-analyzer; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "python" ] || [ "$TARGET_LANGUAGE" = "python" ]; then \
        pip install python-lsp-server[all]; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "typescript" ] || [ "$TARGET_LANGUAGE" = "typescript" ] || \
       [ "$SOURCE_LANGUAGE" = "javascript" ] || [ "$TARGET_LANGUAGE" = "javascript" ]; then \
        npm install -g typescript typescript-language-server; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "c" ] || [ "$TARGET_LANGUAGE" = "c" ] || \
       [ "$SOURCE_LANGUAGE" = "cpp" ] || [ "$TARGET_LANGUAGE" = "cpp" ]; then \
        apt-get update && apt-get install -y clangd && rm -rf /var/lib/apt/lists/*; \
    fi && \
    if [ "$SOURCE_LANGUAGE" = "java" ] || [ "$TARGET_LANGUAGE" = "java" ]; then \
        wget https://www.eclipse.org/downloads/download.php?file=/jdtls/snapshots/jdt-language-server-latest.tar.gz -O /tmp/jdt-language-server.tar.gz && \
        mkdir -p /usr/local/jdt-language-server && \
        tar -xzf /tmp/jdt-language-server.tar.gz -C /usr/local/jdt-language-server && \
        rm /tmp/jdt-language-server.tar.gz; \
    fi

# Run bear -- make for crust projects to generate compile_commands.json
RUN if [ "${TOOL_NAME}" = "crust" ]; then \
        MAKE_TARGET=""; \
        case "${PROJECT_NAME}" in \
            2dpartint) MAKE_TARGET="test" ;; \
            42-kocaeli-printf) MAKE_TARGET="test" ;; \
            aes128-simd) MAKE_TARGET="test" ;; \
            amp) MAKE_TARGET="test" ;; \
            approxidate) MAKE_TARGET="test" ;; \
            avalanche) MAKE_TARGET="all" ;; \
            bhshell) MAKE_TARGET="test" ;; \
            bigint) MAKE_TARGET="test" ;; \
            bitset) MAKE_TARGET="test" ;; \
            blt) MAKE_TARGET="blt_test" ;; \
            bostree) MAKE_TARGET="test" ;; \
            btree-map) MAKE_TARGET="test" ;; \
            c-aces) MAKE_TARGET="test" ;; \
            c-blind-rsa-signatures) MAKE_TARGET="all" ;; \
            c-string) MAKE_TARGET="test" ;; \
            carrays) MAKE_TARGET="test" ;; \
            cfsm) MAKE_TARGET="test" ;; \
            chtrie) MAKE_TARGET="test" ;; \
            circularbuffer) MAKE_TARGET="test" ;; \
            cissy) MAKE_TARGET="all" ;; \
            cjson) MAKE_TARGET="test" ;; \
            clhash) MAKE_TARGET="test" ;; \
            clog) MAKE_TARGET="test" ;; \
            coroutine) MAKE_TARGET="test" ;; \
            cset) MAKE_TARGET="test" ;; \
            csyncmers) MAKE_TARGET="all" ;; \
            dict) MAKE_TARGET="all" ;; \
            emlang) MAKE_TARGET="test" ;; \
            expr) MAKE_TARGET="test" ;; \
            fasthamming) MAKE_TARGET="test" ;; \
            fft) MAKE_TARGET="test" ;; \
            file2str) MAKE_TARGET="test" ;; \
            fs_c) MAKE_TARGET="test" ;; \
            fslib) MAKE_TARGET="all" ;; \
            genetic-neural-network-for-simple-control) MAKE_TARGET="all" ;; \
            geofence) MAKE_TARGET="test" ;; \
            gorilla-paper-encode) MAKE_TARGET="test" ;; \
            graph-recogniser) MAKE_TARGET="all" ;; \
            hamta) MAKE_TARGET="bin/libhamta.so" ;; \
            holdem-odds) MAKE_TARGET="test" ;; \
            hydra) MAKE_TARGET="test" ;; \
            impcheck) MAKE_TARGET="test_full" ;; \
            inversion_list) MAKE_TARGET="all" ;; \
            jccc) MAKE_TARGET="test" ;; \
            kairocompiler) MAKE_TARGET="all" ;; \
            kd3) MAKE_TARGET="all" ;; \
            lambda-calculus-eval) MAKE_TARGET="test" ;; \
            leftpad) MAKE_TARGET="tests" ;; \
            lib2bit) MAKE_TARGET="test" ;; \
            libbeaufort) MAKE_TARGET="test" ;; \
            libfor) MAKE_TARGET="test" ;; \
            libm17) MAKE_TARGET="test" ;; \
            libpgn) MAKE_TARGET="test" ;; \
            libpsbt) MAKE_TARGET="test" ;; \
            libqueue) MAKE_TARGET="test" ;; \
            libvcd) MAKE_TARGET="all" ;; \
            libwecan) MAKE_TARGET="all" ;; \
            linear-algebra-c) MAKE_TARGET="default" ;; \
            ljmm) MAKE_TARGET="all" ;; \
            ltre) MAKE_TARGET="test" ;; \
            math-library-in-c) MAKE_TARGET="all" ;; \
            megalania) MAKE_TARGET="all" ;; \
            merkle-tree-c) MAKE_TARGET="all" ;; \
            morton) MAKE_TARGET="test" ;; \
            murmurhash_c) MAKE_TARGET="test" ;; \
            mvptree) MAKE_TARGET="all" ;; \
            nandc) MAKE_TARGET="test" ;; \
            phills_dht) MAKE_TARGET="all" ;; \
            quadtree) MAKE_TARGET="test" ;; \
            razz_simulation) MAKE_TARGET="test" ;; \
            rbtree-lab) MAKE_TARGET="test" ;; \
            recordmanager) MAKE_TARGET="all" ;; \
            rhbloom) MAKE_TARGET="test" ;; \
            roaring-bitmap) MAKE_TARGET="tests" ;; \
            rubiksolver) MAKE_TARGET="test" ;; \
            satc) MAKE_TARGET="test" ;; \
            simple-config) MAKE_TARGET="tst" ;; \
            simple-sparsehash) MAKE_TARGET="all" ;; \
            simplexml) MAKE_TARGET="test" ;; \
            skp) MAKE_TARGET="test" ;; \
            slothlang) MAKE_TARGET="test" ;; \
            ted) MAKE_TARGET="test" ;; \
            tisp) MAKE_TARGET="test" ;; \
            totp) MAKE_TARGET="test" ;; \
            ulidgen) MAKE_TARGET="test" ;; \
            utf8) MAKE_TARGET="test" ;; \
            vec) MAKE_TARGET="test" ;; \
            worsp) MAKE_TARGET="test" ;; \
            xopt) MAKE_TARGET="test" ;; \
            *) MAKE_TARGET="" ;; \
        esac; \
        if [ -n "$MAKE_TARGET" ]; then \
            cd /workspace/data/tool_projects/crust/${PROJECT_NAME}/c && \
            bear -- make $MAKE_TARGET || true; \
        fi; \
    fi

# Default command
CMD ["/bin/bash"]