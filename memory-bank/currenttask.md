# Current Task: Setup GitHub Actions CI

## Objective

Create a GitHub Actions CI workflow that automatically runs code quality checks on every push to `main` and pull requests targeting `main`.

## Implementation Steps

1. **Add `fmt` command to `justfile`**
   - Add a `fmt` recipe that runs `cargo fmt`
   - This provides a convenient local command for formatting code

2. **Create `.github/workflows/ci.yml`**
   - Create the workflows directory if it doesn't exist
   - Create the CI workflow file with the following configuration:

   **Triggers:**
   - Push to `main` branch
   - Pull requests targeting `main` branch

   **Runner:**
   - `ubuntu-latest`

   **Steps:**
   1. `actions/checkout@v4` - Clone repository into runner
   2. `actions-rust-lang/setup-rust-toolchain@v1` - Install Rust stable with automatic caching and problem matchers
   3. `taiki-e/install-action@v2` - Install `cargo-hack` and `just` (precompiled binaries with caching)
   4. `cargo fmt --check` - Verify code formatting
   5. `just check` - Compilation check across all feature combinations
   6. `just lint` - Clippy lints across all feature combinations
   7. `just test` - Run workspace tests

## Reference: Final Workflow

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-hack,just

      - run: cargo fmt --check
      - run: just check
      - run: just lint
      - run: just test
```

## Notes

- `actions-rust-lang/setup-rust-toolchain@v1` automatically:
  - Installs stable Rust toolchain
  - Caches cargo registry and target directory
  - Sets `RUSTFLAGS=-D warnings`
  - Enables problem matchers for inline error display in GitHub UI

- `taiki-e/install-action@v2` automatically:
  - Downloads precompiled binaries (fast, no compilation)
  - Caches installed tools
  - Verifies SHA256 checksums

- No cleanup required - GitHub Actions runners are ephemeral (destroyed after each run)
