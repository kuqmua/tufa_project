### todo tomorrow
* fix where was for github_file_line_column
* fix proc macro submodules, rename them
* fix github_file_line_column logic
* make refactoring for module-like implementation (no external dependencies from different modules)
* remove type_path.path.segments.len() check in proc_macros. use .get(index) instead of [index]
* format!("{}, ", e)) for error refactor in format tufa_server/src/preparation/check_availability.rs:91:29 error_string
* it possible to implement traits for Vec<u8> for example
* maybe write into implementation ofr errors to use ? syntax
* proc macreo input parameter crate or tufa_common
* think about config field traits
* once_cell for git info must be a macros

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

### init submodules 
```
git submodule init && git submodule update && 
cd scripts/git_sync_command && git checkout main && cd .. && cd .. &&
cd src/proc_macros/impl_box_err_source_from_err && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/enum_extension && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/error_display && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/gen_enum && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/gen_enum_without_values && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/generate_getter_traits_for_struct_fields && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/git_info && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_display && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_display_for_error_struct && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_display_for_simple_error_enum && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_error_with_tracing_for_struct_with_get_source_with_get_where_was && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_error_with_tracing_for_struct_with_get_source_without_get_where_was && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_error_with_tracing_for_struct_without_get_source && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_from_for_upper_struct && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_get_source_with_method && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_get_source_without_method && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_get_where_was_one_or_many_with_method && git checkout main && cd .. && cd .. && cd .. &&
cd src/proc_macros/impl_get_where_was_one_or_many_one_for_error_struct && git checkout main && cd .. && cd .. && cd .. &&
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
