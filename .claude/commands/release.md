Prepare a release for the workspace.

1. Run `cargo test --workspace` to ensure all tests pass.
2. Run `cargo clippy --workspace -- -D warnings` to ensure no lint warnings.
3. Check the current version in each crate's `Cargo.toml` and list them.
4. Ask what the new version should be for each changed crate.
5. Update the version numbers in the relevant `Cargo.toml` files.
6. Run `cargo check --workspace` to verify the version bumps compile.
7. Summarize the changes and suggest a commit message.