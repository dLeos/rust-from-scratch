fn main() {

    move_example();

    clone_example();

    array_example();

    function_ownership_example();

}


fn move_example() {

    let s1 = String::from("Hello");
    println!("s1: {s1}");

    let s2 = s1;
    println!("s2: {s2}");

    // this will produce an error because s1 was moved to s2
    // println!("s1: {s1}"); 

    {
        let s3 = s2;
        println!("s3: {s3}");
    }

    // this will produce an error because s2 was moved to s3
    // println!("s2 after s3 freeing: {s2}");

}


fn clone_example() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    s2.push_str(", world!");

    println!("s1 = {}, s2 = {}", s1, s2);
    
}


fn array_example() {
    // example with primitive data type

    let arr: [i32; 3] = [1, 2, 3];

    {
        let mut arr_inner = arr;
        arr_inner[1] = 20;
        println!("arr_inner {:?}", arr_inner);
    }

    // this is ok, because Array is Copy and i32 is Copy
    println!("arr {:?}", arr);


    // example with not primitive data type

    let arr: [String; 2] = [String::from("foo"), String::from("bar")];
    println!("arr {:?}", arr);

    {
        let mut arr_inner = arr;
        arr_inner[1] = String::from("baz");
        
        println!("arr_inner {:?}", arr_inner);
    }

    // this is will produce an error because String is not Copy
    // println!("arr {:?}", arr);
}


fn function_ownership_example() {

    let s = String::from("hello");
    takes_ownership(s);

    // this will produce an error because s's value moves into the function
    //   and so is no longer valid here
    // println!("s: {s}");


    let x = 5;
    makes_copy(x);

    // this is ok, because i32 is Copy, so it can be used after the function
    println!("x: {x}");


    let s = String::from("hello");
    let s = takes_and_gives_back(s);

    // this is ok, because function returns ownership
    println!("s: {s}");

}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("some_string: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  //  memory is freed.


fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("some_integer: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


// This function takes a String and returns one
fn takes_and_gives_back(some_string: String) -> String { // a_string comes into scope
    println!("some_string: {}", some_string);
    some_string  // a_string is returned and moves out to the calling function
}

