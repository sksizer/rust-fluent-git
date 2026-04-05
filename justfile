set allow-duplicate-recipes := true
set allow-duplicate-variables := true
set shell := ["bash", "-euo", "pipefail", "-c"]

# ---------------------------------------------------------------------------- #
#                                 DEPENDENCIES                                 #
# ---------------------------------------------------------------------------- #

# Rust: https://rust-lang.org/tools/install
cargo := require("cargo")
rustc := require("rustc")

# ---------------------------------------------------------------------------- #
#                                    RECIPES                                   #
# ---------------------------------------------------------------------------- #

# Show available commands
default:
    @just --list

# Build the library
build:
    cargo build

# Run all code checks
full-check:
    cargo fmt --all --check
    cargo clippy -- --deny warnings
alias fc := full-check

# Auto-fix formatting
full-write:
    cargo fmt --all
alias fw := full-write

# Run sync (blocking) tests
test:
    cargo test

# Run async (tokio) tests
test-async:
    cargo test --no-default-features --features tokio

# Run both sync and async tests
test-all: test test-async

# ---------------------------------------------------------------------------- #
#                                   RELEASE                                    #
# ---------------------------------------------------------------------------- #

# Generate changelog from conventional commits
changelog:
    git-cliff --output CHANGELOG.md

# Check for semver violations against the latest git tag
semver-check:
    cargo semver-checks --baseline-rev "$(git describe --tags --abbrev=0)"

# Dry-run a release (default: patch bump)
release-dry-run level="patch":
    cargo release {{level}} --no-confirm

# Perform a release (patch, minor, or major)
release level="patch":
    cargo release {{level}} --execute
