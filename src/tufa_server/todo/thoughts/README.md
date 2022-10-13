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
impl UserCredentialsStruct {
    pub fn new() -> Result<Self, ConfigError> {
        maybe add different user logic later ?
```

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

* #[deny(clippy::unwrap_used)]
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

* Compile on save
```
cargo watch -q -c -x run 
```
    
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