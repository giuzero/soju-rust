# Notes
## Upload a project that has alredy begun

```bash
git remote add origin https://github.com/giuzero/soju-rust.git
git branch -M main
git push -u origin main
```

## Other building tools after installation
```bash
cargo install -f cargo-binutils
cargo install cargo-watch
rustup component add llvm-tools-preview
```

then in `%USERPROFILE%\.cargo\config.toml.`:
```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### Use  cargo-watch
`cargo-watch` monitors your source code to trigger commands every time a file changes.
```bash
cargo watch -x check
```
will run cargo check after every code change.
It supports chaining:
```bash
cargo watch -x check -x test -x run
```

## CI
Continuous Integration empowers each member of the team to integrate their changes into the main branch multiple times a day.

Tight feedback loop.

### Test

```bash
cargo test
```

### Code Coverage
tarpaulin is just for linux right now
### Linting
Check for unidiomatic code
```bash
cargo clippy
```
### Formatting
```bash
cargo fmt
```
### Check for vulnerabilities
```bash
cargo install cargo-audit
cargo audit
```
There is `cargo-deny` for vulnerability checking in the dependency tree.

## Start application
```bash
cargo run
```

## actix-web
Is the to-go rust web-framework.
```rust
//! src/main.rs
// [...]

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```
`HttpServer` handles transport.
`App` handles application logic, in this case routing:
```rust
.route("/", web::get().to(greet))
```
`Route` is composed by handlers and guards. Guards specify conditions to satisfy. Guards implements `Guard` and `Guard::check` is where validation happens.

`web::get()` is a short-cut for `Route::new().guard(guard::Get())`, the request should be passed to the handler if and only if its HTTP method is GET.

`greet` is the handler:
```rust
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hellooooo {}!", &name)
}
```
It implements `Responder` _Trait_, it defines what is returned.

## tokio::main
`main` cannot be async. Async funtions are based on `Future` trait, so would wait for a vaule that is not ready yet. This value is monitored by polling by rust (_are you ready now? and now? and now?_). Its standard library does not include asynchronous runtime, `tokio::main` would take care of polling.

### cargo-expand
It expands procedural macros.

## Health check for actix-web routing
`Responder` is a conversion trait into a `HttpResponse`. We will return a `HttpResponse`instance. `HttpResponse::Ok` is used to get a `HttpResponseBuilder` with 200 code. Since `HttpResponseBuilder` would try to give us a richer response we need to force an empty body with `finish` (it could be omitted really, because `HttpResponseBuilder` implements `Response` as well):
```rust
async fn health_check(req: HttpRequest) -> impl Responder {
    //HttpResponse::Ok().finish()
    HttpResponse::Ok()
}
```
Let's register the handler:
```
App::new()
    .route("/health_check", web::get().to(health_check))
```
## Integration Test
Check the introduction of regressions.
Tests chan be embedded in source code, put in another folder, put in the documentation.

### Clean main
Define [package] and [\[bin]] in Config.toml.
Let's have a main che call a function from `lib.rs`.

```toml
[...]
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "sojurust"
```

This is my code now, just `src/main.rs`:

```rust
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hellooooo {}!", &name)
}

async fn health_check() -> impl Responder {
    //HttpResponse::Ok().finish()
    HttpResponse::Ok()
    
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```
I would move it to `src/lib.rs`, cleaning what I don't need:
```rust
use actix_web::{web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .bind("127.0.0.1:8000")?
        .run() //HttpServer::run
        .await
}
```

Clean `main.rs`:

```rust
use sojurust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
```

Add `[dev-dependencies]` to `Config.toml`. These will be used only in tests.

```
[dev-dependencies]
reqwest = "0.11"
```

Use `[tokio:test]` as runtime crate in tests, then assert, assert, assert!
```rust
`[tokio:test]`
async fn health_check_works() {
    // this is the only thing that depends directly on our code
    spawn_app().await.expect("Failed to spawn our app.");
    
    // perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch app in the background
async fn spawn_app() -> std::io::Result<()> {
    sojurust::run.await()
}
```

In `sojurust::run` we invoke and await `HttpServer::run` that return a `Server`.
The `await` call listens for ever, so `spawn_app` never returns and test logic is never executed. We need `tokio::spawn` to make our application run in background, it will take care of polling and can concurrently run the test logic. In `sojurust::run` we have to return a `Server` without awaiting for it.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server; //new

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// change signature
// no more async
// we return a Server
pub fn run() -> std::io::Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .bind("127.0.0.1:8000")?
        .run() //HttpServer::run
        //.await not needed
    Ok(server)
}
```
Change test and spawn app
```rust
//! tests/health_check.rs
// [...]

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() { //no more async signature
    // No .await, no .expect
    spawn_app();
    // [...]
}
```
### Random port for server
Change binding to `bind(address)` passing an argument `run(address: &str)`. We will pass`127.0.0.1:0`, `0` for random port for testing.

But we need to check what ports are busy: `std::net::TcpListener`.
Use `reqwest` in tests.

### Conversion
%40 = @

%20 = _space_

### Vector for table-driven test
```
let test_cases = vec![
        ("name=geppetto", "no mail"),
        ("email=g.boskonovitch%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
```
With parametrised tests it is important to have good error messages on failures.

### Extractors
Parsing: Form data helper (application/x-www-form-urlencoded) can be used to extract url-encoded data from the request body, or send url-encoded data as the response.
```rust
use serde::Deserialize;
...
#[derive(serde::Deserialize)] //use serde to deserialize (in Cargo.toml serde = { version = "1.0", features = ["derive"] })
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
```
## DB
Use sqlx with Postgres

> Im using WINDOWS10, postgres as docker and pgadmin as docker. I wasted plenty of time trying to make it work.

```bash
docker pull postgres
docker run -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -e POSTGRES_DB=sojudb -p 5432:5432 -d postgres -N 1000


#In windows powershell
$env:DATABASE_URL = 'postgres://user:password@localhost:5432/sojudb'
# as if it was export DATABASE_URL=postgres://user:password@localhost:5432/sojudb

#this is just sqlx-cli
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
sqlx create database
sqlx migrate add create_subscriptions_table
  ```
Now a new folder is created with an empty migration sql script. We have to insert our DDL:
```sql
-- migrations/{timestamp}_create_subscriptions_table.sql
-- Create Subscriptions Table
CREATE TABLE subscriptions(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   name TEXT NOT NULL,
   subscribed_at timestamptz NOT NULL
);
```
Then run the migration:
```
sqlx migrate run
```
Let's check with pga:

> :warning: Do not use user@domain.local as default email, I lost a lot of time trying to access pga, do not know why... #FML
```
docker pull dpage/pgadmin4:latest

docker run --name my-pgadmin -p 5050:80 -e 'PGADMIN_DEFAULT_EMAIL=user@gmail.com' -e 'PGADMIN_DEFAULT_PASSWORD=root'-d dpage/pgadmin4
```
You can find pga in your browser `http://localhost:5050`.



> :warning: You need to use host.docker.internal instead of localhost as db host when you set your local db connection. ...#FML -.-''''

You'll find your table in : `Servers>localhost>Databases>Schemas>public>Tables>subscriptions`

Install sqlx, adding in `Cargo.toml`:
```toml
[dependencies]
# [...]

# Using table-like toml syntax to avoid a super-long line!
[dependencies.sqlx]
version = "0.5.7"
default-features = false
features = [
    "runtime-actix-rustls", # tells sqlx to use the actix runtime for its futures and rustls as TLS backend
    "macros", # to enable sqlx::query! and sqlx::query_as!
    "postgres", # to use some postgres specific features
    "uuid", # uuid support
    "chrono", # date support
    "migrate"
]
```
And now, clean up.
Achieve di scaffolding and file modifications:
```
src/
  configuration.rs
  lib.rs
  main.rs
  routes/
    mod.rs
    health_check.rs
    subscriptions.rs
  startup.rs
```
```rust
//! src/lib.rs
pub mod configuration;
pub mod routes;
pub mod startup; //run function
```

`subscribe` and `FormData` are re-exported in routes/mod.rs:

```rust
//! src/routes/mod.rs
mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;
```

Some refactory is needed with some visibility pub. 
Result as branch "checkpoint 1"