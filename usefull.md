# Useful Rust Resources and Code Snippets

A collection of useful Rust resources, libraries, patterns, and code examples.

## Table of Contents
- [Serialization and Deserialization](#serialization-and-deserialization)
- [Binary Size Optimization](#binary-size-optimization)
- [Type Information](#type-information)
- [Compile-time String Concatenation](#compile-time-string-concatenation)
- [Procedural Macros](#procedural-macros)
- [Cargo Tools](#cargo-tools)
- [Builder Pattern](#builder-pattern)
- [Best Practices](#best-practices)
- [Validation](#validation)
- [Compile-time Init](#compile-time-init)
- [gRPC Implementation](#grpc-implementation)
- [PostgreSQL](#postgresql)
- [Procedural Macro Reexports](#procedural-macro-reexports)
- [Run Scripts](#run-scripts)
- [Cargo Expand](#cargo-expand)
- [Algorithms](#algorithms)
- [AI with Rust](#ai-with-rust)
- [Fonts](#fonts)

## Serialization and Deserialization

[Frunk](https://crates.io/crates/frunk) - A serde deserialization alternative, but more flexible with macroses

Video: [Frunk presentation](https://www.youtube.com/watch?v=Zps2tH8XOm4&list=WL&index=144&ab_channel=%D0%92%D0%B8%D0%B4%D0%B5%D0%BE%D1%81%D0%BA%D0%BE%D0%BD%D1%84%D0%B5%D1%80%D0%B5%D0%BD%D1%86%D0%B8%D0%B9IT-People)

## Binary Size Optimization

Minimized binary sized: [min-sized-rust](https://github.com/johnthagen/min-sized-rust)

## Type Information

[type_description](https://crates.io/crates/type_description) - Crate for type information

## Compile-time String Concatenation

```rust
use std::io::Read;
struct Buf<const N: usize>([u8; N]);
const fn len(strs: &[&str]) -> usize {
    let mut result = 0;
    let mut remaining = strs;
    while let [current, tail @ ..] = remaining {
        result += current.len();
        remaining = tail;
    }
    result
}
const fn concat<const N: usize>(strs: &[&str]) -> Buf<N> {
    let mut buffer = [0; N];
    let mut position_in_buffer: usize = 0;
    let mut remaining = strs;
    while let [current, tail @ ..] = remaining {
        let x = current.as_bytes();
        let mut i = 0;
        while i < x.len() {
            buffer[position_in_buffer] = x[i];
            position_in_buffer += 1;
            i += 1;
        }
        remaining = tail;
    }
    Buf(buffer)
}
macro_rules! my_concat {
    ($($x: expr),+ $(,)?) => {{
        const STRS: &[&str] = &[$($x), +];
        const LEN: usize = len(STRS);
        const CONCAT_BUF: Buf<LEN> = concat(STRS);
        unsafe { core::str::from_utf8_unchecked(&CONCAT_BUF.0)}
    }}
}
```

## Procedural Macros

Video: [Nine rules for creating procedural macros in Rust (Carl Kadie)](https://www.youtube.com/watch?v=zkk0Hyzm30E&list=WL&index=175&t=1132s)

## Cargo Tools

Cargo expand can put generated content into file:
```bash
cargo expand something::something --lib > generated.rs
```

## Builder Pattern

[Rust Type State Builder Pattern](https://github.com/jeremychone-channel/rust-builder)

## Best Practices

Video: [5 Better ways to code in Rust](https://www.youtube.com/watch?v=BU1LYFkpJuk)

## Validation

Crate to validate string length and other values: [validator](https://crates.io/crates/validator)

## Compile-time Init

### Basic Example

```rust
#![feature(inline_const)]

struct S{ 
    one: String,
    two: i32
}
impl S {
    #[must_use]
    pub fn new(one: &str, two: i32) -> Self {
        //case2 - er[E0435]: attempt to use a non-constant value in a constant
        // let one = const { S::one_not_batman(one) };
        // let two = const { S::two_not_negative(two) };
        S{
            one: one.to_string(),
            two
        }
    }
    pub const fn one_not_batman(one: &str) -> &str {
        if matches!(one.as_bytes(), b"batman") {
            panic!("ac209d5a-73f8-4c63-a7d2-cc395a752cf7")
        }
        one
    }
    pub const fn two_not_negative(two: i32) -> i32 {
        if two <= 0 {
            panic!("acfdf18f-7d4a-47f9-bf23-0e7fd1896145")
        }
        two
    }
}
fn main() {
    //case1 - works
    // let _s = S::new(
    //     const { S::one_not_batman("notbatman") },
    //     const { S::two_not_negative(1) }
    // );
    //case2
    let _s = S::new(
        "notbatman",
        10
    );
}
```

### Advanced Example

```rust
#![feature(inline_const)]
mod something {
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Something {
        one: String,
        two: i32,
    }
    pub struct InputCheckedSomething<'lifetime> {
        one: &'lifetime str,
        two: i32,
    }
    pub struct UncheckedSomething<'lifetime> {
        pub one: &'lifetime str,
        pub two: i32,
    }
    #[derive(Debug)]
    pub enum SomethingEr {
        IsBatman,
        IsNegative,
    }
    impl Something {
        const fn check(v: UncheckedSomething) -> Result<UncheckedSomething, SomethingEr> {
            if matches!(v.one.as_bytes(), b"batman") {
                return Err(SomethingEr::IsBatman);
            }
            if v.two <= 0 {
                return Err(SomethingEr::IsNegative);
            }
            Ok(v)
        }
        pub fn runtime_new_with_compile_time_check(v: InputCheckedSomething) -> Self {
            Self {
                one: v.one.to_string(),
                two: v.two,
            }
        }
        pub fn try_run_time_new_with_runtime_check(
            v0: UncheckedSomething,
        ) -> Result<Self, SomethingEr> {
            match Self::check(v0) {
                Ok(v1) => Ok(Self {
                    one: v1.one.to_string(),
                    two: v1.two,
                }),
                Err(er) => Err(er),
            }
        }
        pub const fn compile_time_check(
            v0: UncheckedSomething,
        ) -> InputCheckedSomething {
            match Self::check(v0) {
                Ok(v1) => InputCheckedSomething {
                    one: v1.one,
                    two: v1.two,
                },
                Err(er) => match er {
                    SomethingEr::IsBatman => panic!("e4778c18-87e3-40f7-977b-a27473ce786b"),
                    SomethingEr::IsNegative => panic!("d434f5b3-0b0e-477b-aff0-d17113d0cf18"),
                },
            }
        }
    }
}
fn main() {
    let _compile_time_params_check = something::Something::runtime_new_with_compile_time_check(
        const {
            something::Something::compile_time_check(something::UncheckedSomething {
                one: "notbatman",
                two: 1,
            })
        },
    );
    let _runtime_params_check =
        something::Something::try_run_time_new_with_runtime_check(something::UncheckedSomething {
            one: "notbatman",
            two: 1,
        })
        .expect("b4ccb35f-30a2-4c9b-b449-4a6c8c0b8f35");
    println!("123");
}
```

## gRPC Implementation

Interesting ideas from gRPC implementation API: [Video](https://www.youtube.com/watch?v=zPbaKUIcFx0&t=2889s)

## PostgreSQL

PostgreSQL memory limits: [Article (Russian)](https://vremont-e.ru/ogranicenie-razmera-xranimyx-dannyx-v-postgresql/)

## Procedural Macro Reexports

Found a problem. Is it possible to make reexport serde from module, from which we do ProcMacroDeriveExample reexport? serde in serde::Serialize proc macro doing import from serde, but in places where ProcMacroDeriveExample actually used - serde can be not included in Cargo.toml. I want to do something like "pub use serde::Serialize", but inside generated by serde::Serialize tokens there is still import from serde (like "serde::"). Thats why in all crates what uses ProcMacroDeriveExample i need to put serde in Cargo.toml. Its bad, there is no module-like architecture

```rust
#[proc_macro_derive(ProcMacroDeriveExample)]
pub fn proc_macro_derive_example(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generated = quote! {
        #[derive(Debug, serde::Serialize)]
        pub struct Something {
            pub something: String,
        }
    };
    generated.into()
}
```

## Run Scripts

Prepare run scripts like with npm:
```bash
cargo install cargo-run-script
```

Run custom cargo package.metadata.script:
```bash
cargo run-script *script name*
```

## Cargo Expand

Cargo expand issue (works here only with --lib flag):
```bash
cargo expand your::path::to::module --lib
```

## Algorithms

[All algorithms written in Rust](https://github.com/TheAlgorithms/Rust)

## AI with Rust

[Use AI with Rust](https://youtu.be/StMP7g-0wK4)

## Fonts

Fonts source: [Google Fonts](https://fonts.google.com/)