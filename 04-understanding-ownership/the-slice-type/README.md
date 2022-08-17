# References and Borrowing

In this lesson I learned:
  - *slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection;
  - *slice* is a kind of reference, so it does not have ownership;
  - a *string slice* is a reference to part of a `String`;
  - string literals are slices. It's a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an immutable reference;
  - to go through the `String` element by element, it should be converted to an array of bytes using the `as_bytes` method. Next, we create an iterator over the array of bytes using the `iter` method;
  - `i32` integer array slice has type `&[i32]`.

