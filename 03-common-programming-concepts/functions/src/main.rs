fn main() {

    // Call a funtion

    print_labeled_measurement(5, 'h');
    print_labeled_measurement(5, 104 as char);


    // Statements vs expressions

    let y = {
        let x = 4; // this is statement
        x + 1      // this is expression
    };
    print_labeled_measurement(y, 'h');


    // Call a function which returns value

    print_labeled_measurement( five() , 'h');
    print_labeled_measurement( plus_one(4) , 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
