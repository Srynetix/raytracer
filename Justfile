# Show help
help:
	@just --list

# Build project
build:
	@cargo build

# Format
fmt:
	@cargo fmt

# Lint
lint:
	@cargo clippy

# Test
test:
	@cargo test

# Run
run:
	@cargo run
