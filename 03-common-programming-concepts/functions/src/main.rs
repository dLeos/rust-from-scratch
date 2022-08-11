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


    // Pass arguments by value

    let mut n = 4;
    println!("The value of n before function call : {}", n);
    println!("Invoke Function");
    square_value(n);
    println!("The value of n after function call : {}", n);


    // Pass arguments by reference

    println!("The value of n before function call : {}", n);
    println!("Invoke Function");
    square_reference(&mut n);
    println!("The value of n after function call : {}", n);


    // Example of function with multiple output
    let n = 13;
    let k = 5;
    let (a, b) = divmod(n, k);
    println!("{a} * {k} + {b} = {n}");


    // Example pf functions with array argument
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("sum of {:?}: {}", arr, sum_array_value(arr));
    println!("sum of {:?}: {}", arr, sum_array_reference(&arr));

    println!("array before reverse {:?}", arr);
    reverse_array(&mut arr);
    println!("array after reverse {:?}", arr);
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

fn square_value(mut n:i32){
    n = n * n;
    println!("The value of n inside function : {}", n);
}

fn square_reference(n:&mut i32){
    *n = *n * *n;
    println!("The value of n inside function : {}", n);
}

fn divmod(n: i32, k: i32) -> (i32, i32) {
    (n / k, n % k)
}

fn sum_array_value(arr:[i32;5]) -> i32 {
    let mut sum = 0;
    for element in arr {
        sum += element;
    }
    sum
}

fn sum_array_reference(arr:& [i32;5]) -> i32 {
    let mut sum = 0;
    for element in arr {
        sum += element;
    }
    sum
}

fn reverse_array(arr:&mut [i32;5]) {
    for i in 0..2 {
        let tmp = arr[i];
        arr[i] = arr[4 - i];
        arr[4 - i] = tmp;
    }
}
