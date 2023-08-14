fmt:
	cargo +nightly fmt

build: fmt
	cargo build -p cheats

build_axum: fmt
	cargo build -p axumex

test: fmt
	cargo test -- --nocapture

run: build
	./target/debug/cheats

run_axum: build_axum
	./target/debug/server

run_server: build_server
	./target/debug/socks

build_server: fmt
	cargo build -p socks5

gen_entity:
	sea-orm-cli generate entity -v -o axumex/src/entity --with-serde both