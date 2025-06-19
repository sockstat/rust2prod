watch:
	cargo watch -x check

watch-and-test:
	cargo watch -x check -x test -x run

test:
	cargo test

coverage:
	cargo install cargo-tarpaulin
	cargo tarpaulin --ignore-tests

lint:
	cargo component add clippy
	cargo clippy

fmt:
	cargo component add rustfmt
	cargo fmt

audit:
	cargo install cargo-audit
	cargo audit
