# add a check that the current path ends with docker-env
if [ "${PWD: -10}" != "docker-env" ]; then
    echo "Please run this script from the docker-env directory"
    exit 1
fi

AGENT_NAME=$1
TOOL_NAME=$2
PROJECT_NAME=$3
SOURCE_LANGUAGE=$4
TARGET_LANGUAGE=$5

if [ -z "$AGENT_NAME" ] || [ -z "$TOOL_NAME" ] || [ -z "$PROJECT_NAME" ] || [ -z "$SOURCE_LANGUAGE" ] || [ -z "$TARGET_LANGUAGE" ]; then
    echo "Usage: ./docker_build.sh <AGENT_NAME> <TOOL_NAME> <PROJECT_NAME> <SOURCE_LANGUAGE> <TARGET_LANGUAGE>"
    echo "Agent names: recodeagent, baseagent-concat, baseagent-condensed, noplanning, noanalyzer, novalidator"
    exit 1
fi

IMAGE_NAME=${AGENT_NAME}.${TOOL_NAME}.${PROJECT_NAME}.${SOURCE_LANGUAGE}.${TARGET_LANGUAGE}

docker build -t ${IMAGE_NAME} \
    --build-arg AGENT_NAME="${AGENT_NAME}" \
    --build-arg TOOL_NAME="${TOOL_NAME}" \
    --build-arg PROJECT_NAME="${PROJECT_NAME}" \
    --build-arg SOURCE_LANGUAGE="${SOURCE_LANGUAGE}" \
    --build-arg TARGET_LANGUAGE="${TARGET_LANGUAGE}" \
    -f ./Dockerfile ..
