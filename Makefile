.PHONY: qa
qa:
	cargo fmt -- --check
	cargo clippy
	cargo test --all
