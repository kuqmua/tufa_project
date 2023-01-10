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
struct IHaveOtherNumbers {}
impl HasNumbers for IHaveOtherNumbers {
  const LAST_NUMBER: i32 = 6; 
  const NUMBERS: [i32; 5] = [1, 2, 3, 4, IHaveNumbers::LAST_NUMBER];
}
```
* const destructors
```
struct WillSayGoodbye<'a>(&'a str);
impl<'a> Drop for WillSayGoodbye<'a> {
  fn drop(&mut self) {
    println!("{}", self.0);
  }
}
const GOODBYE: WillSayGoodbye = WillSayGoodbye("bye");
fn main() {
  {
    let _goodbye = GOODBYE;
  }
}
```
* compile time functions as constants
```
const _: () = {
  //some logic in compile time
  ()
}
```
* another compile time functions as constants
```
const fn five_numbers() -> [i32; 5] {
  let mut numbers = [0i32; 5];
  
  // Note that for loops can not (yet) be used in constant functions.
  // So we have to go for a while loop;
  println!("execute");
  let mut i = 0;
  while i < 5 {
    numbers[i] = i as i32 + 1;
    i += 1;
  }
  numbers
}
fn main() {
  //compile time "execute message in console"
  const FIVE_NUMBERS: [i32; 5] = five_numbers();
  //run time "execute message in console"
  let five_numbers: [i32; 5] = five_numbers();
}
```
* const generic functions
```
const fn numbers<const N: usize>() -> [i32; N] {
  let mut numbers = [0i32; N];
  let mut i = 0;
  while i < N {
    numbers[i] = i as i32 + 1;
    i += 1;
  }
  numbers
}
fn main() {
  const TWO_NUMBERS = five_numbers::<2>();
  const TEN_NUMBERS: [i32; 10] = five_numbers();
}
```
* trait bounds on generic parameters for constant function
```
#[derive(Debug, Clone, Copy)]
struct Customer<'a> {
  name: &'a str,
  age: i32,
}

const CUSTOMER: Customer = Customer {
  name: "John",
  age: 42,
};

const fn nth<T: Copy, const N: usize>(items: [T; N], index: usize) -> T {
  items[index]
}

fn main() {
  const CUSTOMERS: [Customer; 2] = [
    Customer {
      name: "John",
      age: 30,
    },
    Customer {
      name: "Jane",
      age: 25,
    },
  ];
  const NTH_CUSTOMER: Customer = nth(CUSTOMERS, 1);
  println!("{:?}", NTH_CUSTOMER);
}
```
* dynamic dispatching
```
trait Animal {
  fn make_sound<'a>(&self) -> &'a str;
}
struct Cat {}
struct Dog {}
impl Animal for Cat {
  fn make_sound<'a>(&self) -> &'a str {
    "meow"
  }
}
impl Animal for Cat {
  fn make_sound<'a>(&self) -> &'a str {
    "woof"
  }
}
const fn favorite_animal() -> Animal {
  Cat {}
}
const fn animal_by_sound<'a>(can_purr: bool) -> &'a dyn Animal {
  match can_purr {
    true => &Cat {},
    false => &Dog {},
  }
}

fn main() {
  const FAVORITE_ANIMAL: &dyn Animal = &favorite_animal();
  println!("My favorite animal makes {}", FAVORITE_ANIMAL.make_sound());
  
  const PURRING_ANIMAL: &dyn Animal = animal_by_sound(true);
  println!("Animal that can purr say {}", PURRING_ANIMAL.make_sound());
}
```
 * Mutex now const(on nightly)
 ```
use std::sync::Mutex;
 
static ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
 
fn main() {
  {
    let mut arr = ARRAY.lock().unwrap();
    for _ in 0..10 {
      arr.push(1);
    }
  }
  println!("Called push {} times", ARRAY.lock().unwrap().len());
}
 ```
Wrapper around component or do not use function component also there is a way to use html without html!{} Macro
* how to create struct fields from different struct fields (mixin)
https://crates.io/crates/mixin
* cargo install cargo-show-asm
tool to view asm files in readable format
[video](https://youtu.be/lRV_5IBUTes) 10minute
* in docs function/struct/macro description must be maximum words, phrases and symbols what descripe function
otherwise user may not find it or know about what function doing
and need to give users possibility to add additional words/phrases to describe it
* tracing throught few microservises with error handlers
* runtime tests with bot loggin in some messanger
* try_ prefix if function can return Err or None
* move traits into own repository to reuse them properly
* use https://crates.io/crates/serde_yaml to control deps versions and features in different repos
* script\program for recursive submodule and repo git syncronization
* add api route to return info about all api routes map/tree in current service
