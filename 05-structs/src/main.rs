struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    // rectangle eg

    let rect = Rectangle{
        width: 30,
        height: 50,
    };

    println!("{:#?}",rect);

    println!("Area of the rectangle: {}", rect.area());



    // users eg

    let mut user1 = User{
        username: String::from("dotslashlog1c"),
        email: String::from("log1c@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("dotslashapaar");
    let user2 = create_user(String::from("abc123"), String::from("abc@xyz.com"));
}

fn create_user(username: String, email: String) -> User{
    User { 
        username: username,
        email: email,
        sign_in_count: 1, 
        active: true 
    }
}

