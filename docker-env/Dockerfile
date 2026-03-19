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
    pytest==9.0.1 \
    transformers==5.0.0 \
    sentence-transformers==5.2.1

# Define ARG parameters for agent_name, tool_name and project_name
ARG AGENT_NAME=recodeagent
ARG TOOL_NAME
ARG PROJECT_NAME
ARG SOURCE_LANGUAGE
ARG TARGET_LANGUAGE

# Set labels for the image
LABEL agent_name=${AGENT_NAME}
LABEL tool_name=${TOOL_NAME}
LABEL project_name=${PROJECT_NAME}

# Copy project snapshot into image, but only specific data for the tool and project
COPY .claude /workspace/.claude
# For baseagent-condensed: remove mcp__* from settings.local.json permissions
COPY docker-env/prepare_baseagent_settings.py /tmp/prepare_baseagent_settings.py
RUN if [ "${AGENT_NAME}" = "baseagent-condensed" ]; then python3 /tmp/prepare_baseagent_settings.py; fi && rm -f /tmp/prepare_baseagent_settings.py
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

# noanalyzer: create 2 placeholder docs (planning reads these when analyzer is skipped).
RUN <<NOANALYZER_SCRIPT
set -e
if [ "${AGENT_NAME}" = "noanalyzer" ]; then
  P=/workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/planning
  cat > "$P/${TARGET_LANGUAGE}-overall-design.md" << 'OVERALL_EOF'
# Placeholder: Overall Design (Not Populated)

**Do not rely on this document.** It is a stub because the analyzer agent was not run (e.g. noanalyzer mode). Proceed with your task using the actual source and target project structure; infer or create design decisions as needed. Do not investigate or try to "fix" the missing content.
OVERALL_EOF
  cat > "$P/${SOURCE_LANGUAGE}-project-research.md" << 'RESEARCH_EOF'
# Placeholder: Source Project Research (Not Populated)

**Do not rely on this document.** It is a stub because the analyzer agent was not run (e.g. noanalyzer mode). Proceed using the actual source project layout and tools (e.g. directory tree, get_file_structure). Do not investigate why this file is empty.
RESEARCH_EOF
fi
NOANALYZER_SCRIPT

# noplanning: create 3 placeholder docs (translator/validator read these when planning is skipped).
RUN <<NOPLANNING_SCRIPT
set -e
if [ "${AGENT_NAME}" = "noplanning" ]; then
  P=/workspace/data/tool_projects/${TOOL_NAME}/${PROJECT_NAME}/planning
  cat > "$P/${SOURCE_LANGUAGE}-functions.md" << 'FUNCS_EOF'
# Placeholder: Functions List (Not Populated)

**Do not rely on this document.** It is a stub because the planning agent was not run (e.g. noplanning mode). Extract functions/methods from the source project yourself and proceed. Do not investigate why this file is empty.
FUNCS_EOF
  cat > "$P/implementation-plan.md" << 'PLAN_EOF'
# Placeholder: Implementation Plan (Not Populated)

**Do not rely on this document.** It is a stub because the planning agent was not run (e.g. noplanning mode). Create your own implementation order from the source/target structure and dependencies. Do not investigate why this file is empty.
PLAN_EOF
  printf '%s\n' '{"classes":{},"methods":{},"variables":{},"_placeholder":"Planning agent was not run. Do not rely on this file; create or infer name mappings from the source and target code."}' > "$P/name-mapping.json"
fi
NOPLANNING_SCRIPT

COPY scripts /workspace/scripts
COPY src /workspace/src
COPY .gitignore /workspace/.gitignore
COPY .gitattributes /workspace/.gitattributes
COPY CLAUDE.md /workspace/CLAUDE.md
COPY README.md /workspace/README.md

# Generate configs directory
RUN mkdir -p /workspace/configs

# Generate configs/claude_mcp_config.json file (empty for baseagent-condensed, full for others)
RUN if [ "${AGENT_NAME}" = "baseagent-condensed" ]; then \
        echo '{"mcpServers": {}}' > /workspace/configs/claude_mcp_config.json; \
    else \
        echo '{\n  "mcpServers": {' > /workspace/configs/claude_mcp_config.json && \
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
}' >> /workspace/configs/claude_mcp_config.json; \
    fi

# Generate agent YAML config file based on AGENT_NAME
# recodeagent: full pipeline; baseagent: single agent; noplanning/noanalyzer/novalidator: recodeagent with one disabled
COPY docker-env/generate_agent_config.py /tmp/generate_agent_config.py
RUN AGENT_NAME="${AGENT_NAME}" TOOL_NAME="${TOOL_NAME}" PROJECT_NAME="${PROJECT_NAME}" \
    SOURCE_LANGUAGE="${SOURCE_LANGUAGE}" TARGET_LANGUAGE="${TARGET_LANGUAGE}" \
    python3 /tmp/generate_agent_config.py && rm /tmp/generate_agent_config.py


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

# install mcp-language-server from src/mcp/mcp_language_server (skip for baseagent-condensed)
RUN if [ "${AGENT_NAME}" != "baseagent-condensed" ]; then \
        cd /workspace/src/mcp/mcp_language_server && go install .; \
    fi

# Install language servers based on SOURCE_LANGUAGE and TARGET_LANGUAGE (skip for baseagent-condensed)
RUN if [ "${AGENT_NAME}" != "baseagent-condensed" ]; then \
    if [ "$SOURCE_LANGUAGE" = "go" ] || [ "$TARGET_LANGUAGE" = "go" ]; then \
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
    fi; \
    fi

# Run bear -- make for crust projects to generate compile_commands.json (skip for baseagent-condensed)
RUN if [ "${AGENT_NAME}" != "baseagent-condensed" ]; then \
    if [ "${TOOL_NAME}" = "crust" ]; then \
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
    fi; \
    fi

# Default command
CMD ["/bin/bash"]