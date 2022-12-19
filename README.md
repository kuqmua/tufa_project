### todo tomorrow
* fix proc macro submodules, rename them
* make refactoring for module-like implementation (no external dependencies from different modules)
* remove type_path.path.segments.len() check in proc_macros. use .get(index) instead of [index]
* format!("{}, ", e)) for error refactor in format tufa_server/src/preparation/check_availability.rs:91:29 error_string
* proc macreo input parameter crate or tufa_common
* add use trait import in the scope in case of macro like this 
```
let f: u32 = {
    use tufa_common::traits::something::Something;
    0
};
```
* some logic around location() file!() line!() column!() - maybe generate all other functions -github link and others on compiletime instead of runtime?
* for all function with git_info input parameters - use get_git_info instead 
* clippy settings token stream  - cannot do what. if u use this macro - will be an error "error: an inner attribute is not permitted in this context"
* remove CONFIG usage from tufa_common
* write logic around concurrent_or_parallel_execution_index
* hashmap<git_info, file_line_column> - useless for serialization and compression
* or maybe put git_info vec only in struct for serialization ?
* must use vec of parallel execution vectors vec;
* rust can compile resursive function and get stackoverflow
```
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

### check warning/errors(not the same)
1. cargo check
2. cargo clippy
3. rust-analyzer from vscode extension

#### Location instead of WhereWas
```
use core::panic::Location;

#[track_caller]
fn location() -> Location<'static> {
    *Location::caller()
}

fn main() {
    println!("{:?}", location());
    println!("{:?}", location());
}
```

### alternative init submodules command
```
git submodule update --init --recursive --checkout
```

### another alternative init submodules command
```
cd scripts && git submodule init git_sync_command && git submodule update git_sync_command && cd git_sync_command/ && cargo run && cd .. && cd ..
```
### thoughts about error handling
variants for vec and hashmap, wrapper and origin must be differenet
then hashmap occurse - get_source somehow must different, without printing time
get_source must return parallel prints for hashmap and vec?
but what need to do if inside hashmap - another hashmap? add some space increment?
and if its a vec or hashmap -  code occurence must not propagate to parents

time [key] error1
 tufa_common/src/server/mongo/something1.rs:70:37 
 tufa_server/src/preparation/something1.rs:92:29
time [key] error2
 tufa_common/src/server/mongo/something2.rs:70:37 
 tufa_server/src/preparation/something2.rs:92:29
tufa_server/src/preparation/parent.rs:92:29
tufa_server/src/preparation/parent.rs:45:29

### init submodules 
```
git submodule init && git submodule update && 
cd scripts/git_sync_command && git checkout main && cd .. && cd .. &&
cd src/proc_macros/enum_extension && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/error_display && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/gen_enum && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/gen_enum_without_values && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/generate_getter_traits_for_struct_fields && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_display_for_error && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_error_with_tracing && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_from_for_upper_struct && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_get_git_info && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_get_source && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_get_where_was_origin_or_wrapper && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/init_error && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/init_from_env && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/init_from_env_with_panic_if_failed && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/provider_kind_from_config && git checkout main && cd .. && cd .. cd .. &&
cd src/proc_macros/struct_field_getter && git checkout main && cd .. && cd .. cd .. &&
cd src/proc_macros/struct_field_setter && git checkout main && cd .. && cd .. cd .. &&
cd src/proc_macros/svg_component && git checkout main && cd .. && cd .. cd .. &&
cd src/tufa_client && git checkout main && cd .. &&  cd .. &&
cd src/tufa_common && git checkout main && cd .. &&  cd .. &&
cd src/tufa_server && git checkout main && cd .. &&  cd .. &&
cd src/tufa_telegram_bot && git checkout main && cd .. &&  cd .. &&
cd src/tufa_grpc_client && git checkout main && cd .. &&  cd .. &&
cd src/tufa_grpc_server && git checkout main && cd ..  cd ..


```
### install cmake for grpc
```
sudo apt install cmake
```
### build submodules dependency cache
```
cargo build
```
### example git submodule
```
git submodule add https://github.com/kuqmua/tufa_client.git
```
```
git submodule add https://github.com/kuqmua/tufa_server.git
```
### up databases
```
cd tufa_server && sudo docker-compose up -d && cd ..
```
### run postgres migrations
```  
cd tufa_server && sqlx migrate run && cd ..
```
### repo tracker example
```
https://github-stats.com/kuqmua/tufa_project
```
### cargo expand issue (works here only with --lib flag)
```
cargo expand your::path::to::module --lib
```
### [all algorithms written in rust](https://github.com/TheAlgorithms/Rust)

### [use AI in with rust](https://youtu.be/StMP7g-0wK4)
