// Example 1
// Ip addresses can ben only two kinds, V4 or V6.
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Storing the data using enum inside a struct
struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

// Example 2
#[derive(Debug)]
enum IpAddr {
    V4(String), // IpAddr::V4() is a function call that takes a String argument
    V6(String),
}

// Example 3
#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Example 1
    let home1 = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback1 = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home1 address: {:?} {}", home1.kind, home1.address);
    println!("loopback1: {:?} {}", loopback1.kind, loopback1.address);

    // Example 2
    let home2 = IpAddr::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr::V6(String::from("::1"));
    
    println!("home2 address: {:?}", home2);
    println!("loopback2: {:?}", loopback2);

    // Example3
    let home3 = IpAddressKind::V4(127, 0, 0, 1);
    let loopback3 = IpAddressKind::V6(String::from("::1"));

    println!("home3 address: {:?}", home3);
    println!("loopback3: {:?}", loopback3);
}
