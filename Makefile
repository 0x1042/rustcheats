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

gen_entity:
	sea-orm-cli generate entity -v -o axumex/src/entity --with-serde both