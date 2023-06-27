fmt:
	cargo fmt 

build: fmt
	cargo build 

test: fmt
	cargo test -- --nocapture

run: build
	./target/debug/rustcheats