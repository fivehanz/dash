

watch:
	cargo watch -q -c -w src/ -w build.rs -x run

check:
	cargo check

build:
	cargo build --release

clean:
	cargo clean

lint:
	cargo clippy

update:
	cargo update
