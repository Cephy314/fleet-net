.PHONY: fmt lint test check install-hooks clean

# Format all code
fmt:
	cargo fmt --all
	taplo fmt

# Run linters
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	cargo test --all

# Run all checks (format check, lint, test)
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all

# Install pre-commit hooks
install-hooks:
	pre-commit install
	pre-commit install --hook-type commit-msg

# Clean build artifacts
clean:
	cargo clean

# Run pre-commit on all files
pre-commit-all:
	pre-commit run --all-files
