### this repo was written as [tufa_project](https://github.com/kuqmua/tufa_project) dependency
If you clone this repo without [tufa_project](https://github.com/kuqmua/tufa_project) </br>
repo may not work as expected </br>
(coz some deps from tufa_project)

Status of last deployment: <br/>
<img src="https://github.com/kuqmua/tufa_server/workflows/CI/badge.svg?branch=master"><br/>

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

## simple route request <a name="heading1"></a>
```
curl http://127.0.0.1:8080/kekw/index.html
```
### get_providers_posts route request <a name="heading2"></a>
```
curl http://127.0.0.1:8080/get_providers_posts/
```
### change project config <a name="heading3"></a>
./env <br/>

### change tests constants <a name="heading4"></a>
.libs/config_lib/src/get_project_information/project_constans.rs <br/>

### start docker daemon <a name="heading5"></a>
```
sudo dockerd
```
### build docker container (maybe some steps can be ignored) <a name="heading6"></a>
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
### run docker container <a name="heading7"></a>
```
docker run --env-file .env --name tufa_server-container -p 8000:8000 --rm -it tufa_server-image
```
### stop docker container <a name="heading8"></a>
```
sudo docker stop tufa_server-container
```
### remove docker container <a name="heading9"></a>
```
sudo docker rm tufa_server-container
```
### remove all unused right now docker containers and images <a name="heading10"></a>
```
sudo docker system prune -f 
```
### run containers with docker-compose <a name="heading11"></a>
```
sudo docker-compose up -d
```
### stop containers with docker-compose <a name="heading12"></a>
```
sudo docker-compose down
```
### default docker volumes folder on linux <a name="heading13"></a>
/var/lib/docker/volumes

### pull and run mongodb docker container <a name="heading14"></a>
(need to write path to your project directory)
```
sudo docker run -p 27017:27017 --name mongo-tufa-wsl2 -v ~/projects/tufa_server/mongodb_volume:/data/db -d mongo:latest
```
### start mongodb docker container <a name="heading15"></a>
with docker: <br/>
```
sudo docker start mongo-tufa-wsl2
```
with docker-compose(other services too): <br/>
```
sudo docker-compose -f docker-compose.yml up -d 
```
### create new rust library <a name="heading16"></a>
```
cargo new example_lib --lib
```
### pull and run postgres docker container <a name="heading17"></a>
```
sudo docker run -p 5432:5432/tcp --name postgres-tufa-wsl2 -v ~/db-volumes/postgresql-volumes/tufa-dev-volume -e POSTGRES_PASSWORD=postgres -d postgres:latest
```
### start postres docker container <a name="heading18"></a>
with docker: <br/>
```
sudo docker start postgres-tufa-wsl2
```
with docker-compose(other services too): <br/>
```
sudo docker-compose -f docker-compose.yml up -d
```
### shutdown wsl(if db clients cannot connect to db in wsl2) <a name="heading19"></a>
windows console: <br/>
```
wsl --shutdown
```
then reopen wsl

### give priviligies to volumes folder <a name="heading20"></a>
```
sudo chown -R username /folderexample 
```
(/db-volumes/mongodb or postgresql) <br/>

### start command <a name="heading21"></a>
```
cd libs/tests_lib && cargo test local && cd .. && cd .. && cargo run
```
### run ci tests <a name="heading22"></a>
```
cd libs/tests_lib && cargo test ci -- --show-output
```
### run local tests <a name="heading23"></a>
```
cd libs/tests_lib && cargo test local -- --show-output
```
### show tree visualization of a dependency graph <a name="heading24"></a>
```
cargo tree
```
### how to tune rustfmt <a name="heading25"></a>
You can tune rustfmt for a project with a configuration file, rustfmt.toml. </br> 
Details can be found in [rustfmt’s](https://github.com/rust-lang/rustfmt#configuring-rustfmt)

### check vulnerabilities in project <a name="heading26"></a>
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
### fix Error: I/O error: Permission denied (os error 13) error <a name="heading27"></a>
```
cd .. sudo chmod -R 777 tufa_server && cd tufa_server
```
### cargo watch <a name="heading28"></a>
```
cargo watch -x check -x test -x "run | bunyan"
```
### install custom linker dependencies <a name="heading29"></a>
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
### start deleopment <a name="heading30"></a>
```
cargo watch -x check -x test -x "run"
```
### pull redis image <a name="heading31"></a>
```
sudo docker pull redis
```
### launch Postgres <a name="heading32"></a>
```
sudo ./scripts/init_db.sh
```
(sudo coz got pesmission denied error)

### install sqlx-cli <a name="heading33"></a>
```
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
```
### example add sqlx migration <a name="heading34"></a>
```
sqlx migrate add create_subscriptions_table
```
### how to use logger <a name="heading35"></a>
log provides five macros: trace, debug, info, warn and error.
RUST_LOG=debug cargo run, for example, will surface all logs at debug-level or higher emitted by our application or the crates we are using. RUST_LOG=session_based_authentication, instead, would filter out all records emitted by our dependencies.

### subscribe route test (change email and name) <a name="heading36"></a>
```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom'  http://127.0.0.1:8000/subscriptions
```
### how to install remove unused dependencies tool <a name="heading37"></a>
```
cargo install cargo-udeps
```
usage: <br/>
```
cargo +nightly udeps
```
### how to install logs formatter? <a name="heading38"></a>
```
cargo install bunyan
```
### docker build <a name="heading39"></a> 
```
sudo docker build --tag session_based_authentication --file Dockerfile .
```
### Generate query metadata to support offline compile-time verification. <a name="heading40"></a>
```
sqlx prepare
```
or  <br/>
```
cargo sqlx prepare -- --lib
```
(to use generated .json query data - env var SQLX_OFFLINE must be true)

### run docker container <a name="heading41"></a>
```
sudo docker run -p 8000:8000 session_based_authentication
```
### smaller rust docker builds <a name="heading42"></a>
We could go even smaller by using rust:1.59.0-alpine, but we would have to cross-compile to the linux-musl target - out of scope for now. Check out rust-musl-builder if you are interested in generating tiny Docker images.
Another option to reduce the size of our binary further is stripping symbols from it - you can find more information about it here.

### ignore Digital Ocean for now <a name="heading43"></a>

### ignore How to get started with postmark <a name="heading44"></a>

### property-based testing <a name="heading45"></a>
There are two mainstream options for property-based testing in the Rust ecosystem: quickcheck and proptest.

### if tests will be more than 1024 <a name="heading46"></a>
If you have large test suite with a flat file structure, you'll soon be building tens of executable every time you run cargo test. While each executable is compiled in parallel, the linking phase is instead entirely sequential! Bundling all your test cases in a single executable reduces the time spent compiling your test suite in CI3.
If you are running Linux, you might see errors like

thread 'actix-rt:worker' panicked at 
'Can not create Runtime: Os { code: 24, kind: Other, message: "Too many open files" }',

when you run cargo test after the refactoring.
This is due to a limit enforced by the operating system on the maximum number of open file descriptors (including sockets) for each process - given that we are now running all tests as part of a single binary, we might be exceeding it. The limit is usually set to 1024, but you can raise it with ulimit -n X (e.g. ulimit -n 10000) to resolve the issue.

### Error: I/O error: Permission denied (os error 13) fix <a name="heading47"></a>
```
sudo chown -R $(whoami) session_based_authentication/
```
### The script needs to be marked as executable and then launched: <a name="heading48"></a>
```
chmod +x ./scripts/init_redis.sh ./script/init_redis.sh
```
### see logs with cargo test <a name="heading49"></a>
```
cargo test -- --nocapture
```
### run integration tests <a name="heading50"></a>
```
cargo test integration
```
(integration tests will fail if they run with unit tests)

### run unit tests <a name="heading51"></a>
```
cargo test unit
```
### run continious integration tests <a name="heading52"></a>
```
cargo test ci
```
### links <a name="heading53"></a>
* [How to connect Robo 3T (Robomongo) to MongoDB Atlas (cloud mongoDB database)](https://www.youtube.com/watch?v=t_X7qFMmWhI) </br>
* [pgadmin create table](https://www.youtube.com/watch?v=h5wgbJiSy7Q) </br>

### run server with modified like js tracing
```
cargo run -q | bunyan
```