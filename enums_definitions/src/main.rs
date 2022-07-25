enum IpAddrKind{
    V4,
    V6,
}
// you can specify types
enum IpAddrKindWithFormat{
    V4(u8, u8, u8, u8),
    V6(String),
}


struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let absent_number: Option<isize> = None;
    let num: Option<isize> = Some(5);   
    println!("There are enums defined here. Code compiles with warnings since none are used.");
}
