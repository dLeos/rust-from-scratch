#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle is: {:?}", rect1);
    println!("Rectangle is: {:#?}", rect1);
    dbg!(&rect1);
    println!("The area of the rectangle is {} square pixels.", area(&rect1));

    let perimeter = dbg!(2 * rect1.width) + 2 * rect1.height;
    println!("The perimeter of the rectangle is {}.", perimeter);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
