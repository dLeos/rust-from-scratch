# References and Borrowing

In this lesson I learned:
 - a *reference* allows to access the data stored at that address; that data is owned by some other variable (like a pointer); 
 - a *reference* is guaranteed to point to a valid value of a particular type for the life of that reference (unlike a pointer);
 - ampersands (`&`) represent references, and it allows to refer to some value without taking ownership of it (see examples at **src/main.rs**);
 - the opposite of referencing by using `&` is *dereferencing*, which is accomplished with the dereference operator, `*`;
 - the action of creating a reference called *borrowing*;
 - by default references are *immutable*;
 - a *mutable reference* allows to modify a borrowed value;
 - mutable references have one big restriction: *if you have a mutable reference to a value, you can have no other references to that value*. It means that only one variable in the scope can borrow mutably;
 - Rust also does not allow us to have a mutable reference while we have an immutable one to the same value;
 - thus:
    - multiple immutable references: OK;
    - one mutable reference: OK;
    - mutable + immutable references: ERROR;
    - multiple mutable references: ERROR;
 - a *reference’s scope* starts from where it is introduced and continues through *the last time that reference is used*. The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called *Non-Lexical Lifetimes* (NLL);
 - ...

