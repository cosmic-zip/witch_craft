echo && echo "Build docker container"
docker buildx build . -t witchcraft

echo && echo "Run witchcraft inside a docker container"
docker run -it witchcraft bash
