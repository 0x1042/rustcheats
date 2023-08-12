fmt:
	cargo +nightly fmt

build: fmt
	cargo build -p cheats

test: fmt
	cargo test -- --nocapture

run: build
	./target/debug/cheats