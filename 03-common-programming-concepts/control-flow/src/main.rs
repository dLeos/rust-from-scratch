use std::io;


fn main() {
    
    if_example();

    loop_example();

    while_example();

    for_example();
}


fn if_example() {

    // example of using if .. else ..

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    // example of using if .. else if .. else ..

    let number = 6;

    if number % 4 == 0 {
        println!("number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("number {number} is divisible by 2");
    } else {
        println!("number {number} is not divisible by 4, 3, or 2");
    }


    // example of using if expression in let statement 

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


fn loop_example() {

    // loop example

    let mut x = 0;

    loop {
        x += 1;

        if x > 3 {
            break;
        }

        println!("In loop");
    }

    println!("Finished");


    // returning value from loop

    let x: i32 = loop {
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("x: {x}");


    // Multiple loops with labels

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}...");

        number -= 1;
    }

    println!("While finished!");
}


fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("for loop: the value is: {element}");
    }

    println!("LIFTOFF!!!");


    for number in 1..4 {
        println!("for loop: {number}");
    }

    println!("LIFTOFF!!!");


    for number in (1..4).rev() {
        println!("for loop: {number}");
    }

    println!("LIFTOFF!!!");
}
