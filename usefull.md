* serde deserialization alternative, but more flexible with macroses [crate](https://crates.io/crates/frunk) [video](https://www.youtube.com/watch?v=Zps2tH8XOm4&list=WL&index=144&ab_channel=%D0%92%D0%B8%D0%B4%D0%B5%D0%BE%D1%81%D0%BA%D0%BE%D0%BD%D1%84%D0%B5%D1%80%D0%B5%D0%BD%D1%86%D0%B8%D0%B9IT-People) 
* something else
* minimized binary sized https://github.com/johnthagen/min-sized-rust
* type_description https://crates.io/crates/type_description
* compile time concats
```
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
* [nine rules for creating procedural macros in rust(carl kadie)](https://www.youtube.com/watch?v=zkk0Hyzm30E&list=WL&index=175&t=1132s)
* cargo expand can put generated content into file
```
cargo expand something::something --lib > generated.rs
```
* [rust type state builder pattern](https://github.com/jeremychone-channel/rust-builder)

* [5 Better ways to code in Rust](https://www.youtube.com/watch?v=BU1LYFkpJuk)

* [crate to validate string length and other values](https://crates.io/crates/validator)

* const check init

```
#![feature(inline_const)]

struct S{ 
    one: std::string::String,
    two: i32
}
impl S {
    #[must_use]
    pub fn new(one: &str, two: i32) -> Self {
        //case2 - error[E0435]: attempt to use a non-constant value in a constant
        // let one = const { S::one_not_batman(one) };
        // let two = const { S::two_not_negative(two) };
        S{
            one: one.to_string(),
            two
        }
    }
    pub const fn one_not_batman(one: &str) -> &str {
        if matches!(one.as_bytes(), b"batman") {
            panic!("one can't be a batman")
        }
        one
    }
    pub const fn two_not_negative(two: i32) -> i32 {
        if two <= 0 {
            panic!("two can't be a negative")
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

* next iteration of const init

```
#![feature(inline_const)]
mod something {
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Something {
        one: std::string::String,
        two: i32,
    }
    pub struct InputCheckedSomething<'a> {
        one: &'a str,
        two: i32,
    }
    pub struct UncheckedSomething<'a> {
        pub one: &'a str,
        pub two: i32,
    }
    #[derive(Debug)]
    pub enum SomethingError {
        IsBatman,
        IsNegative,
    }
    impl Something {
        const fn check(value: UncheckedSomething) -> Result<UncheckedSomething, SomethingError> {
            if matches!(value.one.as_bytes(), b"batman") {
                return Err(SomethingError::IsBatman);
            }
            if value.two <= 0 {
                return Err(SomethingError::IsNegative);
            }
            Ok(value)
        }
        pub fn runtime_new_with_compile_time_check(value: InputCheckedSomething) -> Self {
            Self {
                one: value.one.to_string(),
                two: value.two,
            }
        }
        pub fn try_run_time_new_with_runtime_check(
            unchecked_value: UncheckedSomething,
        ) -> Result<Self, SomethingError> {
            match Self::check(unchecked_value) {
                Ok(value) => Ok(Self {
                    one: value.one.to_string(),
                    two: value.two,
                }),
                Err(error) => Err(error),
            }
        }
        pub const fn compile_time_check(
            unchecked_value: UncheckedSomething,
        ) -> InputCheckedSomething {
            match Self::check(unchecked_value) {
                Ok(value) => InputCheckedSomething {
                    one: value.one,
                    two: value.two,
                },
                Err(error) => match e {
                    SomethingError::IsBatman => panic!("is batman"),
                    SomethingError::IsNegative => panic!("is negative"),
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
        .unwrap();
    println!("123");
}
```

* -- create or update operation i think is irrelevant coz i think its not working properly with GENERATED ALWAYS AS IDENTITY PRIMARY KEY
* few interesting ideas from grpc implementation api https://www.youtube.com/watch?v=zPbaKUIcFx0&t=2889s
* postgresql memory limits https://vremont-e.ru/ogranicenie-razmera-xranimyx-dannyx-v-postgresql/
* found a problem. is it possible to make reexport serde from module, from which we do ProcMacroDeriveExample reexport? serde in serde::Serialize proc macro doing import from serde, 
but in places where ProcMacroDeriveExample actually used - serde can be not included in Cargo.toml. I want to do something like "pub use serde::Serialize", but inside generated by serde::Serialize
tokens there is still import from serde (like "serde::"). Thats why in all crates what uses ProcMacroDeriveExample i need to put serde in Cargo.toml. Its bad, there is no module-like architecture
```
#[proc_macro_derive(ProcMacroDeriveExample)]
pub fn proc_macro_derive_example(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generated = quote::quote! {
        #[derive(Debug, serde::Serialize)]
        pub struct Something {
            pub something: std::string::String,
        }
    };
    generated.into()
}
```