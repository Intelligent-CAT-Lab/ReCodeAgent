# add a check that the current path ends with docker-env
if [ "${PWD: -10}" != "docker-env" ]; then
    echo "Please run this script from the docker-env directory"
    exit 1
fi

CONTAINER_NAME=$1
IMAGE_NAME=${2:-$CONTAINER_NAME}

if [ -z "$CONTAINER_NAME" ]; then
    echo "Please provide a container name as the first argument."
    exit 1
fi

docker run --name $CONTAINER_NAME -d -t \
    --mount type=bind,source=$HOME/.aws,target=/root/.aws \
    $IMAGE_NAME

docker ps | grep $CONTAINER_NAME > /dev/null
echo "# The container is running. To stop running the container: bash docker_stop.sh"