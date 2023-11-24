// Structs

// fn main() {
//     struct User {
//         username: String,
//         email: String,
//         sign_in_count: u64,
//         active: bool,
//     }

//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         ..user1 // Struct update syntax
//         // This wont overwrite the email and username from user1
//     };

//     print!("{} {} {} {}", user2.email, user2.username, user2.active, user2.sign_in_count);

//     user1.email = String::from("newEmail@gmail.com");
//     // Rust doesn’t allow us to mark only certain fields as mutable
//     print!("\n {} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);

// }

// Using the Field Init Shorthand when Variables and Fields Have the Same Name

// fn main() {
//     struct User {
//         username: String,
//         email: String,
//         sign_in_count: u64,
//         active: bool,
//     }

//     fn build_user(email: String, username: String) -> User {
//         User {
//             email,
//             username,
//             active: true,
//             sign_in_count: 1,
//         }
//     }
// }

// Creating Instances From Other Instances With Struct Update Syntax

// #![allow(unused_variables)]
// fn main() {
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// let user1 = User {
//     email: String::from("someone@example.com"),
//     username: String::from("someusername123"),
//     active: true,
//     sign_in_count: 1,
// };

// let user2 = User {
//     email: String::from("another@example.com"),
//     username: String::from("anotherusername567"),
//     active: user1.active,
//     sign_in_count: user1.sign_in_count,
// };
// }

// Different Syntax

// #![allow(unused_variables)]
// fn main() {
//     struct User {
//         username: String,
//         email: String,
//         sign_in_count: u64,
//         active: bool,
//     }

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         ..user1
//     };
// }

// Tuple Structs without Named Fields to Create Different Types

// #![allow(unused_variables)]
// fn main() {
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// let black = Color(0, 0, 0);
// let origin = Point(0, 0, 0);
// }

// Unit-Like Structs without Any Fields

#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    enum Message {
        Write(String),
    }

    impl Message {
        fn call(&self) {
            println!("Value of self: {:?}", self)
        }
    }
    
    let m = Message::Write(String::from("hey there"));
    m.call();
}
