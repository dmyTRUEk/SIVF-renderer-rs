# SIVF renderer (rust implementation)



## FAQ - Frequently Asked Questions:
### Do everything:
```
$ cargo test && cargo doc && cargo run
```

### Run `main.rs`:
```
$ cargo run
```

### Run All Tests:
```
$ cargo test
```
#### Run Unit Tests:
```
$ cargo test --bin sivf-renderer-rs
```
#### Run Integration Tests:
```
$ cargo test --test '*'
```

### Compile Docs:
```
$ cargo doc
```

### Open Docs in browser:
```
file:///path_to_sivf_renderer_rs/target/doc/sivf_renderer_rs/index.html
```



## Useful links:
### Project Organisation
- [Rust Project Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [useful `deny`s](https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html)

### Rust `std`
- [Rust Docs](https://doc.rust-lang.org/std/)
- [std::Iterator](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- [Objects with Trait](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
- [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html)  
  [using `Any` with traits](https://stackoverflow.com/questions/42056422/using-any-with-traits-in-rust)

### VS
- [`to()` vs `into()`](https://stackoverflow.com/questions/25316115/difference-between-tostring-and-intostring)
- [`from` vs `as`](https://stackoverflow.com/questions/48795329/what-is-the-difference-between-fromfrom-and-as-in-rust)

### Libs
- [Image lib examples](https://github.com/image-rs/image/tree/master/examples)
- [Derive More](https://docs.rs/derive_more/0.99.16/derive_more/index.html)
