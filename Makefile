.PHONY: toolchain
toolchain:
	rustup default stable
	rustup component add rustfmt

.PHONY: build
build:
	cd restaurant-server && cargo build && cd -
	cd restaurant-client && cargo build && cd -

.PHONY: format
build:
	cd restaurant-server && cargo fmt && cd -
	cd restaurant-client && cargo fmt && cd -

.PHONY: run-client
build:
	cd restaurant-client && cargo run

.PHONY: run-server
build:
	cd restaurant-server && cargo run

.PHONY: test
test:
	cd restaurant-server && cargo test && cd -
	cd restaurant-client && cargo test && cd -
