# Guessing game

CLI application. Use `cargo run` to run it.

In this lesson I learned:

 - how to import standard library: `use std::io;`
 - how to define variable: `let mut guess = String::new();`
 - `std::io::Stdin` object has method `read_line(&self, buf: &mut String)`;
 - the result of read_line method is a `Result` value, that is Enum. `Result`'s variants are `Ok` and `Err`. An instance of `Result` has an `expect(self, msg: &str)` method;
 - printing with placeholders;
 - how to import *library crates*: by adding dependencies to Cargo.toml;
 - cargo versions supports [Semantic Versioning](https://semver.org/);
 - the general idea of **Cargo.lock** file: writing the dependency versions to make build reproducible;
 - `cargo update` command: updating minor versions;
 - `cargo doc --open` command: building and opening documantation of all local dependencies;
 - how to generate random value:
    - `rand::thread_rng()` method returns the `ThreadRng` which is implement the `RngCore` trait, and as consequence `Rng` trait;
    - `rand::Rng` trait has method `gen_range<T, R>(&mut self, range: R)` which allows generating random value from a range;
    - to describe range the struct `core::ops::Range` is used;
 - Rust allows to *shadow* variables (reuse variable name in the same scope);
 - how to conver string to number type: using `parse()` method, the result of this method is `Result` value;
 - how to compare to numbers:
    - `std::cmp` is a part of the standart library;
    - `cmp` method allows to compare two objects and returns `std::cmp::Ordering` value which is Enum with variants `Less`, `Greater`, and `Equal`;
    - `match` expressions allow to compare values against Enum elements; 
 - how to create an infinite loop;
 - how to `continue` and `break` in the loop;
 - `match` expression allows to *return a value* for one pattern and to `continue` loop iteration for another one.


Additional links:

 - `read_line` method documentation: https://doc.rust-lang.org/stable/std/io/struct.Stdin.html#method.read_line
 - `expect` method documentation: https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect
 - Crate `rand` documentation: https://docs.rs/rand/0.8.5/rand/index.html
 - Struct `core::ops::Range` documentation: https://doc.rust-lang.org/nightly/core/ops/struct.Range.html
 - `str` primitive type documentation: https://doc.rust-lang.org/stable/std/primitive.str.html

