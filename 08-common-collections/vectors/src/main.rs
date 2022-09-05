fn main() {

    // example of initialization

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    println!("{:?}", v);
    

    // example of reading vector values

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    

    // example of iterating over the vector

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
    

    // example of using enum to store multiple types

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

}   // <- "v" and "row" goes out of scope and is freed here
