# AGENTS.md

# ✅ WHAT AGENT MUST DO

## Workspace Discipline

* Keep workspace compiling at all times.
* Run checks before completing any task.
* Respect crate boundaries.
* Use path dependencies for internal crates.
* Keep dependency graph acyclic.
* Place shared logic in a dedicated shared crate.
* Keep Cargo.toml files clean and minimal.
* Use workspace-level dependency versions when possible.
* Maintain deterministic builds.

## Cargo & Tooling

* Run `cargo fmt --all`.
* Run `cargo check --workspace`.
* Run `cargo test --features test-utils -- --nocapture --workspace`.
* Run `cargo clippy --all-targets --all-features -D warnings`.
* Fix all warnings before completion.
* Ensure new features compile with `--all-features`.
* Ensure no feature flag breaks compilation.

## Dependencies

* Add dependencies only when necessary.
* Disable default features unless required.
* Prefer minimal dependency trees.
* Avoid multiple crates solving the same problem.
* Prefer std over external crates.
* Justify any heavy dependency.
* Keep versions consistent across workspace.

## Code Style

* Follow Rust idioms strictly.
* Use snake_case for functions and variables.
* Use UpperCamelCase for types.
* Use SCREAMING_SNAKE_CASE for constants.
* Keep functions small and focused.
* Prefer early returns over deep nesting.
* Avoid overly clever code.
* Prefer readability over micro-optimizations.

## Ownership & Memory

* Avoid unnecessary cloning.
* Prefer borrowing over owning when possible.
* Avoid Arc unless cross-thread sharing is required.
* Avoid Rc in multithreaded contexts.
* Avoid Mutex unless interior mutability is required.
* Prefer immutable data structures.
* Avoid leaking memory via static state.

## Error Handling

* Never use unwrap() in production code.
* Never use expect() in production code, unless if it a 'proc-macro' crate.
* Return typed errors.
* Use thiserror for domain errors.
* Do not use anyhow.
* Propagate errors properly.
* Provide meaningful error messages or use first 8 symbols of random uuid v4 for message.
* if error message contains 8 random symbols - try to find this error id in workspace

## Async

* Use one async runtime across workspace.
* Do not mix runtimes.
* Avoid blocking calls in async code.
* Use spawn_blocking for CPU-heavy tasks.
* Avoid unnecessary async functions.
* Prefer structured concurrency.
* Ensure Send + Sync correctness.

## Traits & Generics

* Avoid unnecessary generics.
* Prefer concrete types internally.
* Keep trait bounds explicit.
* Avoid overly complex trait hierarchies.
* Avoid blanket implementations unless necessary.
* Avoid trait objects unless dynamic dispatch is required.

## Modules & Files

* Keep files under 10000 lines when possible.
* Avoid deep module nesting.
* Group related logic logically.
* Avoid god-modules.
* Keep public API minimal.
* Hide internal implementation details.

## Testing

* Add unit tests for public logic.
* Cover edge cases.
* Cover error paths.
* Avoid duplicating test logic.
* Use test helpers for repeated setup.
* Keep tests deterministic.
* Avoid sleeping in tests.
* Avoid network access in tests.

## Performance

* Avoid unnecessary heap allocations.
* Prefer iterators over temporary Vec.
* Avoid repeated allocations in loops.
* Avoid cloning large structures.
* Benchmark before optimizing.
* Optimize only measurable bottlenecks.

## Unsafe

* Do not write unsafe code.
* Do not use unsafe code.
* Do not use unsafe functions and methods.
* Never introduce UB.

## Public API

* Document all public items.
* Keep API stable unless instructed otherwise.
* Avoid breaking changes.
* Avoid leaking internal types.
* Use explicit return types.
* Avoid exposing generic complexity publicly.

## Refactoring

* Preserve behavior unless asked to change it.
* Keep diffs minimal.
* Avoid mass renaming without reason.
* Do not reformat unrelated files.
* Avoid changing public API silently.

## Git Hygiene

* Make one logical change per commit.
* Write meaningful commit messages.
* Do not mix formatting with logic changes.
* Keep commits reviewable.

## Documentation

* Keep README accurate.
* Do not add comments documentation in code.
* Add rustdoc examples where useful.
* Ensure examples compile.

---

# ❌ WHAT AGENT MUST NOT DO

## Architecture Violations

* Do not introduce cyclic dependencies.
* Do not merge unrelated crates.
* Do not collapse boundaries between layers.
* Do not create hidden coupling.

## Code Quality Violations

* Do not introduce warnings.
* Do not silence clippy without justification.
* Do not add TODO without instruction.
* Do not leave commented dead code.
* Do not commit debug prints unless if they will be removed after debug.

## Safety Violations

* Do not introduce unwrap() in library code.
* Do not introduce expect() in library code unless if its 'proc-macro' crate or test.
* Do not introduce panic!() in library code unless if its 'proc-macro' crate or test.
* Do not introduce unwrap() inside quote!{} block.
* Do not introduce expect() inside quote!{} unless if it is a part of generated test by macro.
* Do not introduce panic!() inside quote!{} unless if it is a part of generated test by macro.
* Do not ignore Result.
* Do not swallow errors.
* Do not use unsafe.
* Do not assume Send/Sync without proof.

## Dependency Violations

* Do not add heavy frameworks casually.
* Do not duplicate functionality.
* Do not add unused dependencies.
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
* Do not leave failing tests.
* Do not skip tests silently.

## API Violations

* Do not expose internal modules publicly.
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

# 🏁 Completion Checklist

Before finishing:

* Workspace compiles.
* Tests pass.
* Clippy passes with zero warnings.
* Formatting applied.
* No unwrap in production code.
* No unused imports.
* No debug prints.
* No TODOs left unintentionally.

End of file.
