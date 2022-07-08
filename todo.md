* rust web app example <br/>
https://github.com/saschagrunert/webapp.rs <br/>
* 60% faster tests run than cargo test <br/>
Cargo nexttest run <br/>
https://nexte.st/ <br/>
* share cache between workspaces <br/>
https://github.com/mozilla/sccache <br/>
* .md file to html converter <br/>
https://dillinger.io/ <br/>
use this inside /api/info <br/>
maybe write proc macro? <br/>
1 make http request to https://dillinger.io/ <br/>
2 write markdown <br/>
3 convert to html <br/>
4 download styled html <br/>
5 open html <br/>
6 write contant as String <br/>
7 put some variables inside String <br/>
OR MAYBE USE HTML FROM CARGO DOC? <br/>
mayb use nodemon cargo doc (maybe some params like exclude) <br/>
* another rust web dev book <br/>
https://www.manning.com/books/rust-web-development <br/>
* find out how to restrict user(or something) in postgres <br/>
to not have foreign key creation possibility to the choosen by you table <br/>
queries to the choosen table too <br/>
* `iter()` for vecs yields `&i32`.
let mut iter = vec1.iter();
`into_iter()` for vecs yields `i32`.
let mut into_iter = vec2.into_iter();
`iter()` for vecs yields `&i32`, and we want to reference one of its
items, so we have to destructure `&&i32` to `i32`
println!("Find 2 in vec1: {:?}", iter .find(|&&x| x == 2));
`into_iter()` for vecs yields `i32`, and we want to reference one of
its items, so we have to destructure `&i32` to `i32`
println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));
* parallel iteration example
```
extern crate rayon;
use rayon::prelude::*;
fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
```
* for loop into iterators?
* wasm server components <br/>
https://spin.fermyon.dev/
* benchmarks </br>
https://youtu.be/eIB3Pd5LBkc
* require constants inside trait and init them by default
```
trait HasNumbers {
  const NUMBERS = [i32; 5];
  const LAST_NUMBER: i32 = 5;
}

struct IHaveNumbers {}

impl HasNumbers for IHaveNumbers {
  const NUMBERS: [i32; 5] = [1, 2, 3, 4, IHaveNumbers::LAST_NUMBER];
}
```

