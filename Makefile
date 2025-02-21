all: run

build: 
	cargo build

run:
	RUSTFLAGS="-Awarnings" cargo run	

clean:
	cargo clean

