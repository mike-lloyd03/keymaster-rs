set dotenv-load

PACKAGE_VERSION := `git tag -l | tail -1 | awk '{print substr($1,2); }'`
IMG_NAME := "$DOCKER_REGISTRY/keymaster"

build:
    docker build . --tag {{IMG_NAME}}:latest --tag {{IMG_NAME}}:{{PACKAGE_VERSION}}

push:
	docker push {{IMG_NAME}}:latest
	docker push {{IMG_NAME}}:{{PACKAGE_VERSION}}
