struct User {
    username: String,
    age: u32,
    email: String,
}

impl User {
    fn say_hello(&self) {
        println!(
            "Hello, I'm {}, I'm {} years old, my email is {}.",
            self.username, self.age, self.email
        );
    }
}

fn main() {
    let someone = User {
        username: String::from("someone"),
        age: 35,
        email: String::from("someone@example.com"),
    };

    someone.say_hello();
    someone.say_hello();
}
