# Table of contents
1. [simple route request](#heading1)
2. [get_providers_posts route request](#heading2)
3. [change project config](#heading3)
4. [change tests constants](#heading4)
5. [start docker daemon](#heading5)
6. [build docker container (maybe some steps can be ignored)](#heading6)
7. [run docker container](#heading7)
8. [stop docker container](#heading8)
9. [remove docker container](#heading9)
10. [remove all unused right now docker containers and images](#heading10)
11. [run containers with docker-compose](#heading11)
12. [stop containers with docker-compose](#heading12)
13. [default docker volumes folder on linux](#heading13)
14. [pull and run mongodb docker container](#heading14)
15. [start mongodb docker container](#heading15)
16. [create new rust library](#heading16)
17. [pull and run postgres docker container](#heading17)
18. [start postres docker container](#heading18)
19. [shutdown wsl(if db clients cannot connect to db in wsl2)](#heading19)
20. [give priviligies to volumes folder](#heading20)
21. [start command](#heading21)
22. [run ci tests](#heading22)
23. [run local tests](#heading23)
24. [show tree visualization of a dependency graph](#heading24)
25. [how to tune rustfmt](#heading25)
26. [check vulnerabilities in project](#heading26)
27. [fix Error: I/O error: Permission denied (os error 13) error](#heading27)
28. [cargo watch](#heading28)
29. [install custom linker dependencies](#heading29)
30. [start deleopment](#heading30)
31. [pull redis image](#heading31)
32. [launch Postgres](#heading32)
33. [install sqlx-cli](#heading33)
34. [example add sqlx migration](#heading34) 
35. [how to use logger](#heading35)
36. [subscribe route test (change email and name)](#heading36))
37. [how to install remove unused dependencies tool](#heading37)
38. [how to install logs formatter?](#heading38)
39. [docker build](#heading39)
40. [Generate query metadata to support offline compile-time verification](#heading40)
41. [run docker container](#heading41)
42. [smaller rust docker builds](#heading42)
43. [ignore Digital Ocean for now](#heading43)
44. [ignore How to get started with postmark](#heading44)
45. [property-based testing](#heading45)
46. [if tests will be more than 1024](#heading46)
47. [Error: I/O error: Permission denied (os error 13) fix](#heading47)
48. [The script needs to be marked as executable and then launched](#heading48)
49. [see logs with cargo test](#heading49)
50. [run integration tests](#heading50)
51. [run unit tests](#heading51)
52. [run continious integration tests](#heading52)
53. [links](#heading53)

## simple route request <a name="heading1"/>

### git_sync_command

### compile_time_git_info

### enum_extension

### error_display

### gen_enum

### gen_enum_without_values

### generate_getter_traits_for_struct_fields

### impl_display_for_error

### impl_error_with_tracing

### impl_from_for_upper_struct

### impl_get_code_occurence

### impl_get_git_info

### impl_get_source

### impl_get_where_was_origin_or_wrapper

### init_error

### init_from_env

### init_from_env_with_panic_if_failed

### proc_macro_helpers

### provider_kind_from_config

### struct_field_getter

### struct_field_setter

### svg_component

### tufa_client 

### tufa_common 

### tufa_grpc_client 

### tufa_grpc_server 

### tufa_server 

### tufa_telegram_bot
#### run
```
RUST_LOG=info TELOXIDE_TOKEN="" cargo run
```