default: build

build: target/release/ToDo
	cargo build --release

.PHONY: run
run:
	cargo run
