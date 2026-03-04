# AGENTS.md

## 🧭 Project Overview

This repository is a Rust workspace consisting of multiple crates.
The primary goals of this project are:

* Maintain strict type safety
* Keep compile times reasonable
* Preserve clean module boundaries
* Avoid unnecessary abstractions
* Ensure deterministic builds

Agents working on this repository must follow the rules below.

---

# 🏗 Workspace Structure Rules

## 1. Crate Organization

* Each logical domain must live in its own crate.
* Avoid cyclic dependencies between crates.
* Macros must live in a separate `proc-macro` crate.

## 2. Dependency Policy

* Avoid adding new external dependencies unless absolutely necessary.
* Prefer `std` over external crates.
* If adding a dependency:

  * Ensure it is actively maintained.
  * Avoid large transitive dependency trees.
  * Disable default features when possible.
* Never duplicate functionality already available in the workspace.

---

# 🧠 Code Design Principles

## 3. Ownership & Lifetimes

* Prefer explicit lifetimes when required for clarity.
* Avoid unnecessary cloning.
* Avoid `Arc<Mutex<T>>` unless concurrency truly requires it.
* Prefer interior mutability only when justified.

## 4. Error Handling

* Do not use `unwrap()`
* Do not use `expect()` unless inside `proc-macro` crate.
* Prefer `thiserror` for domain errors.
* Do not use `anyhow` only at application boundaries.
* Errors must be meaningful and typed.

## 5. Traits & Generics

* Avoid over-generic APIs.
* Use trait bounds explicitly.
* Avoid blanket trait implementations unless well justified.
* Prefer concrete types over `dyn Trait` unless dynamic dispatch is required.

---

# 🧪 Testing Rules

## 6. Unit Tests

* Every public function must have at least one test.
* Use `#[cfg(test)]` modules.
* Avoid integration tests when a unit test is sufficient.

## 7. Integration Tests

* Integration tests belong in `/tests`.
* Do not duplicate logic.
* Do not duplicate logic from unit tests.
* Use test helpers from a dedicated test-support crate.

---

# 🚀 Build & Tooling

## 8. Formatting & Linting

Agents must run:

```
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

No PR should introduce new warnings.

## 9. CI Requirements

Before completing a task, ensure:

```
cargo check --workspace
cargo clippy --workspace --all-targets --all-features
cargo test --features test-utils
```

All commands must pass.

---

# 🗂 Module & File Conventions

## 10. Module Layout

* Avoid files larger than 10000 lines.
* Split large modules logically.
* Do not create deep module nesting (>3 levels) without justification.

## 11. Naming Conventions

* Types: `UpperCamelCase`
* Functions: `snake_case`
* Constants: `SCREAMING_SNAKE_CASE`
* Traits: adjective or capability-based naming (`Serializable`, `Executable`)

---

# ⚡ Performance Guidelines

* Avoid unnecessary heap allocations.
* Prefer iterators over intermediate `Vec` allocations.
* Avoid cloning large structures.
* Benchmark before optimizing.
* Use `#[inline]` only when profiling justifies it.

---

# 🧩 Async & Concurrency

* Use `tokio` runtime consistently across workspace.
* Do not mix async runtimes.
* Avoid blocking calls inside async functions.
* Use `spawn_blocking` when necessary.
* Prefer structured concurrency patterns.

---

# 🔐 Unsafe Code Policy

* Unsafe code is forbidden.

---

# 🧼 Refactoring Rules

Agents must:

* Preserve public API unless explicitly asked to change it.
* Avoid large refactors unless requested.
* Keep diffs minimal.
* Maintain backward compatibility.

---

# 📦 Public API Guidelines

* All public items must not have rustdoc comments.
* Avoid leaking internal types.
* Prefer explicit return types over `impl Trait` in public APIs.
* Keep API surface minimal.

---

# 🧭 Git & Commit Rules

* One logical change per commit.
* Do not mix formatting-only changes with logic changes.
* Commit messages must follow:

```
<crate>: short description

Optional longer explanation.
```

---

# 🚫 What Agents Must NOT Do

* Do not introduce hidden global state.
* Do not silently change behavior.
* Do not weaken type safety.
* Do not introduce panics.
* Do not bypass lints.

---

# 🏁 Completion Criteria

A task is complete only if:

* Code compiles for the entire workspace.
* Tests pass.
* Clippy passes with no warnings.
* Formatting is applied.
* No TODOs are left unless explicitly allowed.

---

End of AGENTS.md
