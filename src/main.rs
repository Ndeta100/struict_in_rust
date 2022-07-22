mod rectangles;
fn main() {
    println!("Hello, world!");
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let mut user1 = User {
        email: String::from("exap@gmail.com"),
        username: String::from("Ndeta100"),
        active: true,
        sign_in_count: 1,
    };
    println!("user's email is {}", user1.email);
    fn build_email(email: String, username: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
    rectangles::area(34, 23);
}

fn tupple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
