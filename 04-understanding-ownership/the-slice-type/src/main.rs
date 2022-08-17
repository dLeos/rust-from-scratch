


fn main() {
    let my_string = String::from("Hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);


    mutable_slice_example();
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn mutable_slice_example() {
    let mut a = [1, 2, 3, 4, 5];
    println!("array before: {:?}", a);

    let slice = &mut a[1..3];

    // this line will produce error because multiple mutable borrows occurs
    // let slice2 = & a[4..5];

    slice[0] = 100;

    println!("slice: {:?}", slice);
    println!("array after: {:?}", a);
}

