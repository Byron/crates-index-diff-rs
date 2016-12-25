info:
	$(info Available Targets)
	$(info ---------------------------------------------------------------------------)
	$(info test       | run all tests with cargo)
	$(info quick-test | run all fast tests with cargo (those which dont clone themselves))
	
CARGO = $(shell command -v cargo)
bare_index_path = tests/fixtures/index-bare

$(bare_index_path):
	mkdir -p $(dir $@)
	git clone --bare https://github.com/rust-lang/crates.io-index $@

test: $(bare_index_path)
	CRATES_INDEX_DIFF_TEST_EXISTING_INDEX=$(bare_index_path) && cargo test
	
quick-test: $(bare_index_path)
	CRATES_INDEX_DIFF_TEST_EXISTING_INDEX=$(bare_index_path) && cargo test quick
	
