# What is ownership?

*Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.*

In this lesson I learned:

 - *ownership* is a set of rules that governs how a Rust program manages memory;
 - *ownership* rules:
   - each value in Rust has an *owner*;
   - there can only be *one* owner at a time;
   - when the owner goes out of scope, the value will be *dropped*;
 - when the size of data is known at compile-time, a **stack** is used for storage. Example: all primitive data types that have a fixed size;
 - when the size of data is not known at compile time rather it is known at the run time, it goes in a portion of program memory called a **heap**. Example: `Vector` and `String` objects.
 - Rust has at least two types of string in  `str` and `String`:
   - `str` strings are immutable and their content must be known at compile time;
   - in case of `str` the text is hardcoded directly into the final executable;
   - `String` value can be mutated and can store text that is unknown at compile time;
   - `String` type organized as a heap which allows dynamically allocate memory in runtime;
 - Rust *allocate* amount of memory for heap-based variables in runtime when variable is initialized;
 - allocated memory is **automatically returned** once the variable that owns it goes out of scope. When a variable goes out of scope, Rust automatically calls `drop` function;
 - instead of copy operator `=` **moves** the right variable to the left one, to avoid *double free* error in the future;
 - Rust will never automatically create *deep* copies of your data;
 - method `clone` is used to make *deep* copy the *heap data*;
 - Rust has a special annotation called the `Copy` trait that is placed on types that are stored on the stack. If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable. That's why for example integer variables are copied rather than moved;
 - here are some of the types that implement `Copy`:
    - all the integer types, such as `u32`;
    - the Boolean type, `bool`, with values `true` and `false`;
    - all the floating point types, such as `f64`;
    - the character type, `char`;
    - tuples and arrays, if they only contain types that also implement `Copy`. For example, (`i32`, `i32`) implements `Copy`, but (`i32`, `String`) does not;
 - passing a variable to a *function* will move or copy, just as assignment does. Heap-based function variables will free after the function will be finished;
 - function returning values can also transfer *ownership*;
 - examples are in **src/main.rs**.
