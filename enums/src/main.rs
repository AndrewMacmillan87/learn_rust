// enum is short for 'enumeration' - the action of mentioning a number of things one by one.
// allows you to define a type by enumerating the values it can hold

#[derive(Debug)]
enum IPAddressType {
    V4,
    V6,
}

// Lets create an IPAddress type that uses the IPAddressType enum
#[derive(Debug)]
struct IPAddress {
    kind: IPAddressType,
    address: String
}

// This enum declaration allows us to create a similar construct to the enum/struct combo
// declaring the values to take params makes the code more concise
#[derive(Debug)]
enum IPAddressType2 {
    V4(String),
    V6(String),
}

fn main() {
    let ipv4 = IPAddressType::V4;
    let ipv6 = IPAddressType::V6;

    println!("\nipv4 = {:#?}", ipv4);
    println!("ipv6 = {:#?}", ipv6);

    // We can now declare IPAddresses using the appropriate types
    let loopback_v4 = IPAddress {
        kind: IPAddressType::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback_v6 = IPAddress {
        kind: IPAddressType::V6,
        address: String::from("::1"),
    };

    println!("\nloopback_v4 = {:#?}", loopback_v4);
    println!("\nloopback_v6 = {:#?}", loopback_v6);

    // However, we do not need the struct to create this construct
    // This does the same job - it has a string value and the correct enum type (V4 or V6)

    let loopback_v4 = IPAddressType2::V4(String::from("127.0.0.1"));
    let loopback_v6 = IPAddressType2::V6(String::from("::1"));

    println!("\nloopback_v4 = {:#?}", loopback_v4);
    println!("\nloopback_v6 = {:#?}", loopback_v6);

}
