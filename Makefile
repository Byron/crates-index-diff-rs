help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

	
CARGO = $(shell command -v cargo)

##@ Development

baseline-atomic: ## run very slow tests that single-step through all commits
	GITOXIDE_PACK_CACHE_MEMORY=1g GITOXIDE_OBJECT_CACHE_MEMORY=3g RUST_BACKTRACE=1 cargo test --test baseline_atomic --release --features max-performance -- --nocapture

test: ## run all tests with cargo
	RUST_BACKTRACE=1 cargo test --test crates-index-diff
	GITOXIDE_PACK_CACHE_MEMORY=1g RUST_BACKTRACE=1 cargo test --test baseline --release --features max-performance

