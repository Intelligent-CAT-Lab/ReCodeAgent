#!/usr/bin/env bash

# Check if all required arguments are provided
if [ $# -lt 5 ]; then
    echo "Usage: $0 <agent_name> <tool_name> <project_name> <source_language> <target_language>"
    echo "Example: $0 recodeagent crust 2dpartint c rust"
    exit 1
fi

# Parse input parameters
AGENT_NAME=$1
TOOL_NAME=$2
PROJECT_NAME=$3
SOURCE_LANGUAGE=$4
TARGET_LANGUAGE=$5
CONTAINER_NAME=${TOOL_NAME}.${PROJECT_NAME}.${SOURCE_LANGUAGE}.${TARGET_LANGUAGE}

# Construct config file path based on the parameters
CONFIG_FILE_PATH="configs/${AGENT_NAME}_${TOOL_NAME}_${PROJECT_NAME}_${SOURCE_LANGUAGE}_${TARGET_LANGUAGE}.yaml"

echo "Using parameters:"
echo "- Agent: $AGENT_NAME"
echo "- Tool: $TOOL_NAME"
echo "- Project: $PROJECT_NAME"
echo "- Source Language: $SOURCE_LANGUAGE"
echo "- Target Language: $TARGET_LANGUAGE"
echo "- Container Name: $CONTAINER_NAME"
echo "- Config File Path: $CONFIG_FILE_PATH"

# Navigate to docker-env directory
DOCKER_ENV_DIR="$PWD/docker-env"
if [ ! -d "$DOCKER_ENV_DIR" ]; then
    echo "Error: docker-env directory not found"
    exit 1
fi

echo "Using docker-env directory: $DOCKER_ENV_DIR"
cd "$DOCKER_ENV_DIR" || exit 1

# Build the Docker image
echo "Building Docker image..."
bash docker_build.sh "$TOOL_NAME" "$PROJECT_NAME" "$SOURCE_LANGUAGE" "$TARGET_LANGUAGE"

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Error: Docker image build failed"
    exit 1
fi

IMAGE_NAME="${TOOL_NAME}.${PROJECT_NAME}.${SOURCE_LANGUAGE}.${TARGET_LANGUAGE}"
echo "Docker image built successfully: $IMAGE_NAME"

# Run the Docker container
echo "Starting Docker container: $CONTAINER_NAME"
bash docker_run.sh $CONTAINER_NAME

# Check if the container started successfully
if [ $? -ne 0 ]; then
    echo "Error: Failed to start Docker container"
    exit 1
fi

echo "Docker container started successfully: $CONTAINER_NAME"

# Wait a moment for container to be ready
sleep 2

# Start tmux session 0 in the container and run the scripts/run.sh
echo "Starting tmux session 0 and running scripts/run.sh..."
docker exec "$CONTAINER_NAME" tmux new-session -d -s 0 "cd /workspace && bash scripts/run.sh $CONFIG_FILE_PATH; exec bash"

if [ $? -ne 0 ]; then
    echo "Error: Failed to start tmux session or run script"
    exit 1
fi

echo "=====================================================
Docker container is running with tmux session 0
You can attach to the session with:
docker exec -it $CONTAINER_NAME tmux attach-session -t 0

To stop the container:
docker stop $CONTAINER_NAME
docker rm $CONTAINER_NAME
====================================================="

# Return to the original directory
cd - > /dev/null

echo "Setup completed successfully!"