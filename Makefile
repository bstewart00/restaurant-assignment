.PHONY: toolchain
toolchain:
	rustup default stable
	rustup component add rustfmt

.PHONY: build
build:
	cd restaurant-server && cargo build && cd -
	cd restaurant-client && cargo build && cd -

.PHONY: format
format:
	cd restaurant-server && cargo fmt && cd -
	cd restaurant-client && cargo fmt && cd -

.PHONY: test
test:
	cd restaurant-server && cargo test && cd -
	cd restaurant-client && cargo test && cd -

.PHONY: run-client
run-client:
	cd restaurant-client && cargo run

.PHONY: run-server
run-server:
	cd restaurant-server && cargo run
