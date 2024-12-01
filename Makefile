.PHONY: toolchain
toolchain:
	rustup default stable

.PHONY: build
build:
	cd restaurant-server && cargo build && cd -
	cd restaurant-client && cargo build && cd -

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
