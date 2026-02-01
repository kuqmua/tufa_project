### common dev commands

* cargo test --features test-utils -- --nocapture
* RUST_LOG=sqlx=debug cargo test --features test-utils -- --nocapture
* RUSTFLAGS="-Awarnings" RUST_LOG=sqlx=debug cargo test --features test-utils -- --nocapture
* RUSTFLAGS="-Awarnings" cargo clippy --all-targets --all-features

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
sudo docker build -t server-image .
```
#### run docker container <a name="heading277"></a>
```
docker run --env-file .env --name server-container -p 8000:8000 --rm -it server-image
```
#### stop docker container <a name="heading278"></a>
```
sudo docker stop server-container
```
#### remove docker container <a name="heading279"></a>
```
sudo docker rm server-container
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
Details can be found in [rustfmt](https://github.com/rust-lang/rustfmt#configuring-rustfmt)

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
cd .. sudo chmod -R 777 server && cd server
```
#### cargo watch <a name="heading2728"></a>
```
cargo watch -x check -x test -x "run | bunyan"
```
```
cargo watch -q -x run
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
#### see logs with cargo test <a name="heading2749"></a>
```
cargo test -- --nocapture
```
#### run server with modified like js tracing
```
cargo run -q | bunyan
```
#### compilation cache
```
cargo install sccache --locked
```
### up databases
```
cd server && sudo docker-compose up -d && cd ..
```
### run postgres migrations
```  
cd server && sqlx migrate run && cd ..
```