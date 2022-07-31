# ERROR SOLVING

## Unresolved import

```rust
error[E0432]: unresolved import `actix_web`
 --> src\main.rs:1:5
  |
1 | use actix_web::{web, App, HttpRequest, HttpServer, Responder};
  |     ^^^^^^^^^ use of undeclared crate or module `actix_web`
```
### Solution 
In `Cargo.toml` add the missing dependecy under `[dependecies]` manifest key:

```toml
#! Cargo.toml
# [...]

[dependencies]
actix-web = "4"
```

or add it with `cargo-edit`:
```bash
cargo install cargo-edit #not installed by default
cargo add actix-web --vers 4.0.0
```
## Undeclared Crate
```rust
error[E0433]: failed to resolve: use of undeclared crate or module `tokio`
 --> src\main.rs:8:3
  |
8 | #[tokio::main]
  |   ^^^^^ use of undeclared crate or module `tokio`
```
### Solution

In `Cargo.toml` add the missing crate under `[dependecies]` manifest key:

```
#! Cargo.toml
# [...]

[dependencies]
# [...]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```
Use `features = ["full"]` to add all the features.

## Main cannot be async
```rust
error[E0752]: `main` function is not allowed to be `async`
 --> src\main.rs:9:1
  |
9 | async fn main() -> std::io::Result<()> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`

```
### Solution

`main` cannot be async and it have to use `Future` trait based on polls.
You are using `HttpServer` or any other async function somewhere.
Add `#[tokio::main]` on top of the main procedure.

## Unused variable
```rust
warning: unused variable: `req`
 --> src/main.rs:3:23
  |
3 | async fn health_check(req: HttpRequest) -> impl Responder {
  |                       ^^^ 
  | help: if this is intentional, prefix it with an underscore: `_req`
  |
  = note: `#[warn(unused_variables)]` on by default
  ```
### Solution
Did you miss something? Maybe you can remove the unused variable or put the underscore as prefix of the variable name.

## no {function name} in the root
```rust
error[E0432]: unresolved import `sojurust::run`
 --> src/main.rs:3:5
  |
3 | use sojurust::run; //run is the function name
  |     ^^^^^^^^^^^^^ no `run` in the root

For more information about this error, try `rustc --explain E0432`.
```
### Solution
Simply rust cannot find the function, check your source code.

## Running 0 tests
Rust can't find unit-tests
```
PS C:\Users\zerob\Desktop\PROGETTI\soju-rust> cargo test
   Compiling sojurust v0.1.0 (C:\...\soju-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.88s
     Running unittests src/lib.rs (target\debug\deps\sojurust-c7f8521fe6946b6e.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target\debug\deps\sojurust-aca568cca7a1187e.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests sojurust

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Solution
I had my tests in `test` folder but folder should have named `tests`.

# Async function test would not terminate
`test health_check_works has been running for over 60 seconds`

### Solution
Refactor!


## this type alias takes 1 generic argument but 2 generic arguments were supplied
```rust
error[E0107]: this type alias takes 1 generic argument but 2 generic arguments were supplied
  --> src/lib.rs:8:26
   |
8  | pub fn run() -> std::io::Result<Server, std::io::Error> {
   |                          ^^^^^^         -------------- help: remove this generic argument
   |                          |
   |                          expected 1 generic argument
   |
note: type alias defined here, with 1 generic parameter: `T`
  --> C:\Users\zerob\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\std\src\io\error.rs:55:10
   |
55 | pub type Result<T> = result::Result<T, Error>;
   |          ^^^^^^ -
```
### Solution

-.-  99% this error is verbose

# OTHER
Remember the .expected("err message") to unwrap a Result

