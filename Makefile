dev: check test run

check:
	@cargo check

test:
	@cargo test

run:
	@cargo run

.PHONY: docs
docs:
	@cargo doc --release --workspace --no-deps --all-features --target-dir docs/technical
	@git add docs/technical

viewdocs:
	@cd docs/technical; python -m http.server 4004

update:
	@cargo upgrade
	@cargo update

release: test docs
