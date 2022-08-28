help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

	
CARGO = $(shell command -v cargo)

##@ Development

test: ## run all tests with cargo
	RUST_BACKTRACE=1 cargo test --jobs 1
	
quick-test: ## run all fast tests with cargo (those which dont clone themselves
	cargo test --jobs 1 quick
	
