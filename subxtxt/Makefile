.PHONY: help build-node-image

build-node-image: ## Builds the aleph-node image for integration testing
	@docker build --tag "aleph-node:local-test" -f ./tests/docker/Dockerfile .

help: ## Displays this help
	@awk 'BEGIN {FS = ":.*##"; printf "$(MAKEFILE_NAME)\nUsage:\n  make \033[1;36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[1;36m%-25s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)