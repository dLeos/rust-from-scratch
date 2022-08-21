# Defining and Instantiating Structs

In this lesson I learned:
  - a *struct*, or *structure*, is a custom data type that lets you package together and name multiple related values that make up a meaningful group;
  - like *tuples*, the pieces of a *struct* can be of different types;
  - unlike with *tuples*, in a *struct* each piece of data has *name*;
  - *struct* defined with the keyword `struct`, see examples in **src/main.rs**;
  - when specifying *instance* of struct the order of fields can be changed;
  - *instance* can only be mutable in its entirety; Rust doesn’t allow us to mark only certain fields as mutable;
  - *field init shorthand* syntax lets you avoid repetition of field names when instance of struct is initialized (see example in **src/main.rs**: `email` instead of `email: email`);
  - *struct update syntax* lets you initialize all not-specified values same value as the fields in the given instance; the instance should be of the same type;
  - struct update syntax uses `=` like an assignment, if some fields being updated are not implemented `Copy` trait then given instance cannot be used after creating new one;
  - Rust has a *tuple structs*: *tuple structs* have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields;
  - each *tuple struct* has its own type, even though the fields within the struct might have the same types;
  - a dot (`.`) followed by the index lets you access an value of tuple struct field;
  - Rust has a *unit-like structs*: *unit-like struct* is a struct without any fields.
