
// constant can be declared in global scope:

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// while variable cannot:

// let THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;   // produce an error


fn main() {
    
    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS}");

    // Mutable variable example:

    let mut x = 5;
    println!("The value of x is: {x}");

    x = x + 1;
    println!("The value of x is: {x}");


    // Shadowing example:

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    // Shadowing + mutable example:

    let x = 5;

    let mut x = x + 1;

    {
        x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
