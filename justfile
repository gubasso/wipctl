# Justfile — project task runner.
# Every recipe runs inside the pinned Nix devshell. Override with `just dev='' <recipe>`
# when the caller is already inside the devshell (CI does this).
dev := "nix develop --command"

# The global host tools, checked once before any recipe runs. just evaluates
# every top-level assignment first, so a missing tool stops the run with the
# checker's message. The list lives in host-requirements.toml, and the checker
# comes from the devShell, so it runs through the same prefix as every recipe.
# A recipe that alone uses a host tool checks it as its first line.
_host := shell(dev + " exobrain host-check nix git")

# List available recipes.
default:
	@just --list

# --- gates ---

# Run the commit-stage gate (formatting and lints, as the hooks define them).
lint:
	{{dev}} exobrain host-check prek
	{{dev}} prek run --all-files --show-diff-on-failure

# Run the push-stage gate: the integration tests, the release build, the package
# build and verification, and the dependency, history, and networked checks.
lint-push:
	{{dev}} exobrain host-check prek
	{{dev}} prek run --hook-stage pre-push --all-files --show-diff-on-failure

# Run the schema gate: every shipped schema against the draft, and the fixture
# against the schemas that govern it.
schemas:
	{{dev}} ./scripts/check-schemas

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
