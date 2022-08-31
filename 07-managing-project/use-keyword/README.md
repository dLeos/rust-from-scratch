# Bringing Paths into Scope with the use Keyword

The `use` keyword creates a shortcut to a path, and then the shorter name can be used everywhere else in the scope. Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.

The `as` keyword after the path in use statement, used to specify a new local name, or *alias*, for the type;

The `pub use` construction allows *re-exporting* bringing items, making that item available for others to bring into their scope. It helps to provide better interface for users;

The *idiomatic* way how to use bring items into scope:
  - the *idiomatic* way to bring a *function* into scope with use is by bringing the function's parent module into scope;
  - the *idiomatic* way to bring *structs, enums, and other items* into scope with use is to specify the full path (example: `use std::collections::HashMap;`) unless there are no two items with the same name in the scope.


Bringing external modules:
  - the keyword `use` also allows bringing external modules. External dependency should be described in *Cargo.toml*;
  - standard `std` library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change *Cargo.toml* to include `std`.

Combining several use statements into one line:
  - if we’re using multiple items defined in the same crate or same module, we can combine *nested paths* into one line using curly brackets (example `use std::{cmp::Ordering, io};`);
  - if one of paths is a complete path, then to merge these paths into one use statement, we can use *self* in the nested path (example: `use std::io::{self, Write};`).
  - *the glob operator* `*` allow bringing *all* public items defined in a path into scope. But this makes it harder to tell what names are in scope and where a name used in your program was defined.

Many packages are available at [crates.io](https://crates.io/).

