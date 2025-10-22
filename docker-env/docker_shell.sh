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

# start in /workspace/ directory
echo "
NOTICE:

- The root dir of this artifact is mounted to /workspace/ in the container

"
docker exec -it -w /workspace -e NODE_PATH=/usr/lib/node_modules $CONTAINER_NAME /bin/bash
