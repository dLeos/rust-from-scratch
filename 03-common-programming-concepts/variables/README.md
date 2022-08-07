# Variables and Mutability

CLI application. Use `cargo run` to run it.

In this lesson I learned:

 - by default Rust variables are immutable, the keyword `mut` is used to make it mutable;
 - Rust naming convention is to write variable names in *snake_case*: lowercase and underscopes between words;
 - it is possible to assign multiple variables in one statement: `let (hello, world) = ("Hello", "world");`;
 - the difference between immutable variables and constants:
    - constants defined with keyword `const`;
    - constants always immutable;
    - constants require data type annotation;
    - constants can be declared in any scope, including the global scope;
    - constants may be set only to constant expression, not to runtime result;
    - constants cannot be *shadowed*;
    - constants naming convention in Rust is to use *SCREAMING_SNAKE_CASE*: uppercase and underscopes between words;
 - the *shadowing* concept, including *shadowing* in inner scope (see **src/main.rs**);
 - the difference between *shadowing* and mutable variables: *shadowing* can change the type of the value but reuse the same name.
