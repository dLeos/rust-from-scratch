use std::io;


fn main() {

    // Borrowing example

    let s = read_str();

    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);


    // Mutable borrowing example
    
    let mut s = String::from("hello");

    println!("String before change '{}'", s);

    change(&mut s);

    println!("String after change '{}'", s);

    change_in_change(&mut s);

    println!("String after one more change '{}'", s);


    // Other examples

    error_examples();
}


fn read_str() -> String {
    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    s.trim().to_string()
}


fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership 
  // of what it refers to, it is not dropped.


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn change_in_change(some_string: &mut String) {
    let s = some_string;  // s is a mutable references
    change(s);
}


fn error_examples() {

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("String after one more change '{}'", r2);

    // code above is OK because r1 is never used and 
    //  it scope ends immediately after the definition


    // line below will produce error, because multiple mutable borrowing 
    //  is not possible in Rust
    // (even if only one of r1, r2 is &mut and the second is &)
    // println!("String after one more change '{}'", r1)
}

