dev: frontend-dev
build: api-build frontend-build
lint: api-lint frontend-lint

api-dev: watch

frontend-dev:
	cd apps/frontend && bun --bun astro dev

watch:
	# cargo watch -q -c -x run
	bacon run

check:
	cargo check

api-build:
	cargo build --release

api-build-freebsd:
	cargo build --release --target x86_64-unknown-freebsd

api-build-docker-freebsd:
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --target x86_64-unknown-freebsd --release

clean:
	cargo clean

api-lint:
	cargo clippy

frontend-lint:
	cd apps/frontend && bun --bun astro lint

update:
	cargo update
