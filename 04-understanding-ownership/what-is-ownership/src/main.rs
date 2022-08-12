fn main() {

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


