
// Example of simple enum and struct with enum field

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}


// Example of enum with data

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


// Example of enum with variant that hold named fields
//      and enum impl

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn quit() -> Self {
        Self::Quit
    }

    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));


    let m = Message::Write(String::from("hello"));
    m.call();


    // Option examples:

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

