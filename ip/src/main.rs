use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
fn main() {
    let localhost_v4 = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));
    println!("{:?}", localhost_v4.is_multicast());
}
