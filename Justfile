# Show help
help:
	@just --list

# Build project
build:
	@cargo build --tests

# Format
fmt:
	@cargo fmt

# Lint
lint:
	@cargo clippy --tests

# Test
test:
	@cargo test --release

# Run
run:
	@cargo run --release
