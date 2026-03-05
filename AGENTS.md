# AGENTS.md

# WHAT AGENT MUST DO

## Workspace Discipline

- Keep dependency graph acyclic.
- Place shared logic in a dedicated shared crate.
- Use workspace-level dependencies.

## Dependencies

- Add dependencies only when prompt explicitly requests it.
- Disable default features unless required.
- Prefer `std` over external crates.
- Keep versions consistent across workspace.

## Ownership & Memory

- Prefer borrowing over cloning, especially for large structures.
- Use `Arc` only for cross-thread sharing.
- Use `Mutex` only for interior mutability.
- Prefer immutable data.
- Avoid memory leaks via static state.

## Error Handling

- Use enums and `thiserror` for errors.

## Async

- Use a single async runtime across workspace.

## Traits & Generics

- Avoid unnecessary generics.
- Keep trait bounds explicit.
- Use trait objects only when dynamic dispatch is required.

## Modules & Files

- Avoid deep module nesting.
- Avoid god modules.
- Keep public API minimal.

## Testing

- Add unit tests for public logic.
- Use test helpers for repeated setup.
- Keep tests deterministic.
- Avoid sleeping in tests.
- Avoid network access in tests.
- If error message contains 8 random symbols — search workspace for that id.

## Performance

- Avoid allocations inside hot loops.

## Public API

- Avoid leaking internal types.
- Use explicit return types.

## Refactoring

- Preserve behavior unless change is requested.
- Keep diffs minimal.
- Avoid reformatting unrelated files.
- Keep generated functions and closures inside usage scope.

## Documentation

- Keep README accurate.
- Ensure examples compile.

---

# WHAT AGENT MUST NOT DO

The agent must **not** perform the following actions:

## Architecture

- Merge unrelated crates.
- Break architecture boundaries or introduce hidden coupling.
- Edit Cargo.toml of unrelated crates.
- Add new crates unless explicitly requested.

## Code Quality

- Silence clippy without justification.
- Introduce TODO markers without instruction.
- Leave commented dead code.
- Commit debug prints.

## Safety

- Use `unwrap()`.
- Use `expect()` or `panic!()` in library code except in `proc-macro`, tests, or generated test code inside `quote!`.
- Ignore `Result`.
- Swallow errors.
- Use or write `unsafe`.
- Assume `Send` or `Sync` without proof.
- Introduce undefined behavior.

`expect()` messages must contain **8 random symbols from UUID v4**.

## Dependencies

- Duplicate existing functionality.
- Use outdated crates.

## Performance

- Allocate unnecessarily.
- Clone blindly.
- Block async executors.
- Hold locks across `.await`.

## Async

- Mix async runtimes.
- Use async where it is unnecessary.
- Ignore cancellation safety.

## Testing

- Depend on external services.
- Use flaky time-based tests.
- Skip tests silently.

## API

- Change public API without instruction.
- Widen trait bounds unnecessarily.
- Leak generics to users.

## Refactoring

- Refactor the entire workspace without request.
- Reformat the whole repository.
- Rename public items casually.
- Change semantics silently.

## Git

- Squash unrelated changes.
- Commit code.
- Include generated artifacts.
- Modify `Cargo.lock`.

---

# Run before completion

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --features test-utils
```