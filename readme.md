# Run Docker from Rust code
---
```
docker rm $(docker ps -a -q)
docker rmi $(docker images -q)


docker build -t my-python-app:latest .
cargo r

docker run -p 8080:8080 my-python-app:latest

docker stop python-container

docker run -p 8080:8080 my-python-app:latest

docker rm my-python-app
```
