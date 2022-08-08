use std::io;


fn main() {

    // Integer data type

    let x = 0x1_234u32;
    println!("x: {x}");

    let x_code = b'x';
    println!("ASCII code of X:{x_code}");


    // Integer overfloating
    //   If you will enter number from 128 to 255 the program will:
    //     - debug mode: panicked at 'attempt to add with overflow';
    //     - release mode: continue to work performing two’s complement wrapping.

    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
        
    let x: u8 = x.trim().parse().expect("Failed to parse integer");
    let x = x + 128u8;
    println!("x: {x}");


    // Handling integer overfloating
    //   An example with saturating to minimum or maximum values
    //   If you will enter number from 128 to 255 the program will assign x 255u8
    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
        
    let x: u8 = x.trim().parse().expect("Failed to parse integer");
    let x = x.saturating_add(128u8);
    println!("x: {x}");


    // Integer division

    let x = 13 / 5;
    println!("x: {x}");


    // Floating-point data type

    let x = 13.0 / 5f32;
    println!("x: {x}");


    // Boolean data type

    let t = true;
    let f: bool = false;
    println!("{t} != {f}");


    // Character data type

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("characters: {c} {z} {heart_eyed_cat}");


    // Tuple data type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);

    let unit_tup = ();
    println!("Empty tuple: {:?}", unit_tup);
    
    let mut tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple values are: {x} {y} {z}");

    let x = tup.0;
    println!("The fisrt value is: {x}");

    let (_, y, _) = tup;
    println!("The second value is: {y}");

    let z = tup.2;
    println!("The third value before changing is: {z}");

    tup.2 = tup.2 + 1;
    let z = tup.2;
    println!("The third value after changing is: {z}");


    // Array data type

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("First two elements of an array: {first} {second}");

    let b = [3; 5];
    println!("Array: {:?}", b);
    println!("Array len: {}", b.len());


    // Rust will panic if array index would be invalid

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("Please enter an month index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = months[index - 1];

    println!("The {index} month is: {element}");


    // Get slice of the array

    let summer_months = &months[5..8];
    println!("Summer months are: {:?}", summer_months);

    let september_index = 8;
    let december_index = 11;
    let autumn_months: &[&str] = &months[september_index..december_index];
    println!("Autumn months are: {:?}", autumn_months);
}
