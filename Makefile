# shai Makefile
# Common development tasks

.PHONY: help build test test-all check fmt clippy doc clean install run repl release

# Default target
help:
	@echo "shai - AI shell agent"
	@echo ""
	@echo "Available targets:"
	@echo "  build       - Build debug binary"
	@echo "  release     - Build optimized release binary"
	@echo "  test        - Run unit tests"
	@echo "  test-all    - Run all tests (including integration)"
	@echo "  check       - Run cargo check (fast type-check)"
	@echo "  fmt         - Format code with rustfmt"
	@echo "  clippy      - Run clippy lints"
	@echo "  doc         - Generate documentation"
	@echo "  clean       - Clean build artifacts"
	@echo "  install     - Install binary to ~/.cargo/bin"
	@echo "  run         - Run one-shot with example prompt"
	@echo "  repl        - Start REPL mode"

# Build debug
build:
	cargo build

# Build release
release:
	cargo build --release

# Run tests
test:
	cargo test --lib

test-all:
	cargo test

# Fast type check
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt --check

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Generate documentation
doc:
	cargo doc --no-deps --document-private-items

# Open documentation
doc-open: doc
	cargo doc --open

# Clean build artifacts
clean:
	cargo clean

# Install binary
install: release
	cargo install --path .

# Run one-shot with example
run: build
	./target/debug/shai "list files in current directory"

# Start REPL
repl: build
	./target/debug/shai

# Development: watch and rebuild (requires cargo-watch)
watch:
	cargo watch -x build -x test

# Pre-commit checks (run before committing)
pre-commit: fmt-check clippy test
	@echo "All pre-commit checks passed!"

# CI pipeline simulation
ci: check fmt-check clippy test
	@echo "CI checks passed!"

# Update dependencies
update:
	cargo update

# Audit dependencies for vulnerabilities (requires cargo-audit)
audit:
	cargo audit

# Generate SBOM (requires cargo-cyclonedx)
sbom:
	cargo cyclonedx -o sbom.xml

# Benchmarks (if any)
bench:
	cargo bench

# All the things
all: clean fmt clippy test release
	@echo "Complete build successful!"