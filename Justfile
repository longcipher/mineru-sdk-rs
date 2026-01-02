format:
  rumdl fmt .
  taplo fmt
  cargo +nightly fmt --all
fix:
  rumdl check --fix .
lint:
  typos
  rumdl check .
  taplo fmt --check
  cargo +nightly fmt --all -- --check
  cargo +nightly clippy --all -- -D warnings -A clippy::derive_partial_eq_without_eq -D clippy::unwrap_used -D clippy::uninlined_format_args
  cargo machete
test:
  cargo test --all-features
test-coverage:
  cargo tarpaulin --all-features --workspace --timeout 300
check-cn:
  rg --line-number --column "\p{Han}"
# Full CI check
ci: lint test