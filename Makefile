.DEFAULT_GOAL := run

run:
	@cargo run

# cargo-watch
watch: 
	@cargo watch -q -c -w src -x run

build:
	@cargo build

clean:
	@cargo clean
