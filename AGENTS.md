## WHAT AGENT MUST DO

- Place shared logic in a dedicated shared crate.
- Use workspace-level dependencies.
- Add dependencies only when prompt explicitly requests it.
- Disable default features unless required.
- Prefer `std` over external crates.
- Declare crates.io dependencies only in workspace.dependencies.
- Use dependency.workspace = true for dependencies in workspace projects
- Prefer borrowing over cloning, especially for large structures.
- Use `Arc` only for cross-thread sharing.
- Use `Mutex` only for interior mutability.
- Prefer immutable data.
- Avoid memory leaks via static state.
- Use enums and `thiserror` for errors.
- Use a single async runtime across workspace.
- Keep trait bounds explicit.
- Use trait objects only when dynamic dispatch is required.
- Keep public API minimal.
- Add unit tests for public logic.
- Use test helpers for repeated setup.
- Keep tests deterministic.
- If error message contains 8 random symbols then search workspace for that id.
- Avoid allocations inside hot loops.
- Preserve behavior unless change is requested.
- Keep diffs minimal.
- Keep generated functions and closures inside usage scope.
- `expect()` messages must contain **8 first symbols from random UUID v4**.
- Use abbreviations when creating names.

## WHAT AGENT MUST NOT DO

- Merge unrelated crates.
- Break architecture boundaries or introduce hidden coupling.
- Edit Cargo.toml of unrelated crates.
- Add new crates unless explicitly requested.
- Silence clippy without justification.
- Leave commented dead code.
- Commit debug prints.
- Use `unwrap()`.
- Use `expect()` or `panic!()` in library code except in `proc-macro`, tests, or generated test code inside `quote!`.
- Ignore `Result` or swallow errors.
- Use or write `unsafe`.
- Assume `Send` or `Sync` without proof.
- Use outdated versions in case of adding new crate.
- Block async executors.
- Hold locks across `.await`.
- Mix async runtimes.
- Ignore cancellation safety.
- Depend on external services in tests.
- Use flaky time-based tests.
- Change public API without instruction.
- Leak generics to users.
- Refactor or reformat without request.
- Rename public items casually.
- Change semantics silently.

## Run before completion

```bash
cargo fmt
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Toolchain note

- This repository is intended for the latest Rust nightly toolchain.
