fmt:
	cargo +nightly fmt

build: fmt
	cargo build

clean:
	cargo clean

build_cheats: fmt
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

build_fs: fmt
	cargo build -p fullstack

run_fs: build_fs
	./target/debug/fullstack

build_reset: fmt
	cargo build -p reset

build_dag: fmt
	cargo build -p dag

build_sv2: fmt
	cargo build -p socks5v2