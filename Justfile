# ------------------------------------------------------------------------------
# Justfile - auth0-sdk
# ------------------------------------------------------------------------------

# ------------------------------------------------------------------------------
# Default
# ------------------------------------------------------------------------------

[private]
default:
    @just --list --unsorted

# ------------------------------------------------------------------------------
# Quality
# ------------------------------------------------------------------------------

[doc("format all Rust code")]
[group("quality")]
fmt:
    cargo fmt --all

[doc("check Rust formatting like CI")]
[group("quality")]
fmt-check:
    cargo fmt --all -- --check

[doc("run Clippy like CI")]
[group("quality")]
clippy:
    cargo clippy --workspace --tests --all-features -- -D warnings

[doc("run all workspace tests and doctests")]
[group("quality")]
test:
    cargo test --workspace --all-features

[doc("run the local equivalent of GitHub CI checks")]
[group("quality")]
ci-check: fmt-check clippy test

# ------------------------------------------------------------------------------
# Build
# ------------------------------------------------------------------------------

[doc("build the workspace with all features")]
[group("build")]
build:
    cargo build --workspace --all-features

[doc("build workspace documentation with all features")]
[group("build")]
doc:
    cargo doc --workspace --all-features --no-deps

# ------------------------------------------------------------------------------
# Maintenance
# ------------------------------------------------------------------------------

[doc("remove Cargo build artifacts")]
[group("maintenance")]
clean:
    cargo clean
