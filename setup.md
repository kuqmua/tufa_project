# Table of contents
1. [git_sync_command](#heading1)
2. [compile_time_git_info](#heading2)
3. [enum_extension](#heading3)
4. [error_display](#heading4)
5. [gen_enum](#heading5)
6. [gen_enum_without_values](#heading6)
7. [generate_getter_traits_for_struct_fields](#heading7)
8. [impl_display_for_error](#heading8)
9. [impl_error_with_tracing](#heading9)
10. [impl_from_for_upper_struct](#heading10)
11. [impl_get_code_occurence](#heading11)
12. [impl_get_git_info](#heading12)
13. [impl_get_source](#heading13)
14. [impl_get_where_was_origin_or_wrapper](#heading14)
15. [init_error](#heading15)
16. [init_from_env](#heading16)
17. [init_from_env_with_panic_if_failed](#heading17)
18. [proc_macro_helpers](#heading18)
19. [provider_kind_from_config](#heading19)
20. [struct_field_getter](#heading20)
21. [struct_field_setter](#heading21)
22. [svg_component](#heading22)
23. [tufa_client](#heading23)
24. [tufa_common](#heading24) 
25. [tufa_grpc_client](#heading25)
26. [tufa_grpc_server](#heading26)
27. [tufa_server](#heading27) 
28. [tufa_telegram_bot](#heading28)

<!-- its better to add elements only to the end -->

### git_sync_command <a name="heading1"/>

### compile_time_git_info <a name="heading2"/>

### enum_extension <a name="heading3"/>

### error_display <a name="heading4"/>

### gen_enum <a name="heading5"/>

### gen_enum_without_values <a name="heading6"/>

### generate_getter_traits_for_struct_fields <a name="heading7"/>

### impl_display_for_error <a name="heading8"/>

### impl_error_with_tracing <a name="heading9"/>

### impl_from_for_upper_struct <a name="heading10"/>

### impl_get_code_occurence <a name="heading11"/>

### impl_get_git_info <a name="heading12"/>

### impl_get_source <a name="heading13"/>

### impl_get_where_was_origin_or_wrapper <a name="heading14"/>

### init_error <a name="heading15"/>

### init_from_env <a name="heading16"/>

### init_from_env_with_panic_if_failed <a name="heading17"/>

### proc_macro_helpers <a name="heading18"/>

### provider_kind_from_config <a name="heading19"/>

### struct_field_getter <a name="heading20"/>

### struct_field_setter <a name="heading21"/>

### svg_component <a name="heading22"/>

### tufa_client <a name="heading23"/> 
```
cargo install --locked trunk
```
```
rustup target add wasm32-unknown-unknown
```
### usage
```
trunk serve
```
### fonts source
[link](https://fonts.google.com/)
### yew highlighting
https://github.com/Alexandre-Borghi/yew-highlighting

### tufa_common <a name="heading24"/> 

### tufa_grpc_client <a name="heading25"/> 
#### install dependencies
```
sudo apt install cmake
```

### tufa_grpc_server <a name="heading26"/> 

### tufa_server <a name="heading27"/> 
#### install dependencies
```
sudo apt install cmake
```

### tufa_telegram_bot <a name="heading28"/>
#### run
```
RUST_LOG=info TELOXIDE_TOKEN="" cargo run
```