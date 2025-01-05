.PHONY: build-debug
build-debug:
	cargo build

.PHONY: test
test: build-debug
	cargo test  --bin zia

.PHONY: test-cov
test-cov: build-debug
	cargo llvm-cov --html

.PHONY: lint
lint: 
	cargo clippy --all -- -D warnings