# AGENTS.md

# WHAT AGENT MUST DO

## Workspace Discipline

* Keep dependency graph acyclic.
* Place shared logic in a dedicated shared crate.
* Use workspace-level dependencies.

## Dependencies

* Add dependencies only when propmt says so.
* Disable default features unless required.
* Avoid multiple crates solving the same problem.
* Prefer std over external crates.
* Keep versions consistent across workspace.

## Ownership & Memory

* Prefer borrowing over cloning.
* Avoid unnecessary Arc and Mutex.
* Prefer immutable data.
* Avoid Arc unless cross-thread sharing is required.
* Avoid Mutex unless interior mutability is required.
* Prefer immutable data structures.
* Avoid leaking memory via static state.

## Error Handling

* Use enums and thiserror for errors.
* Do not use anyhow.

## Async

* Use one async runtime across workspace.
* Do not block async executors.
* Use spawn_blocking for CPU-heavy tasks.
* Avoid unnecessary async functions.

## Traits & Generics

* Avoid unnecessary generics.
* Keep trait bounds explicit.
* Avoid trait objects unless dynamic dispatch is required.

## Modules & Files

* Avoid deep module nesting.
* Avoid god modules.
* Keep public API minimal.

## Testing

* Add unit tests for public logic.
* Avoid duplicating test logic.
* Use test helpers for repeated setup.
* Keep tests deterministic.
* Avoid sleeping in tests.
* Avoid network access in tests.
* if error message in tests contains 8 random symbols - try to find this error id in workspace

## Performance

* Avoid unnecessary allocations.
* Avoid cloning large structures.
* Do not allocate inside hot loops.

## Public API

* Avoid breaking changes.
* Avoid leaking internal types.
* Use explicit return types.
* Avoid exposing generic complexity publicly.

## Refactoring

* Preserve behavior unless asked to change it.
* Keep diffs minimal.
* Do not reformat unrelated files.
* Avoid changing public API silently.
* generated function and closures must not be declared outside scope of usage

## Documentation

* Do not add documention to code.
* Do not add comments to code.
* Keep README accurate.
* Ensure examples compile.

---

# WHAT AGENT MUST NOT DO

## Architecture Violations

* Do not introduce cyclic dependencies.
* Do not merge unrelated crates.
* Do not collapse boundaries between layers.
* Do not create hidden coupling.
* Do not edit Cargo.toml of unrelated crates.
* Do not modify workspace structure.
* Do not add new crates unless explicitly requested.

## Code Quality Violations

* Do not silence clippy without justification.
* Do not add TODO without instruction.
* Do not leave commented dead code.
* Do not commit debug prints unless if they will be removed after debug.

## Safety Violations

* Do not use unwrap().
* Do not use expect() and panic!() in library code unless if its 'proc-macro' crate or test or inside quote!{} unless if it is a part of generated test by macro.
* expect() message must be 8 symbols of random uuid v4.
* Do not ignore Result.
* Do not swallow errors.
* Do not use functions and methods and do not write unsafe code.
* Do not assume Send/Sync without proof.
* Never introduce UB.

## Dependency Violations

* Do not add heavy frameworks casually.
* Do not duplicate functionality.
* Do not use outdated crates.

## Performance Violations

* Do not allocate in hot paths unnecessarily.
* Do not clone blindly.
* Do not block async executors.
* Do not hold locks across await.

## Async Violations

* Do not mix async runtimes.
* Do not call blocking IO in async.
* Do not use async when not needed.
* Do not ignore cancellation safety.

## Testing Violations

* Do not rely on external services in tests.
* Do not use flaky time-based tests.
* Do not skip tests silently.

## API Violations

* Do not change signatures without instruction.
* Do not widen trait bounds unnecessarily.
* Do not leak generics to users.

## Refactor Violations

* Do not refactor entire workspace without request.
* Do not reformat whole repo unnecessarily.
* Do not rename public items casually.
* Do not change semantics silently.

## Git Violations

* Do not squash unrelated changes.
* Do not commit broken code.
* Do not include generated artifacts.
* Do not modify Cargo.lock unless required.

---

# Run before completion

cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --features test-utils

Fix all issues before finishing the task.

End of file.
