# Data types

In this lesson I learned:

 - Rust is *statically typed language*;
 - the general syntaxis is: `let variable_name: data_type = value;`;
 - data type can be skipped if the compiler can infer what type is used;
 - Rust *primitive types* classified into:
    - *scalar types*:
        - *numeric*: ingteger, float;
        - *non-numeric*: boolean, character;
    - *compound*:
        - array;
        - tuple;
 - the **integer** types are:
    - *signed*: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`;
    - *unsigned*: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`;
    - the default integer type is `i32`;
 - the **integer** literals:
    - decimal: 1234, 1_234, 123**u8** (the type can be explicitly specified as a suffix)
    - hex: 0xff
    - octal: 0o77
    - binary: 0b1111_0000
    - byte (`u8` only): b'A'
 - **integer** overflow:
    - *debug* mode: Rust will *panic* at runtime (raise an error);
    - *release* mode: Rust performs *two’s complement wrapping*. For example, in the case of a `u8`, the value 256 becomes 0, the value 257 becomes 1, and so on.
    - These methods helps to explicitly handle the overflowing behaviour:
        - `wrapping_*` - wrap in all modes;
        - `checked_*` - return the None value if there is an overflow;
        - `overflowing_*` - return the value and a boolean indicating whether there was overflow;
        - `saturating_*` - saturate at the value’s minimum or maximum values;

 - the **floating-point** types are `f32`, `f64`, the default float type is `f64`;
 - the **floating-point** numbers are represented according to the IEEE-754 standard;

 - the **boolean** type is specified using `bool` and can hold two values: `true` or `false`;

 - the **character** type is declaring with `char` keyword and scpecified with single quotes;
 - Rust **character** type is four bytes in size and represents *Unicode Scalar Value*, so it can hold not just ASCII but also Asian characters, emoji, and so on;

 - the **tuple** type allows to group together a number of variables with different types;
 - the **tuple** can be turned into separate variables, it is called *destructing*;
 - dot operator (.) allows to access to **tuple** values (see examples in **src/main.rs**);
 - the **tuple** can be an empty, it has a special name, *unit*;

 - the **array** type in Rust havew a fixed length;
 - a collection that is allowed to grow or shrink in size is a **vector**;
 - look **src/main.rs** for the **array** defenitions;
 - Rust will panic (raise runtime error) if **array** index will be invalid. This is one of Rust memory safety principles;

 - **array** and **tuple** can be printed with `{:?}` placeholder notation;
 - slice of **array** allows you to refer to a subset of a contiguous memory location of the array. Unlike an array, the size of the slice is not known at compile time. see **src/main.rs** for how to slice;

 - **array** and **tuple** are immutable by default;
 - the keyword `mut` allow to define mutable **array** or **tuple**.

