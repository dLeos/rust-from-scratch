# Method syntax

In this lesson I learned:
  - Rust *methods* are like functions but defined within the context of a struct/enum/trait object;
  - `impl` block used to define methods;
  - the first argument of methods is `&self` or `&mut self` aobject, which is a short of `self: &Self` or `self: &mut Self`;
  - Rust has a feature called *automatic referencing and dereferencing*: when you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so object matches the signature of the method;
  - functions defined within an `impl` block are called *associated functions*, because they’re associated with the type named after the `impl`;
  - *associated functions* don’t have `self` as their first parameter;
  - *associated functions* which returns a new instance of the struct are often called `new`, but `new` isn’t a special name and isn’t built into the language;
  - each struct is allowed to have *multiple* `impl` blocks.