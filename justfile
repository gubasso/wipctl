# Justfile — project task runner.
# Every recipe runs inside the pinned Nix devshell. Override with `just dev='' <recipe>`
# when the caller is already inside the devshell (CI does this).
dev := "nix develop --command"

# List available recipes.
default:
	@just --list

# --- gates ---

# Run the commit-stage gate (formatting and lints, as the hooks define them).
lint:
	{{dev}} pre-commit run --all-files --show-diff-on-failure

# Run the push-stage gate (dependency advisories and license policy).
lint-push:
	{{dev}} pre-commit run --hook-stage pre-push --all-files --show-diff-on-failure

# Run the test suite.
test:
	{{dev}} cargo nextest run --all-features

# Format the source tree in place.
fmt:
	{{dev}} cargo fmt --all

# Format, then run the commit-stage gate and the tests.
check: fmt lint test

# Everything `check` runs, plus the push-stage gate.
check-all: check lint-push

# --- build ---

# Build the debug binary.
build:
	{{dev}} cargo build --all-features

# Build the optimized binary.
build-release:
	{{dev}} cargo build --release --all-features

# Run the CLI with the given arguments.
run *args:
	{{dev}} cargo run -- {{args}}

# --- publishing ---

# Rehearse the crates.io publish without uploading anything.
publish-dry:
	{{dev}} ./scripts/publish-dry

# Cut a release.
release *args:
	{{dev}} ./scripts/release {{args}}

# --- installation ---

# Install the CLI from this checkout; set PREFIX to install into PREFIX/bin.
install:
	{{dev}} ./install.sh

# Remove the installed CLI. Reports and succeeds when it is not installed.
uninstall:
	{{dev}} ./uninstall.sh

# Remove the installed CLI, then install this checkout over it.
reinstall: uninstall install
