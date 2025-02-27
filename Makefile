all: run

run: 
	RUSTFLAGS="-Awarnings" cargo run
build:
	cargo build
clean:
	cargo clean