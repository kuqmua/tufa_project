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

//start client
* remove contast yellow lines (,make them less then 1px and grey/white)
* make buttons less contrast
* make buttons filled with background color
* make stule struct with methods like in flutter
* make common padding/margin constants
* expander as wrapper component
* material design spiner
* proc macro to implement component around all the buttons inside </br>
https://fonts.google.com/icons?icon.set=Material+Icons </br>
like make http, save icons, open them, rewrite and make component around it inside namespace
* same drawer overlay closing function with expander component
* pack trunk + rust into dockerimage </br>
https://www.youtube.com/watch?v=uYhLWN86V48&t=142s </br>
* antd alert animation not done yet
* use web-sys = { version = "0.3.4", features = ["console"] } </br>
web_sys::console::log() </br>
let msg = format!("roots: {:?}\nroots colors: {:?}", roots_serialized, roots_colors_serialized); </br>
web_console::log_1(&msg.into()); </br>
instead of gloo console log </br>
* maybe should try https://github.com/material-components/material-components-web
* maybe write this first before ant design https://github.com/orgs/react-component/repositories?q=&type=public&language=typescript&sort=
* (yew)Did not understand how to cast type Element to HtmlSelectElement(in web_sys crate HtmlSelectElement has selected_index method, Element does not)(feature was activated in cargo toml).
```
<select id="select_id">
        <option value="1">11</option>
        <option value="2">22</option>
    </select>
    <button id="btn">Button</button>
<script>
    const btn = document.querySelector('#btn');
    const sb = document.querySelector('#select_id')
    btn.onclick = (event) => {
        let b = sb.selectedIndex);//working
    };
</script>

let s: Option<i32> = match document.query_selector("select_id")) {
    Err(e) => None
    Ok(option_element) => match option_element {
        None => None,
        Some(element) => {//type Element
            let b = element.selected_index();//does not work
            None
        }
    },
}
```
* [yew events](https://yew.rs/docs/0.18.0/concepts/html/events)
//end client

* cargo audit command usage

//start server
* write logs into db in different service
Try two dbs: postgres and mongo. 
To write logs inside postgres need to add migrations
And know exactly what fields to write
And maybe fields can be null/empty

* add write logs in db into dir if provider doesnt respond to request 
What to use? Postgres? Clickhouse? Mongo

* write fetching result in file and check if there same posts or not

* providers statistics in files. if 3 time provider is inactive then do not fetch him next time then next two times...

* check every link's status code - maybe they rename it or something
only status without body

* do http reqwest again for some unsuccess http statuses in vec of unsuccess links
reqwest::StatusCode::REQUEST_TIMEOUT 
let wrong_cases_thread = thread::spawn(move || {
refetch logic

* compute post's hash and send hashes from client app to server to check server already send them or not 

* rename some local variables in functions

* service for date/time checking and executing arxiv for example one time per week and check of server restarted in this timestamp

* problem - now code waiting for all http reqwests to complete. rewrite it into event loop
Or message queue

* there is a problem with main arxiv link to check if provider available or not. and not only with arxiv

* do some work on better differences in colors in prints

* function to write save path from string and change some symbols
(Partially done)

* if size of working dir > 100mb then remove all containg

* write analog for twitter an use not only nitters
404 Not Found - CAUSE
This account's tweets are protected.
Only confirmed followers have access to @_KudoHiroyuki's tweets.

* https://doc.rust-lang.org/std/primitive.i32.html#method.checked_add
rewrite code in which there is a buffer overflow

* futures in some cases instead of threads (like file open or write in file)
Or tokio tasks

* thread pool instead of for loop

* let _ = join_all(vec_of_write_into_files_futures).await; //todo: add state of success/unsuccess

* if let Ok(something) = something.lock() {}
instead of 
something.lock().unwrap();
or match some none

* generate_biorxiv_hashmap_links and others rename this - remove hashmap

* thread pool with this let cpus = num_cpus::get();

* write some logic and flag what choose between env and in code constants for more efficient production variant 

* pub const PROJECT_MODE: &str = "Development";//later as ENV variable only

* todo: add medium

* pull and run postgres docker container
```
sudo docker run -p 5432:5432/tcp --name postgres-tufa-wsl2 -v ~/db-volumes/postgresql-volumes/tufa-dev-volume -e POSTGRES_PASSWORD=postgres -d postgres:latest
```
* how to change default volume folder for this command?
and mongo
and in dcoker-compose too

* find this in code and fix "(todo change this print)" 

* limit for mongo "get data" functions
get concrete number of provider links as function or command line or env

* #[deny(,   unwrap_used)]
files to write warnings:
rss_parse_string_into_struct
rss_fetch_and_parse_provider_data
rss_divide_to_equal_for_each_provider
rss_check_available_providers
get_providers_link_parts_from_mongo
generate_twitter_hashmap_links

* github parsing return in second parameter option<String, line! file!> to analize missing parse logic 

* implement get_providers_link_parts with all success completed and with not all

* get_providers_link_parts whole and for each provider
and maybe rewrite it as struct with methods

* Resource::PostgreSql => { 

* rename this check_new_posts_threads_parts

* 5 => {
                println!("todo 5 elements github parsing")
            }
github parsing

* add provider_kind into inner related to providers functions as input parameter

* twitter fetch Syntax: 2:97 Element atom:link prefix is unbound

* ProviderKind::Medrxiv => {
                            fetch_result_string.remove(0);
* rewrite it without extra allocation. like let f = fetch_result_string[1..]

* rewrite ProviderKind input params as function params in get_providers_link_parts_from_mongo

* check config values on non negative, overflow, more than capacity, not zero

* Add method to config struct what check integer value for correct value 
and use this method instead of use fields directly. 
Make fields private and use get/set. Check strings too

* todo write into mongo collection and create flag where to write logs
rss_write_error_logs_into_file_for_provider(file_name, json_object);

* add bool or option return type to create_dir_if_dont_exists. maybe it would be runtime error if i dont

* std::path::PathBuf instead if string in path to file logic

* if cannot connect to db then program not ending. just waiting. find out why and fix it

* add area of visibility / scope like this for all private functions/methonds
pub(in crate::get_project_information::get_config::structures_implementations::config_struct_impl)

* todo: find out why cannot write this path crate::get_project_information::get_config::structures_implementations::config_struct_impl::wrap_config_checks_impl

* https://github.com/rust-cv
rust image detection libs/crates

* maybe add this to code
#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]

* make default value for all environment variables
I DONT THINK ITS A GOOD IDEA
            
* dockerfile with ARM support

* rename MONGO_FOURTH_HANDLE_URL_PART and others into something more meaningfull 

* vec_of_values are not filtered in
mongo_insert_docs_in_empty_collection(
        mongo_url,
        db_name_handle,
        db_collection_handle, //fix naming later
        db_collection_document_field_name_handle,
        vec_of_values,
    )
all 2200+ twitter subs inserted
fix this by adding filter from env

* early return refactoring with ok_or and ?
example
fn bar1(x: Option<u64>) -> Result<u64, MyErrors> {
    let x = x.ok_or(MyErrors::SomeError)?;
    // A lot of stuff going on.
    Ok(x * 2)
}
            
* optimize for loop for std::env::var to std::env::vars
but will it be safe?

* env vars names - make it private

* seaORM instead of diesel?
https://www.sea-ql.org/SeaORM/
Diesel	SeaORM
Sync	Async
Static	Dynamic
Native Driver	Pure Rust
Relational
Schema first
MySQL / Postgres / SQLite
NOPE, NOT STABLE YET
check it in the middle of 2022

* restructure config with prints Structure fields (together some inner structs)
            
* remove unwrap() into expect() to give more meaning
except maybe loops or early return cases 

* warning: variant is never constructed: `PostgreSql`
  --> src/providers_info/get_project_information/get_providers_link_parts.rs:62:5
   |
62 |     PostgreSql,
   |     ^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

* Result instead of option in all code possible coz return Option loosing information about error 

* big enum of enums(recursive) of all errors in project

* move providers link parts (specific urls) to project constants

* if error size > then ok return type then allocate error on heap, otherwise actual return type would be more then needed
maybe write some tests or linter or lib or rustanalyzer flag to check it

* Rename all functions return type to mean something.
type = ....

* wrap blocking diesel function into spawn blocking under tokio runtime
(rustacean station - tokio ecosystem with alice ryhl) 24:30

* procedural macros for config fields like vec![]. with correct naming require
without typed enums
like 
Config {
  test_field: from_env!(test_field.to_upper_snake_case()) 
}

* reuse files with functions and constants using #[path=""] instead of module system
its maybe bad practice but must work

* try SNAFU as error handling crate 
https://crates.io/crates/snafu

* LINKS_LIMIT_PROVIDERS not working now. fix it

* parameter db initialization provider link parts source must be
different than PROVIDERS_LINK_PARTS_SOURCE 

* divide and move outside postgres and mongo methods from provider_kind_methods 

* rewrite prints like this
let person = get_person();
// ...
println!("Hello, {person}!"); // captures the local `person`
  
* fs::remove_dir_all(&path)?; //todo: its blocking, rewrite to async
  
* inside get_all_local_providers_data
get_enabled_providers_vec should be get_enabled_initialiation_providers_vec. add additional vars into env file

* docker build image inside ci pipeline

* use Rc instead of Arc in cases with one thread.
Between threads still must use Arc

* can implement this
impl User<u32> {

}
meaning implementation only for u32 type
  
* use checked_sub() and checked_add() methods

* in cases across await points use tokio::sync::Mutex instead of std Mutex
use this lint 
https://rust-lang.github.io/rust-clippy/master/#await_holding_lock
  
* links limits and other getters functions like before on config new initialization (before proc macro)

* add connect_timeout to all mongo client_options usage
  
* add time to errors like this ./server/utils/dependencies.ts:524:17  15:30:40 25.01.22 
(partially done)

* add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
  
* https://blog.turbo.fish/proc-macro-error-handling/

* actix-web html + session-based-authentication
https://www.lpalmieri.com/posts/session-based-authentication-in-rust/
  
* there is a function to handle request error status inside reqwest crate
.error_for_status()

* add check on ports fields from env or config to be not equal 0-1000 range
  
* routes 
/api/html/info
/api/json/info
/api/protobuf/info

* cut enum extension into few different proc macroses
* write match should trace inside init or new function of error initialization
* env var json tracing on/off
* different formatting for env var json tracing on/off

//end server

//start server tests
* parsing providers posts tests

* server what running web tests every hour like http requests and get valid data or not
github parsing version to check different structs of posts as test in the loop

* Add tests for new libraries versions and what version i use. 
Like http request to crates.io or something similar

* write test what checks if something missing in .dockerignore and .gitignore - local or ci? 

* Write tests what checks all function to have #![deny(,   unwrap_used)]

* test to check what cargo run executes in the right folder

* test what check commit name for only eng symbols

* #[should_panic(expected = "99")]
write some tests with with macro

* Right test what parse repo code and check what each function returns some type or enum

* create big json file to test parsing speed as test

* test for all env var to working and usage

* is there some test implementation that checks all your rust project functions (with result return type) on difference between return type in case of ok and error? i mean for cases where size of ok must be >= size of error. if otherwise im going to use Box(Error) in case of error. i think it can be done with big proc macro that expand test.

* test or proc macro to name error function type like function but with CamelCase and ErrorStruct on the end

* check all files names to not contain ":" symbol. its needed to correct parsing line

* check all files name be equal to his inner struct/function

* try to use nexttest instead of cargo test
https://nexte.st/
https://github.com/nextest-rs/nextest
//end server tests

//start server thoughts
* add everywhere explicit types
Except maybe functional way 

* 
```
std::stringify
```

* 
```
std::str::SplitN
```

* 
```
split().map()
instead of find(<item></item>)
```

* error had .description method
maybe rewrite error messages?
But description is depreciated

* hashmaps into vecs

* &str instead of String everywhere if possible

* 'static remove if possible or maybe add if its better 

* ensure! macros

* RES.STATUS: 521 <unknown status code>

* operation timed out print time elapsed

* 
```
Rc<RefCell<Data>>
Rc::new(<RefCell::new(Data { value: 42}))
```

* find out why constants which used in tests looks like unused

* google some style guide variable naming for project(parsing espesially) and rename variables

* [dependencies] and [dev-dependencies] read about it

* 
```
thread '<unnamed>' panicked at 'twitter_provider_names is empty!!!', libs/providers_info_lib/src/get_project_information/generate_hashmap_links/generate_twitter_hashmap_links.rs:7:9
+++++++++++++++++++++++++++
Finished dev [unoptimized + debuginfo] target(s) in 0.10s
Running `target/debug/tufa_server`
We are on a multicore system with 12 CPUs
ENV: Development
server can reach https://www.google.com/
20 elements in Twitter HashMap
i can reach https://www.google.com/
file: src/fetch/rss_check_available_providers.rs:32
UnhandledFetchStatusInfo::Failureerror sending request for url (https://nitter.fdn.fr/TheCherno/rss): error trying to connect: tcp connect error: Network is unreachable (os error 101)
thread '<unnamed>' panicked at 'twitter_provider_names is empty!!!', libs/providers_info_lib/src/get_project_information/generate_hashmap_links/generate_twitter_hashmap_links.rs:7:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any', src/check_new_posts_threads_parts.rs:418:18ed
```

* parts of config as feature flags? 

* is there some service whats run postgres and mongo in cloud for free like 100mb for free?

* Dynamic library (dylib)
Find out about it 
crate-type = ["dylib"]

* difference between "".to_string() and String::new()

* refactor project into rust workspaces to reuse dependencies

* caret requirements 
tilde requirements
wildcard requirements

* different user_credentials files for project databases and providers
Move all from user_credentials into .env file

* config as dynamic library maybe?

* Faster docker build 
Cargo chef
https://github.com/LukeMathWalker/cargo-chef

* Redis rust library
https://github.com/mitsuhiko/redis-rs

* AVIF imag format in some cases better than webp, png and jpeg

* clippy rust flags (can use deny for them - code would be more safe)
https://efanzh.org/2021/05/25/rust-lints.html

* global .gitignore
https://www.youtube.com/watch?v=D97rnxDqq1I

* #[deny(  unwrap_used)]
Find out why its triggers for tokio:main functions

* rewrite something with "from" and "into" traits

* check rust-analyzer flags to enable
    
* "Rust Doc Viewer" vscode extension

* check if i have dependency with std::net in code. (vulnerability in std::net), after Rust 1.53.0 no problem with that
    
* find out how to use ngnix
    
* add to each function return bool if its not returning yet

* how to solve problem with two different versions of tokio runtime? main 1.7.1, for mongo 0.2.25

* find out dofference between diesel vs tokio-postgres vs sqlx

* find out how tokio runtime works if there is no join all method and will it be actuall in parallel or not

* add stackoverflow provider
https://stackoverflow.com/feeds/question/23412033
    
* difference between usage in global scope and local scope
use lib_or_crate_or_something::something;

* https://github.com/tangramdotdev/tangram
train, deploy, and monitor machine learning models

* Add different project in inner folder of tufa backend 
what parse few rust files, dockerfile, docker-compose and .env 
And add new env variable and logic around it in all that files
Then run tests and cargo check
    
* pull requests instead of commits (+ github actions) - do not add changes if test fail
    
* move from ubuntu to alphine linux on wsl2
    
* if i go into docker container and modify file, will changes be saved after container restart?
if yes then maybe write some send logs logic around it?
like check last logs send date and time then decide send or not send

* add to dockerfile or before docker build test what check valid env data fields

* pm2 instead of docker?
    
* commit message and hash in file inside docker container
for more information about code inside container
    
* red hat / openshift / reg.ru postgres cloud cluster

* rename ERROR_RED="255" # u8 into ERROR_PRINT_COLOR_RED and others

* rust language detection library
https://github.com/pemistahl/lingua-rs

* https://doc.rust-lang.org/std/macro.stringify.html

* find some "try to remove .clone() from rust code" exercises

* This week in rust check 

* Hazard pointer https://youtu.be/2Iu2BnO9iHg

* Type safe relationship between enums.
We have two different enums Number and ErrorNumber
And we can write some matching for them
Like Number::One => ErrorNumber::One
This can be realized safetly as tree
Like implement get_error for Number or something
It can be graph but we can mismatch something like
Number::One => ErrorNumber::One
Number::Two => ErrorNumber::One
Its not a  TYPE SAFE Graph, coz it's compiled successfully
It must be many to many Graph-enum-type-system
Ask someone about it

* find out what happens in function can return some error
but you use ? operator on result and await cases

* Complete all exercises
https://github.com/rust-lang/rustlings

* learn more about iterator methods
```
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.into_iter().filter(|pair| pair.1 == &value).count()
    --------------
    WHY THIS NOT WORKING?
    map.into_iter().filter(|(key, val)| val == &value).count()
}
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.into_iter().fold(0, |result, map| {
        result + map.into_iter().filter(|pair| pair.1 == &value).count()
    })
}
```

* learn more about rust macro system
```
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!")
    };
}
```

* AsRef trait find out more
```
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}
``` 

* error conversion with ? example
```
use std::num::ParseIntError;
use std::str::FromStr;
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}
impl From<CreationError> for ParsePosNonzeroError {
    fn from(e: CreationError) -> Self {
        ParsePosNonzeroError::Creation(e)
    }
}
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(e: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(e)
    }
}
impl FromStr for PositiveNonzeroInteger {
    type Err = ParsePosNonzeroError;
    fn from_str(s: &str) -> Result<PositiveNonzeroInteger, Self::Err> {
        let x: i64 = s.parse()?;
        Ok(PositiveNonzeroInteger::new(x)?)
    }
}
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}
impl PositiveNonzeroInteger {
    #[must_use]
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            PositiveNonzeroInteger::from_str("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }
    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::from_str("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }
    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::from_str("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }
    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(PositiveNonzeroInteger::from_str("42"), Ok(x.unwrap()));
    }
}
```
    
* find out more about downcast errors

* .map_err() function in code examples

* use Option ref to remove borrow error
```
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
```
    
* rewrite error handling like this?
```
    if let Err(merge_errors) = builder.merge_pipelines() {
        errors.extend(merge_errors);
    }
```

* how to clone custom error? which contains std error or lib errors

* crate for git repos integration 
```
git2 = "0.13"
```
    
* impl serde::Serialize and deserialize for structs and enums

* learn https://github.com/Amanieu/parking_lot
    
* systemclt is-enabled docker
    
* read about async cancelation
https://blog.yoshuawuyts.com/async-cancellation-1/

* read about debugging Rust application inside linux container
https://blog.erebe.dev/blog/debug-rust-aplication-inside-container/index.html

* read how to write async tests
https://docs.rs/tokio/1.14.0/tokio/attr.test.html
  
* Checking Unused Dependencies in a Rust Project with Github Actions
https://erayerdin.com/checking-unused-dependencies-in-a-rust-project-with-github-actions-ckwm3yov901cwlvs1h48z54xi
    
* write about dbs, tables in postgres and clusters dbs and collections in mongo
should i create table for each provider?
should i create collection for each provider?
    
* rust cache github action
https://github.com/marketplace/actions/rust-cache
    
* rust macros book
https://veykril.github.io/tlborm/syntax-extensions/source-analysis.html

* how to do error handling:
```
pub struct Error(Box<ErrorImpl>);

enum ErrorImpl {
     EverythingWentBad,
     Aaaaaaaaaaa,
}
```
if size_of::<ErrorImpl>() > size_of::<usize>() - its rare
and it is assumed that errors are rare 
and it is assumed that with such an error T inside Result<T, Error> 
if this return true
size_of::<ErrorImpl>() > size_of::<T>()... 
then do Box<>

example from serde_json
https://github.com/serde-rs/json/blob/3f459308f5055e9a4b1b611a77dad07132011e8d/src/error.rs#L16-L19

https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant 
linter for it big error enums

* while using Result<SomeType, SomeError>
if size_of::<SomeError> > size_of::<Box<SomeError>> (i think its == usize (32 or 64))
then using Box version will be allocation on runtime.
so if errors will be inside loop or multiple parallel tasks
program can allocate on runtime...
the question IS: how many allocations program must do on runtime to slow down the program?
what the difference between 
*1 a little less stack size on runtime with possible allocations( with what % of execution time)
*2 a bigger stack size without possible allocations on runtime
so we need to implement 2 versions inside program
and switch between them watching error allocation statistics

...also write different error types for allocation trottling cases
and unefficiency stack-based version.
like 
big error version for lower allocation statistics
small error (yeah with less info about error) version for high allocation statistics
(in small version still allocate big error version from time to time - for more info)
    
* rust zero to production book 
https://github.com/LukeMathWalker/zero-to-production
    
* tokio::fs::metadata
check path exists in file system
    
* let a = async { 10 }.delay(Duration::from_secs(10));
let b = async { 11 };
try using race for different dbs or local files.
for example get provider link parts from mongo postgres local using race
assert_eq!(a.race(b).await, 11);

race - wait for first output, return early on error
join - wait for all ouputs, continue on error
try_join - wait for all outputs, return early on error
try race - wait for first output, continue on error
https://www.youtube.com/watch?v=QlPDI9IsSXU&list=WL&index=99
    
* Overriding Rust Dependencies
https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html
    
* add tracing::error? https://crates.io/crates/tracing-error
*update* dont know how to use it

* find about rust type erasure. write some examples

* rust error handling from zero to prod rust author https://www.youtube.com/watch?v=yLGLKKFFc3g  
   
* find out difference between sqlx and tokio-postgres
https://crates.io/crates/tokio-postgres
https://crates.io/crates/sqlx
    
* show github link to code repo from error  
    
* add field how_to_fix  error into error  

* async runtime error handling state for write/dont write/partially write into console error info
for example in case then error repeats
first time write all error 
then write partially.
or maybe write only error increment
or make more time between function exection that can return this error
    
* do not use Mutex.lock() with match expression
like match mutex.lock() {}
explanation https://fasterthanli.me/articles/a-rust-match-made-in-hell

* deadlock detection with parking_lot
```
parking_lot = { version = "0.12.0", features = ["deadlock_detection"] } 
```
just find this inside page to see example
https://fasterthanli.me/articles/a-rust-match-made-in-hell
  
* Rust Macro Expand vscode extension
https://marketplace.visualstudio.com/items?itemName=Odiriuss.rust-macro-expand
   
* https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=
 
* Cargo now shows you warnings when a dependency will be rejected by a future version of Rust. After running cargo build or cargo check, you might see:
warning: the following packages contain code that will be rejected by a future version of Rust: old_dep v0.1.0
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
You can run the cargo report command mentioned in the warning to see a full report of the code that will be rejected. This gives you time to upgrade your dependency before it breaks your build.
    
* On Linux, you can induce a system reboot by writing a b to /proc/sysrq-trigger (if you’re going to try this you probably want to use a VM):
echo b > /proc/sysrq-trigger
We can sort of simulate an operating system level crash (or having the plug pulled on our server rack) by using this:
```
pub fn reboot_kernel() {
    File::create("/proc/sysrq-trigger")
        .unwrap()
        .write_all(b"b")
        .unwrap();
}
```

* Rust exposes this system call via the File::sync_all method. If we call sync_all, when it returns successfully, the operating system promises that the data has been written to disk and will exist on disk even in the event of a hard crash like a power failure or OS crash. As I’ve tried to emphasize repeatedly, this is an oversimplification, and many things can go wrong depending on your filesystem configuration, but for today, let’s keep things simple and assume syncing is a reliable process.

* show llvm ir command
rustc simple1.rs --emit=llvm-ir
    
* open api routes ?

* rust template gitignore
https://github.com/github/gitignore/blob/main/Rust.gitignore
    
* Sometimes you need to write methods that accept a string slice (&str) and conditionally return either a modified version of it or the original one. For these cases, you might use Cow<str>, so that you only allocate new memory when necessary.
```
use std::borrow::Cow;
fn capitalize(name: &str) -> Cow<str> {
    match name.chars().nth(0) {
        Some(first_char) if first_char.is_uppercase() => {
            // No allocation is necessary, as the string
            // already starts with an uppercase char
            Cow::Borrowed(name)
        }
        Some(first_char) => {
            // An allocation is necessary, as the old string
            // does not start with an uppercase char
            let new_string: String = first_char.to_uppercase()
              .chain(name.chars().skip(1))
              .collect();
            Cow::Owned(new_string)
        },
        None => Cow::Borrowed(name),
    }
}
fn main() {
    println!("{}", capitalize("bob"));   // Allocation
    println!("{}", capitalize("John"));  // No allocation
}
```
If you’re using VSCode + Rust Analyzer, I highly suggest going into the settings > Rust Analyzer > Check On Save: Command and setting "clippy" as the new default instead of "check". Same UX, better warnings.

* rust reverse ranges problem https://kaylynn.gay/blog/post/rust_ranges_and_suffering
    
* inside cargo.toml
use the “*” to pull the latest version of the dependency.
    
* more readable tracing like a tree https://lib.rs/crates/tracing-tree
examples https://fasterthanli.me/articles/request-coalescing-in-async-rust
    
* usefull crates:
https://crates.io/crates/base64 (encodes and decodes base64 as bytes or utf8)
https://crates.io/crates/clap (command line parser)
https://crates.io/crates/signal-hook (Unix signal handling)

* If your function is propagating the error upstream (e.g. using the ? operator), it should not log the error. It can, if it makes sense, add more context to it.
    
* algebraic data types inside postgres
https://stackoverflow.com/questions/44431740/algebraic-data-types-in-postgres
    
* This library provides implementations of Mutex, RwLock, Condvar and Once that are smaller, faster and more flexible than those in the Rust standard library, as well as a ReentrantMutex type which supports recursive locking. It also exposes a low-level API for creating your own efficient synchronization primitives.
https://lib.rs/crates/parking_lot
    
* 
```
let x = vec![(1, 2), (3, 4), (5, 6)].into_iter();
let _: HashMap<u64, u64> = HashMap::from_iter(x);
```
    
* writing cross-platform “bash” scripts in Rust
https://docs.rs/xshell/latest/xshell/
maybe rewrite shell scripts for service init in xshell?

* some useful methods
let v = vec!["Hello", "World", "!"].into_iter();
let w: Vec<String> = v.map(String::from).collect();
into_par_iter()
inspect
chain
fold
for_each_concurrent
buffer_unordered
 
* gen rust files
https://matklad.github.io/2022/03/26/self-modifying-code.html
    
* Building REST API's with Rust, Actix Web and MongoDB
https://dev.to/feezyhendrix/building-rest-apis-with-rust-actix-web-and-mongodb-4lbf?utm_campaign=Rust%2BNigeria%2BNewsletter&utm_medium=web&utm_source=Rust_Nigeria_Newsletter_4

* disable unsafe code
#![forbid(unsafe_code)]
    
* Tiny and Fast Docker image for Rust Application
https://azzamsa.com/n/rust-docker/

* Rust grpc tutorial
https://youtu.be/JkSa-qA2jnY

* [onnx-common NN format standart, runtime for neural net](https://docs.rs/onnxruntime/latest/onnxruntime/)

* [2 neural nets for text and images](https://openai.com/blog/clip/)

* using vector in tracing macro example https://github.com/tokio-rs/tracing/blob/v0.1.x/examples/examples/valuable_json.rs

* parallel find in rust ?

* habr rss https://habr.com/ru/rss/post/553356/https://habr.com/ru/rss/post/553356/?with_hubs=true:?with_tags=true:?limit=100

* https://github.com/baptiste0928/cargo-install

* maybe use std::any::type_name::<>() for printing type names

//end server thoughts

* use rust lints declarations in Cargo.toml https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-lints-section https://doc.rust-lang.org/stable/cargo/reference/workspaces.html#the-lints-table
