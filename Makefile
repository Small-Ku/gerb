fmt:
	cargo fmt
	cargo sort
	find src -name "*.py" | xargs black
	cargo clippy --bin gerb