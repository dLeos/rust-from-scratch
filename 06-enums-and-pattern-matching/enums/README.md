# Defining an Enum

In this lesson I learned:
  - *enumerations*, also referred to as *enums*, give you a way of saying a value is one of a possible set of values;
  - an *instance* of *enum* can only be one of its variants;
  - when enum instance used as function parameter or struct field its type corresponds to enum type;
  - *enum variants* can hold data;
  - *each variant can have different types and amounts of associated data!*;
  - you can put any kind of data inside an enum variant: strings, numeric types, structs, or another enum for example;
  - enum variant can hold named fields like a struct does;
  - like structs, enums can have methods and associated functions;
  - Rust doesn’t have the *null* feature that many other languages have (it saves from possible errors);
  - the `Option` enum implements the concept of a value being present or absent;
  - the `Option` enum defined by the standard library;
  - the `Option` type encodes the very common scenario in which a value could be something or it could be nothing;
  - the `Option` and its variants `Some` and `None` are already included in the prelude: you can use them directly without any prefixes.
  - the `<T>` syntax denotes generic type parameter;
  - you have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. This helps catch one of the most common issues with null: assuming that something isn't null when it actually is.

The `Option` documentation:
  - https://doc.rust-lang.org/stable/std/option/enum.Option.html

