include .env
export

build:
	cargo build
release:
	cargo build --profile release
run:
	cargo run

clean_local_registry:
	rm -rf ~/.cargo/registry

update: clean_local_registry
	rustup update
	cargo update