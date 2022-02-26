all: lint

export CARGO_NET_GIT_FETCH_WITH_CLI = true

build:
	cargo build

release:
	cargo build --release

lint:
	cargo clippy
	cargo clippy --tests

test:
	cargo test -- --test-threads=1

fmt:
	bash tools/fmt.sh

update:
	cargo update

clean:
	cargo clean
	git stash
	git clean -fdx

doc:
	cargo doc --open
