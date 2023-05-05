# Table of content
1. [git_sync_command](#heading1)
2. [compile_time_git_info](#heading2)
3. [enum_extension](#heading3)
4. [error_display](#heading4)
5. [gen_enum](#heading5)
6. [gen_enum_without_values](#heading6)
7. [generate_getter_traits_for_struct_fields](#heading7)
8. [impl_display_for_error](#heading8)
12. [impl_get_git_info](#heading12)
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
29. [error_occurence](#heading29)

<!-- its better to add elements only to the end -->

## git_sync_command <a name="heading1"/>

## compile_time_git_info <a name="heading2"/>

## enum_extension <a name="heading3"/>

## error_display <a name="heading4"/>

## gen_enum <a name="heading5"/>

## gen_enum_without_values <a name="heading6"/>

## generate_getter_traits_for_struct_fields <a name="heading7"/>

## impl_display_for_error <a name="heading8"/>

## impl_get_git_info <a name="heading12"/>

## init_from_env_with_panic_if_failed <a name="heading17"/>

## proc_macro_helpers <a name="heading18"/>

## provider_kind_from_config <a name="heading19"/>

## struct_field_getter <a name="heading20"/>

## struct_field_setter <a name="heading21"/>

## svg_component <a name="heading22"/>

## tufa_client <a name="heading23"/>

## error_occurence <a name="heading29"/> 

```
cargo install --locked trunk
```
```
rustup target add wasm32-unknown-unknown
```
#### usage
```
trunk serve
```
#### fonts source
[link](https://fonts.google.com/)
#### yew highlighting
https://github.com/Alexandre-Borghi/yew-highlighting

## tufa_common <a name="heading24"/> 

## tufa_grpc_client <a name="heading25"/> 
#### install dependencies
```
sudo apt install cmake
```

## tufa_grpc_server <a name="heading26"/> 
#### install dependencies
```
sudo apt install cmake
```

## tufa_server <a name="heading27"/> 
#### tufa_server setup content
1. [simple route request](#heading271)
2. [get_providers_posts route request](#heading272)
3. [change project config](#heading273)
4. [change tests constants](#heading274)
5. [start docker daemon](#heading275)
6. [build docker container (maybe some steps can be ignored)](#heading276)
7. [run docker container](#heading277)
8. [stop docker container](#heading278)
9. [remove docker container](#heading279)
10. [remove all unused right now docker containers and images](#heading2710)
11. [run containers with docker-compose](#heading2711)
12. [stop containers with docker-compose](#heading2712)
13. [default docker volumes folder on linux](#heading2713)
14. [pull and run mongodb docker container](#heading2714)
15. [start mongodb docker container](#heading2715)
16. [create new rust library](#heading2716)
17. [pull and run postgres docker container](#heading2717)
18. [start postres docker container](#heading2718)
19. [shutdown wsl(if db clients cannot connect to db in wsl2)](#heading2719)
20. [give priviligies to volumes folder](#heading2720)
21. [start command](#heading2721)
22. [run ci tests](#heading2722)
23. [run local tests](#heading2723)
24. [show tree visualization of a dependency graph](#heading2724)
25. [how to tune rustfmt](#heading2725)
26. [check vulnerabilities in project](#heading2726)
27. [fix Error: I/O error: Permission denied (os error 13) error](#heading2727)
28. [cargo watch](#heading2728)
29. [install custom linker dependencies](#heading2729)
30. [start deleopment](#heading2730)
31. [pull redis image](#heading2731)
32. [launch Postgres](#heading2732)
33. [install sqlx-cli](#heading2733)
34. [example add sqlx migration](#heading2734) 
35. [how to use logger](#heading2735)
36. [subscribe route test (change email and name)](#heading2736))
37. [how to install remove unused dependencies tool](#heading2737)
38. [how to install logs formatter?](#heading2738)
39. [docker build](#heading2739)
40. [Generate query metadata to support offline compile-time verification](#heading2740)
41. [run docker container](#heading2741)
42. [smaller rust docker builds](#heading2742)
43. [ignore Digital Ocean for now](#heading2743)
44. [ignore How to get started with postmark](#heading2744)
45. [property-based testing](#heading2745)
46. [if tests will be more than 1024](#heading2746)
47. [Error: I/O error: Permission denied (os error 13) fix](#heading2747)
48. [The script needs to be marked as executable and then launched](#heading2748)
49. [see logs with cargo test](#heading2749)
50. [run integration tests](#heading2750)
51. [run unit tests](#heading2751)
52. [run continious integration tests](#heading2752)
53. [links](#heading2753)

#### simple route request <a name="heading271"></a>
```
curl http://127.0.0.1:8080/kekw/index.html
```
#### get_providers_posts route request <a name="heading272"></a>
```
curl http://127.0.0.1:8080/get_providers_posts/
```
#### change project config <a name="heading273"></a>
./env <br/>

#### change tests constants <a name="heading274"></a>
.libs/config_lib/src/get_project_information/project_constans.rs <br/>

#### start docker daemon <a name="heading275"></a>
```
sudo dockerd
```
#### build docker container (maybe some steps can be ignored) <a name="heading276"></a>
```
rustup install nightly
```
```
rustup target add x86_64-unknown-linux-musl
```
```
cargo +nightly build --release
```
```
sudo docker build -t tufa_server-image .
```
#### run docker container <a name="heading277"></a>
```
docker run --env-file .env --name tufa_server-container -p 8000:8000 --rm -it tufa_server-image
```
#### stop docker container <a name="heading278"></a>
```
sudo docker stop tufa_server-container
```
#### remove docker container <a name="heading279"></a>
```
sudo docker rm tufa_server-container
```
#### remove all unused right now docker containers and images <a name="heading2710"></a>
```
sudo docker system prune -f 
```
#### run containers with docker-compose <a name="heading2711"></a>
```
sudo docker-compose up -d
```
#### stop containers with docker-compose <a name="heading2712"></a>
```
sudo docker-compose down
```
#### default docker volumes folder on linux <a name="heading2713"></a>
/var/lib/docker/volumes

#### pull and run mongodb docker container <a name="heading2714"></a>
(need to write path to your project directory)
```
sudo docker run -p 27017:27017 --name mongo-tufa-wsl2 -v ~/projects/tufa_server/mongodb_volume:/data/db -d mongo:latest
```
#### start mongodb docker container <a name="heading2715"></a>
with docker: <br/>
```
sudo docker start mongo-tufa-wsl2
```
with docker-compose(other services too): <br/>
```
sudo docker-compose -f docker-compose.yml up -d 
```
#### create new rust library <a name="heading2716"></a>
```
cargo new example_lib --lib
```
#### pull and run postgres docker container <a name="heading2717"></a>
```
sudo docker run -p 5432:5432/tcp --name postgres-tufa-wsl2 -v ~/db-volumes/postgresql-volumes/tufa-dev-volume -e POSTGRES_PASSWORD=postgres -d postgres:latest
```
#### start postres docker container <a name="heading2718"></a>
with docker: <br/>
```
sudo docker start postgres-tufa-wsl2
```
with docker-compose(other services too): <br/>
```
sudo docker-compose -f docker-compose.yml up -d
```
#### shutdown wsl(if db clients cannot connect to db in wsl2) <a name="heading2719"></a>
windows console: <br/>
```
wsl --shutdown
```
then reopen wsl

#### give priviligies to volumes folder <a name="heading2720"></a>
```
sudo chown -R username /folderexample 
```
(/db-volumes/mongodb or postgresql) <br/>

#### start command <a name="heading2721"></a>
```
cd libs/tests_lib && cargo test local && cd .. && cd .. && cargo run
```
#### run ci tests <a name="heading2722"></a>
```
cd libs/tests_lib && cargo test ci -- --show-output
```
#### run local tests <a name="heading2723"></a>
```
cd libs/tests_lib && cargo test local -- --show-output
```
#### show tree visualization of a dependency graph <a name="heading2724"></a>
```
cargo tree
```
#### how to tune rustfmt <a name="heading2725"></a>
You can tune rustfmt for a project with a configuration file, rustfmt.toml. </br> 
Details can be found in [rustfmt’s](https://github.com/rust-lang/rustfmt#configuring-rustfmt)

#### check vulnerabilities in project <a name="heading2726"></a>
cargo-audit, a convenient cargo sub-command to check if vulnerabilities have <br/>
been reported for any of the crates in the dependency tree of your project. <br/>
installation: <br/>
```
cargo install cargo-audit
```
usage: <br/>
```
cargo audit
```
#### fix Error: I/O error: Permission denied (os error 13) error <a name="heading2727"></a>
```
cd .. sudo chmod -R 777 tufa_server && cd tufa_server
```
#### cargo watch <a name="heading2728"></a>
```
cargo watch -x check -x test -x "run | bunyan"
```
#### install custom linker dependencies <a name="heading2729"></a>
[page](https://www.lpalmieri.com/posts/session-based-authentication-in-rust/) </br>
On Windows: <br/>
```
cargo install -f cargo-binutils
```
```
rustup component add llvm-tools-preview

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```
On Linux: <br/>
Ubuntu  <br/>
```
sudo apt-get install lld clang
```
Arch <br/>
```
sudo pacman -S lld clang
```
```
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
```
On MacOS <br/>
```
brew install michaeleisel/zld/zld
```
```
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```
#### start deleopment <a name="heading2730"></a>
```
cargo watch -x check -x test -x "run"
```
#### pull redis image <a name="heading2731"></a>
```
sudo docker pull redis
```
#### launch Postgres <a name="heading2732"></a>
```
sudo ./scripts/init_db.sh
```
(sudo coz got pesmission denied error)

#### install sqlx-cli <a name="heading2733"></a>
```
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
```
#### example add sqlx migration <a name="heading2734"></a>
```
sqlx migrate add create_subscriptions_table
```
#### how to use logger <a name="heading2735"></a>
log provides five macros: trace, debug, info, warn and error.
RUST_LOG=debug cargo run, for example, will surface all logs at debug-level or higher emitted by our application or the crates we are using. RUST_LOG=session_based_authentication, instead, would filter out all records emitted by our dependencies.

#### subscribe route test (change email and name) <a name="heading2736"></a>
```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom'  http://127.0.0.1:8000/subscriptions
```
#### how to install remove unused dependencies tool <a name="heading2737"></a>
```
cargo install cargo-udeps
```
usage: <br/>
```
cargo +nightly udeps
```
#### how to install logs formatter? <a name="heading2738"></a>
```
cargo install bunyan
```
#### docker build <a name="heading2739"></a> 
```
sudo docker build --tag session_based_authentication --file Dockerfile .
```
#### Generate query metadata to support offline compile-time verification. <a name="heading2740"></a>
```
sqlx prepare
```
or  <br/>
```
cargo sqlx prepare -- --lib
```
(to use generated .json query data - env var SQLX_OFFLINE must be true)

#### run docker container <a name="heading2741"></a>
```
sudo docker run -p 8000:8000 session_based_authentication
```
#### smaller rust docker builds <a name="heading2742"></a>
We could go even smaller by using rust:1.59.0-alpine, but we would have to cross-compile to the linux-musl target - out of scope for now. Check out rust-musl-builder if you are interested in generating tiny Docker images.
Another option to reduce the size of our binary further is stripping symbols from it - you can find more information about it here.

#### ignore Digital Ocean for now <a name="heading2743"></a>

#### ignore How to get started with postmark <a name="heading2744"></a>

#### property-based testing <a name="heading2745"></a>
There are two mainstream options for property-based testing in the Rust ecosystem: quickcheck and proptest.

#### if tests will be more than 1024 <a name="heading2746"></a>
If you have large test suite with a flat file structure, you'll soon be building tens of executable every time you run cargo test. While each executable is compiled in parallel, the linking phase is instead entirely sequential! Bundling all your test cases in a single executable reduces the time spent compiling your test suite in CI3.
If you are running Linux, you might see errors like

thread 'actix-rt:worker' panicked at 
'Can not create Runtime: Os { code: 24, kind: Other, message: "Too many open files" }',

when you run cargo test after the refactoring.
This is due to a limit enforced by the operating system on the maximum number of open file descriptors (including sockets) for each process - given that we are now running all tests as part of a single binary, we might be exceeding it. The limit is usually set to 1024, but you can raise it with ulimit -n X (e.g. ulimit -n 10000) to resolve the issue.

#### Error: I/O error: Permission denied (os error 13) fix <a name="heading2747"></a>
```
sudo chown -R $(whoami) session_based_authentication/
```
#### The script needs to be marked as executable and then launched: <a name="heading2748"></a>
```
chmod +x ./scripts/init_redis.sh ./script/init_redis.sh
```
#### see logs with cargo test <a name="heading2749"></a>
```
cargo test -- --nocapture
```
#### run integration tests <a name="heading2750"></a>
```
cargo test integration
```
(integration tests will fail if they run with unit tests)

#### run unit tests <a name="heading2751"></a>
```
cargo test unit
```
#### run continious integration tests <a name="heading2752"></a>
```
cargo test ci
```
#### links <a name="heading2753"></a>
* [How to connect Robo 3T (Robomongo) to MongoDB Atlas (cloud mongoDB database)](https://www.youtube.com/watch?v=t_X7qFMmWhI) </br>
* [pgadmin create table](https://www.youtube.com/watch?v=h5wgbJiSy7Q) </br>

#### run server with modified like js tracing
```
cargo run -q | bunyan
```

## tufa_telegram_bot <a name="heading28"/>
#### run
```
RUST_LOG=info TELOXIDE_TOKEN="" cargo run
```