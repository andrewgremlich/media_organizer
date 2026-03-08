Check for outdated dependencies and upgrade them safely.

1. Run `cargo outdated` (install with `cargo install cargo-outdated` if not available).
2. Review each outdated dependency. Flag any that need special attention:
   - `ffmpeg-next` must match the system FFmpeg version — do NOT upgrade without confirming.
   - Major version bumps may have breaking API changes.
3. For safe minor/patch upgrades, update the version in the relevant `Cargo.toml`.
4. Run `cargo check --workspace` after each change to verify it compiles.
5. Run `cargo test --workspace` to confirm nothing broke.
6. Summarize what was upgraded and what was skipped (and why).