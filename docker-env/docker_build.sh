# add a check that the current path ends with docker-env
if [ "${PWD: -10}" != "docker-env" ]; then
    echo "Please run this script from the docker-env directory"
    exit 1
fi

TOOL_NAME=$1
PROJECT_NAME=$2
SOURCE_LANGUAGE=$3
TARGET_LANGUAGE=$4

if [ -z "$TOOL_NAME" ] || [ -z "$PROJECT_NAME" ] || [ -z "$SOURCE_LANGUAGE" ] || [ -z "$TARGET_LANGUAGE" ]; then
    echo "Usage: ./docker_build.sh <TOOL_NAME> <PROJECT_NAME> <SOURCE_LANGUAGE> <TARGET_LANGUAGE>"
    exit 1
fi

docker build -t ${TOOL_NAME}.${PROJECT_NAME}.${SOURCE_LANGUAGE}.${TARGET_LANGUAGE} \
    --build-arg TOOL_NAME="${TOOL_NAME}" \
    --build-arg PROJECT_NAME="${PROJECT_NAME}" \
    --build-arg SOURCE_LANGUAGE="${SOURCE_LANGUAGE}" \
    --build-arg TARGET_LANGUAGE="${TARGET_LANGUAGE}" \
    -f ./Dockerfile ..
