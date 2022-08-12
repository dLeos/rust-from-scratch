# What is ownership?

*Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.*

In this lesson I learned:

 - *ownership* is a set of rules that governs how a Rust program manages memory;
 - *ownership* rules:
   - each value in Rust has an *owner*;
   - there can only be one owner at a time;
   - when the owner goes out of scope, the value will be dropped;
 - ...


