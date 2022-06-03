fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
/* enum IpAddrKind {
    V4,
    V6,
}
   let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
fn route(ip_kind: IpAddrKind) {}

Value inside enum?
   enum IpAddr {
        V4(String),
        V6(String),
    }
 */
/* enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1")); */

