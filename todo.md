# Rust Web App Development

## Resources
* [rust web app example](https://github.com/saschagrunert/webapp.rs)
* [60% faster tests run than cargo test](https://nexte.st/)
* [share cache between workspaces](https://github.com/mozilla/sccache)
* [.md file to html converter](https://dillinger.io/)
* [another rust web dev book](https://www.manning.com/books/rust-web-development)

## Implementation Plan
1. Make http request to https://dillinger.io/
2. Write markdown
3. Convert to html
4. Download styled html
5. Open html
6. Write content as String
7. Put some variables inside String
8. OR MAYBE USE HTML FROM CARGO DOC?
9. Maybe use nodemon cargo doc (maybe some params like exclude)

# Rust Language Features

## Iterators
* `iter()` for vecs yields `&i32`
* `into_iter()` for vecs yields `i32`
* Parallel iteration example with Rayon

## Constants
* Require constants inside trait and init them by default
* Const destructors
* Compile time functions as constants
* Const generic functions
* Trait bounds on generic parameters for constant function

## Memory Management
* Dynamic dispatching
* Mutex now const (on nightly)
* Wrapper around component or do not use function component
* How to create struct fields from different struct fields (mixin)

# Development Tools

## Cargo Tools
* cargo install cargo-show-asm - tool to view asm files in readable format
* cargo audit command usage
* try to use nexttest instead of cargo test

## Documentation
* In docs function/struct/macro description must be maximum words, phrases and symbols what describe function
* Otherwise user may not find it or know about what function doing
* And need to give users possibility to add additional words/phrases to describe it

# Testing

## Test Strategies
* Tracing through few microservices with error handlers
* Runtime tests with bot logging in some messenger
* Parsing providers posts tests
* Server what running web tests every hour like http requests and get valid data or not
* Add tests for new libraries versions and what version i use

## Test Implementation
* Write test what checks if something missing in .dockerignore and .gitignore
* Test to check what cargo run executes in the right folder
* Test what check commit name for only eng symbols
* #[should_panic(expected = "99")] - write some tests with with macro
* Right test what parse repo code and check what each function returns some type or enum
* Create big json file to test parsing speed as test
* Test for all env var to working and usage

# Error Handling

## Error Management
* try_ prefix if function can return Err or None
* Big enum of enums (recursive) of all errors in project
* If error size > then ok return type then allocate error on heap, otherwise actual return type would be more then needed
* Rename all functions return type to mean something
* Try SNAFU as error handling crate

## Error Implementation
* How to do error handling with Box<ErrorImpl>
* If your function is propagating the error upstream (e.g. using the ? operator), it should not log the error
* Add field how_to_fix error into error
* Async runtime error handling state for write/dont write/partially write into console error info

# Database Integration

## PostgreSQL
* Find out how to restrict user (or something) in postgres to not have foreign key creation possibility to the chosen by you table
* Queries to the chosen table too
* SeaORM instead of diesel?

## MongoDB
* Add write logs in db into dir if provider doesnt respond to request
* What to use? Postgres? Clickhouse? Mongo

# Data Processing

## Provider Management
* Write fetching result in file and check if there same posts or not
* Providers statistics in files. if 3 time provider is inactive then do not fetch him next time then next two times...
* Check every link's status code - maybe they rename it or something
* Only status without body

## Data Handling
* Do http reqwest again for some unsuccess http statuses in vec of unsuccess links
* Compute post's hash and send hashes from client app to server to check server already send them or not
* Rename some local variables in functions

# Concurrency

## Threading
* Futures in some cases instead of threads (like file open or write in file)
* Thread pool instead of for loop
* Thread pool with this let cpus = num_cpus::get()

## Async Programming
* let _ = join_all(vec_of_write_into_files_futures).await; //todo: add state of success/unsuccess
* If let Ok(something) = something.lock() {} instead of something.lock().expect();
* In cases across await points use tokio::sync::Mutex instead of std Mutex

# Configuration

## Environment Variables
* Write some logic and flag what choose between env and in code constants for more efficient production variant
* pub const PROJECT_MODE: &str = "Development";//later as ENV variable only
* Optimize for loop for std::env::var to std::env::vars
* Env vars names - make it private
* Make default value for all environment variables (I DONT THINK ITS A GOOD IDEA)

## Config Management
* Check config values on non negative, overflow, more than capacity, not zero
* Add method to config struct what check integer value for correct value
* Make fields private and use get/set. Check strings too

# Docker Integration

## Container Management
* Pull and run postgres docker container
* How to change default volume folder for this command?
* And mongo
* And in docker-compose too

## Docker Configuration
* Dockerfile with ARM support
* Docker build image inside ci pipeline
* Faster docker build with Cargo chef
* Tiny and Fast Docker image for Rust Application

# Code Quality

## Code Refactoring
* Early return refactoring with ok_or and ?
* Remove unwrap() into expect() to give more meaning
* Result instead of option in all code possible coz return Option loosing information about error
* Move providers link parts (specific urls) to project constants
* Reuse files with functions and constants using #[path=""] instead of module system

## Code Analysis
* Complete all exercises from https://github.com/rust-lang/rustlings
* Learn more about iterator methods
* Learn more about rust macro system
* AsRef trait find out more
* Find out more about downcast errors

# Performance Optimization

## Memory Optimization
* &str instead of String everywhere if possible
* 'static remove if possible or maybe add if its better
* Sometimes you need to write methods that accept a string slice (&str) and conditionally return either a modified version of it or the original one. For these cases, you might use Cow<str>, so that you only allocate new memory when necessary.

## Performance Tools
* Benchmarks
* Hazard pointer
* Read about async cancellation
* Show llvm ir command: rustc simple1.rs --emit=llvm-ir

# External Services

## API Integration
* Add stackoverflow provider
* https://stackoverflow.com/feeds/question/23412033

## Service Management
* Service for date/time checking and executing arxiv for example one time per week and check of server restarted in this timestamp
* Problem - now code waiting for all http reqwests to complete. rewrite it into event loop
* Or message queue

# Project Structure

## Workspaces
* Refactor project into rust workspaces to reuse dependencies
* Move traits into own repository to reuse them properly
* Use https://crates.io/crates/serde_yaml to control deps versions and features in different repos

## Module Organization
* Script/program for recursive submodule and repo git synchronization
* Different user_credentials files for project databases and providers
* Move all from user_credentials into .env file

# Version Control

## Git Integration
* Crate for git repos integration: git2 = "0.13"
* Pull requests instead of commits (+ github actions) - do not add changes if test fail

## Repository Management
* Add api route to return info about all api routes map/tree in current service
* This week in rust check
* Checking Unused Dependencies in a Rust Project with Github Actions

# UI Development

## Web Components
* WASM server components
* Wrapper around component or do not use function component
* Also there is a way to use html without html!{} Macro

## Frontend Integration
* Use this inside /api/info
* Maybe write proc macro?
* OR MAYBE USE HTML FROM CARGO DOC?

# Machine Learning

## Neural Networks
* [onnx-common NN format standart, runtime for neural net](https://docs.rs/onnxruntime/latest/onnxruntime/)
* [2 neural nets for text and images](https://openai.com/blog/clip/)
* https://github.com/tangramdotdev/tangram - train, deploy, and monitor machine learning models

# System Administration

## System Configuration
* Find out how to use nginx
* Systemctl is-enabled docker
* Read about debugging Rust application inside linux container
* On Linux you can induce a system reboot by writing a b to /proc/sysrq-trigger

## File Management
* Function to write save path from string and change some symbols
* If size of working dir > 100mb then remove all containing
* Tokio::fs::metadata - check path exists in file system

# Security

## Access Control
* Find out how to restrict user (or something) in postgres to not have foreign key creation possibility to the chosen by you table
* Queries to the chosen table too

## Data Protection
* Rust exposes this system call via the File::sync_all method. If we call sync_all when it returns successfully, the operating system promises that the data has been written to disk and will exist on disk even in the event of a hard crash like a power failure or OS crash.

# Internationalization

## Language Detection
* Rust language detection library: https://github.com/pemistahl/lingua-rs

# Code Generation

## Procedural Macros
* Cut enum extension into few different proc macroses
* Write match should trace inside init or new function of error initialization
* https://blog.turbo.fish/proc-macro-error-handling/
* Some logic around location() file!() line!() column!() - maybe generate all other functions - github link and others on compile time instead of runtime?

## Code Generation Tools
* Gen rust files: https://matklad.github.io/2022/03/26/self-modifying-code.html