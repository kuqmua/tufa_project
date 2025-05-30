### MOVE IT todo tomorrow and merge with notes
* fix proc macro submodules, rename them
* make refactoring for module-like implementation (no external dependencies from different modules)
* remove type_path.path.segments.len() check in proc_macros. use .get(index) instead of [index]
* format!("{}, ", e)) for error refactor in format server/src/preparation/check_availability.rs:91:29 error_string
* proc macreo input parameter crate or common
* add use trait import in the scope in case of macro like this 
```
let f: u32 = {
    use common::traits::something::Something;
    0
};
```
* some logic around location() file!() line!() column!() - maybe generate all other functions -github link and others on compiletime instead of runtime?
* for all function with git_info input parameters - use get_git_info instead 
* clippy settings token stream  - cannot do what. if u use this macro - will be an error "error: an inner attribute is not permitted in this context"
* remove CONFIG usage from common
* must use vec of parallel execution vectors vec;
* rust can compile resursive function and get stackoverflow
* todos with github links on place to do
* implements two different methods for code_ocurence with github and just project source. display only for simple impl

### check warning/errors(not the same)
1. cargo check
2. cargo clippy
3. rust-analyzer from vscode extension

#### Location instead of file!() line!() column!()
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
 common/src/server/mongo/something1.rs:70:37 
 server/src/preparation/something1.rs:92:29
time [key] error2
 common/src/server/mongo/something2.rs:70:37 
 server/src/preparation/something2.rs:92:29
server/src/preparation/parent.rs:92:29
server/src/preparation/parent.rs:45:29


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
git submodule add https://github.com/kuqmua/client.git
```
```
git submodule add https://github.com/kuqmua/server.git
```
### up databases
```
cd server && sudo docker-compose up -d && cd ..
```
### run postgres migrations
```  
cd server && sqlx migrate run && cd ..
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

### do not compile program if there is no explecit version of dependencies. proc macro open cargo toml files

# install cargo-pgrx

