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

* handle all todos in parse_github_html

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

* #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
files to write warnings:
rss_parse_string_into_struct
rss_fetch_and_parse_provider_data
rss_divide_to_equal_for_each_provider
rss_check_available_providers
parse_github_html and inner
get_providers_link_parts_from_mongo
generate_twitter_hashmap_links

* github parsing return in second parameter option<String, line! file!> to analize missing parse logic 

* implement get_providers_link_parts with all success completed and with not all

* get_providers_link_parts whole and for each provider
and maybe rewrite it as struct with methods

* Resource::PostgreSql => { 

* rename this check_new_posts_threads_parts

* file: src/fetch/parse_github_html.rs:1226
different children.len(): 3
file: src/fetch/parse_github_html.rs:1553
different children.len(): 3

* 5 => {
                println!("todo 5 elements github parsing")
            }
github parsing

* add provider_kind into inner related to providers functions as input parameter

* pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
    TimeMeasurement,
    CleaningWarningLogsDirectory,
}
add Info
add ThreadError
and other specific errors

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

* remove unhandled_success_handled_success_are_there_items_initialized_posts_dir - like fileds from config
rename it 

* if error size > then ok return type then allocate error on heap, otherwise actual return type would be more then needed
maybe write some tests or linter or lib or rustanalyzer flag to check it

* Rename all functions return type to mean something.
type = ....

* wrap blocking diesel function into spawn blocking under tokio runtime
(rustacean station - tokio ecosystem with alice ryhl) 24:30

* Common projects constants as lazy static to reuse
Them inside parent modules
  
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

* how to write proc macro argument?
for derive_impl_box_err_source_from_err
EXISTS INSIDE AST!!!

* LINKS_LIMIT_PROVIDERS not working now. fix it

* parameter db initialization provider link parts source must be
different than PROVIDERS_LINK_PARTS_SOURCE 

* divide and move outside postgres and mongo methods from provider_kind_trait 

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
  
* auto check int and float overflow
first, install cargo clippy
#![deny(clippy::integer_arithmetic, clippy::float_arithmetic)]
use checked_sub() and checked_add() methods

* in cases across await points use tokio::sync::Mutex instead of std Mutex
use this lint 
https://rust-lang.github.io/rust-clippy/master/#await_holding_lock
  
* links limits and other getters functions like before on config new initialization (before proc macro)

* add connect_timeout to all mongo client_options usage
  
* add time to errors like this ./server/utils/dependencies.ts:524:17  15:30:40 25.01.22 
(partially done)

* add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
  
* https://blog.turbo.fish/proc-macro-error-handling/
  
* web-server library-agnostic modules architecture. must add actix, rocket, warp, axum

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
* remove lazy_static! from git file open logic(lazy_static! works on runtime, but logic work on compile time)
* write match should trace inside init or new function of error initialization
* env var json tracing on/off
* different formatting for env var json tracing on/off
* get_where_was must return WhereWas, not a string. For a must must be a different method