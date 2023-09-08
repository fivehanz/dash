serve:
	npx nx run-many -t serve

watch:
	# cargo watch -q -c -x run
	bacon run

check:
	cargo check

build:
	npx nx run-many -t build

build-release:
	cargo build --release

build-release-freebsd:
	cargo build --release --target x86_64-unknown-freebsd

build-docker-freebsd:
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --target x86_64-unknown-freebsd --release

clean:
	cargo clean

lint:
	npx nx affected -t lint

update:
	cargo update

export:
	npx nx run-many -t export
