# AGENTS.md

## WHAT AGENT MUST DO

- Place shared logic in a dedicated shared crate.
- Use workspace-level dependencies.
- Add dependencies only when prompt explicitly requests it.
- Disable default features unless required.
- Prefer `std` over external crates.
- Keep versions consistent across workspace.
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
- Use explicit return types.
- Preserve behavior unless change is requested.
- Keep diffs minimal.
- Keep generated functions and closures inside usage scope.
- Keep README accurate.
- Ensure examples compile.

## WHAT AGENT MUST NOT DO

- Merge unrelated crates.
- Break architecture boundaries or introduce hidden coupling.
- Edit Cargo.toml of unrelated crates.
- Add new crates unless explicitly requested.
- Silence clippy without justification.
- Introduce TODO markers without instruction.
- Leave commented dead code.
- Commit debug prints.
- Use `unwrap()`.
- Use `expect()` or `panic!()` in library code except in `proc-macro`, tests, or generated test code inside `quote!`.
- Ignore `Result` or swallow errors.
- Use or write `unsafe`.
- Assume `Send` or `Sync` without proof.
- Duplicate existing functionality.
- Use outdated crates.
- Clone blindly.
- Block async executors.
- Hold locks across `.await`.
- Mix async runtimes.
- Use async where unnecessary.
- Ignore cancellation safety.
- Depend on external services in tests.
- Use flaky time-based tests.
- Skip tests silently.
- Change public API without instruction.
- Widen trait bounds unnecessarily.
- Leak generics to users.
- Refactor or reformat without request.
- Rename public items casually.
- Change semantics silently.
- Squash unrelated changes.
- Commit code.
- Modify `Cargo.lock`.

`expect()` messages must contain **8 random symbols from UUID v4**.

## Run before completion

​```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
​```
