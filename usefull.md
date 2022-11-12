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

