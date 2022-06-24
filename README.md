# hello.rust

Evaluating [Rust](https://www.rust-lang.org/)

## Development

Having installed the Rust Toolchain using [rustup](https://www.rust-lang.org/tools/install)

```shell
cd app

# build debug
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
# build release
$ cargo build -r
   Finished release [optimized] target(s) in 0.00s

# run tests
$ cargo test
...
     Running tests\test_example.rs (target\debug\deps\test_example-2659a83bdf422e1d.exe)

running 6 tests
test tests::test_add ... ok
...


# run, again run release by adding `-r`
$ cargo run
   Compiling hello-rust v0.1.0 (...\hello.rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target\debug\hello-rust.exe`
...
$ cargo run -r
    Finished release [optimized] target(s) in 0.00s
     Running `target\release\hello-rust.exe`
```

## Open Topics

- Code Coverage
  - The rustc book [instrument-coverage](https://doc.rust-lang.org/rustc/instrument-coverage.html)
- Containerization
  - Official [rust](https://registry.hub.docker.com/_/rust/) image
  - [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)
  - [5x Faster Rust Docker Builds with cargo-chef](https://www.lpalmieri.com/posts/fast-rust-docker-builds/)
  - [Packaging a Rust web service using Docker](https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/)
- Rust Frameworks
  - [Which Rust web framework to choose in 2022 (with code examples)](https://kerkour.com/rust-web-framework-2022)
  - [Web Frameworks](https://www.arewewebyet.org/topics/frameworks/)

## References

- [rust-starter/rust-starter](https://github.com/rust-starter/rust-starter)
- [rust-unofficial/awesome-rust](https://github.com/rust-unofficial/awesome-rust)
- The rustc book [Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [Rust Memory Container Cheat-sheet](https://github.com/usagi/rust-memory-container-cs)
