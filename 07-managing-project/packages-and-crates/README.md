# Packages and Crates

In this lesson I learned:
  - a *crate* is the smallest amount of code that the Rust compiler considers at a time;
  - a crate can come in one of two forms: a *binary crate* or a *library crate*;
  - *binary crates* are probrams that you can compile and execute;
  - *library crates* define functionality intended to be shared with multiple projects;
  - *library crates* don't have main function and they are not executable;
  - the *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate;
  - a *package* is a bundle of one or more crates and *Cargo.toml* file that describes how to build those crates;
  - a package can contain as many binary crates as you like, but at most only one library crate???
  - a package must contain at least one crate, whether that's a library or binary crate;
  - Cargo follows a convention that **src/main.rs** is the crate root of a binary crate with the same name as the package;
  - Cargo knows that if the package directory contains **src/lib.rs**, the package contains a library crate with the same name as the package, and **src/lib.rs** is its crate root;
  - Cargo passes the crate root files to rustc to build the library or binary;
  - a package can have multiple binary crates by placing files in the **src/bin** directory: each file will be a separate binary crate (see example at **src**).
