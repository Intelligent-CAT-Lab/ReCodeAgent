# add a check that the current path ends with docker-env
if [ "${PWD: -10}" != "docker-env" ]; then
    echo "Please run this script from the docker-env directory"
    exit 1
fi

CONTAINER_NAME=$1

if [ -z "$CONTAINER_NAME" ]; then
    echo "Please provide a container name as the first argument."
    exit 1
fi

# if container does not exist, do nothing
# if container exists, stop and remove it
if [ "$(docker ps -q -f name=$CONTAINER_NAME)" ]; then
    docker stop $CONTAINER_NAME
    echo "# The container is stopped. Please run ./docker_run.sh to start it again if needed."
fi