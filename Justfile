@_default:
  just -l

# Run clippy
clippy:
  cargo clippy --all --tests --all-features --no-deps

# Run test coverage
coverage:
  cargo tarpaulin --out Html
