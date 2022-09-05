# Storing Lists of Values with Vectors

In this lesson I learned:

- a *vector* allows you to store a variable number of values next to each other in memory;
- vectors are implemented using *generics*;
- `Vec::new()` function creates an empty vector;
- `vec!` macro creates a new vector that holds given values;
- `push` method adds elements into vector;
- we can't have mutable and immutable (or more than one mutable) references on vector elements in the same scope;
- two approaches to read vector values:
    - *indexing syntax* - using `&` and `[]` gives us a reference to the element at the index value;
    - the `get` method - returns `Option<&T>`;
- *iterating* over values in a vector: we can iterate over immutable or mutable references to each element in a vector using `for` loop;
- storing values of multiple types:
    - if you know set of possible types you can *use an enum to store multiple types*;
    - if you don't know the exhaustive set of types a program will get at runtime to store in a vector, you can use a trait object (it will be covered in Chapter 17);
- once the vector gets dropped, all of its contents are also dropped.

Links:

- `Vec` documentation: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
- Rust built-in collections: https://doc.rust-lang.org/stable/std/collections/

