all: fmt lint

export CARGO_NET_GIT_FETCH_WITH_CLI = true

build:
	cargo build

release:
	cargo build --release

lint:
	cargo clippy --features="with_precompile_utils,with_common_precompiles"
	cargo clippy --tests --features="with_precompile_utils,with_common_precompiles"

test:
	cargo test -- --test-threads=1

fmt:
	cargo +nightly fmt

fmtall:
	bash tools/fmt.sh

update:
	cargo update

clean:
	cargo clean
	git stash
	git clean -fdx

doc:
	cargo doc --open
