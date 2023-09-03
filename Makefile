serve:
	npx nx run-many -t serve

watch:
	# cargo watch -q -c -x run
	bacon run

check:
	cargo check

build:
	npx nx run-many -t build

clean:
	cargo clean

lint:
	npx nx affected -t lint

update:
	cargo update

export:
	npx nx run-many -t export
