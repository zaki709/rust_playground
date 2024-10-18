DOCKER_IMAGE=rust_playground

.PHONY build:
build:
	docker build -t ${DOCKER_IMAGE} .

.PHONY run:
run:
	docker run --rm -v .:/usr/src/app -w /usr/src/app ${DOCKER_IMAGE} cargo run

.PHONY shell:
shell:
	docker run --rm -it -v .:/usr/src/app ${DOCKER_IMAGE} /bin/bash