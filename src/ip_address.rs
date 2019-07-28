
// enum defintion
pub enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
    Random,
}

pub fn kind_of_ip(ip_kind: IpAddrKind){
    match ip_kind{
        IpAddrKind::V4(v1, v2, v3, v4) => println!("this is ip v4\nthe value: {}.{}.{}.{}",v1, v2, v3, v4),
        IpAddrKind::V6(value) => println!("thi is ip v6\nthe value: {}", value),
        IpAddrKind::Random => println!("this is random ip "),
    }
}