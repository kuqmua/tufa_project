### todo tomorrow
* fix where was for github_file_line_column
* fix proc macro submodules, rename them
* fix github_file_line_column logic

### init submodules 
```
git submodule init && git submodule update && 
cd proc_macros/impl_box_err_source_from_err && git checkout main && cd .. && cd .. &&
cd proc_macros/enum_extension && git checkout main && cd .. && cd .. &&
cd proc_macros/error_display && git checkout main && cd .. && cd .. &&
cd proc_macros/gen_enum && git checkout main && cd .. && cd .. &&
cd proc_macros/gen_enum_without_values && git checkout main && cd .. && cd .. &&
cd proc_macros/git_info && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_display && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_from_for_upper_struct && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_get_source_for_original_error_struct && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_get_source_for_parent_error_struct && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_get_source_for_simple_error_enum && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_get_source_for_source_error_enum && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_get_where_was_one_or_many_for_enum && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_get_where_was_one_or_many_one_for_error_struct && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_display_for_error_struct && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_display_for_simple_error_enum && git checkout main && cd .. && cd .. &&
cd proc_macros/impl_error_with_tracing_for_original_error_struct_without_source_enum && git checkout main && cd .. && cd .. &&
cd proc_macros/init_error && git checkout main && cd .. && cd .. &&
cd proc_macros/init_error_with_tracing && git checkout main && cd .. && cd .. &&
cd proc_macros/init_error_with_tracing_for_original_error_struct && git checkout main && cd .. && cd .. &&
cd proc_macros/init_from_env && git checkout main && cd .. && cd .. &&
cd proc_macros/provider_kind_from_config && git checkout main && cd .. && cd ..
cd proc_macros/struct_field_getter && git checkout main && cd .. && cd ..
cd proc_macros/struct_field_setter && git checkout main && cd .. && cd ..
cd proc_macros/svg_component && git checkout main && cd .. && cd ..
cd tufa_client && git checkout main && cd .. && 
cd tufa_common && git checkout main && cd .. && 
cd tufa_server && git checkout main && cd .. && 
cd tufa_telegram_bot && git checkout main && cd .. && 
cd tufa_grpc_client && git checkout main && cd .. && 
cd tufa_grpc_server && git checkout main && cd .. && 

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
