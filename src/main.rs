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

    match std::env::home_dir() {
        Some(data) => println!("option is some, data = {:?}", data),
        None => println!("option is none"),
    }

    match std::env::var("LANG") {
        Ok(data) => println!("ok! {:?}", data),
        Err(err) => println!("err {}", err),
    }
}
