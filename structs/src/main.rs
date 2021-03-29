#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width && self.height >= other.height) || (self.width >= other.height && self.height >= other.width)
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}
struct IpAdd {
    V4(String),
    V6(String),
}
struct IpFlex {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn enums() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    let work = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };
    let home2 = IpAdd::V4(String::from("127.0.0.1"));
    let loopback = IpAdd::V6(String::from("::1"));

    let home3 = IpFlex::V4(127, 0, 0, 1);
    let loopback2 = IpFlex::V6(String::from("::1"));
}
fn main() {
 
    basics();
    basics_2();
    enums();
}

fn basics_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect: {:?}", rect1);
    println!("The area of the rectangle is: {}", rect1.area());
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn basics() {
     let mut user1 = User {
        email: String::from("1231@gmail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("changeto@live.com");

    println!("user1 email: {}", user1.email);
 
    let user2 = create_user(String::from("user2@gmail.com"), String::from("user2"));

    println!("user2 email: {}", user2.email);

    let user3 = User {
        email: String::from("user3@gmail.com"),
        username: String::from("user3"),
        ..user2
    };
    println!("user3 email: {} user3 active: {}", user3.email, user3.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Color(r, g, b) = black;
    println!("black rgb: {}{}{} ", black.0, g, black.2);
      
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
