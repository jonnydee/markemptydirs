# markemptydirs

## Building

`markemptydirs` is written in Rust, so you'll need to grab a
[Rust installation][rust] in order to compile it.
It compiles with Rust 2018 Edition or newer. In general, `markemptydirs` tracks
the latest stable release of the Rust compiler.

To build:

```sh
git clone https://github.com/jonnydee/markemptydirs-rs
cd markemptydirs
cargo build --release
```

A subsequent:

```sh
./target/release/markemptydirs --version
```

Should output:

```text
markemptydirs 0.1.0
```
## Known Issues

- Application crashes when no command-line parameter is provided.
- `purge` command currently not yet implement.
- **The software is still experimental. Use at your own risk!**


[rust]: https://www.rust-lang.org
