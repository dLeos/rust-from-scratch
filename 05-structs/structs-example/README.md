# An example program using structs

In this lesson I learned:
 - *structs* help to organize code and make it more readable;
 - *attribute* `[derive(Debug)]` just before the struct definition allows to include functionality to print out debugging information;
 - placeholder `{:#?}` is pretty-printed version of `{:?}`;
 - macro `dbg!` is alternative way to print out a value;
 - `println!` takes a reference;
 - `dbg!` takes ownership of an expression and results in the value of this expression. It allows to use `dbg!` directly in expressions;
 - `dbg!` print to the standart error stream (stderr).

