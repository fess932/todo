include .env
export

build:
	cargo build
release:
	cargo build --profile release
run:
	RUST_LOG=tower_http=trace cargo run --bin server

clean_local_registry:
	rm -rf ~/.cargo/registry

update: clean_local_registry
	rustup update
	cargo update
