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
}
