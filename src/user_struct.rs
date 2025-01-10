struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User {
    fn new(username: String, email: String, sign_in_count: u64, active: bool) -> User {
        User {
            username,
            email,
            sign_in_count,
            active,
        }
    }
    fn deactive(&mut self) {
        self.active = false;
    }
}
struct mypoint(i32, i32);

fn main() {
    let usernamecc = String::from("zain");
    let email = String::from("zaingmail.com");
    let sign_in_count = 1;
    let active = true;
    let user1 = User {
        usernamecc,
        email,
        sign_in_count,
        active,
    };
    let user1 = User::new(
        String::from("  zain"),
        String::from("zain@gmail.com"),
        1,
        true,
    );
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.active);
    println!("Sign-in count: {}", user1);
    deactive(&mut user1);
    println!("Active: {}", user1.active);
}
