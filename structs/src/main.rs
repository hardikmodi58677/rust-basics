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

// These are called unit-like structs since they behave similarly to (), the unit type. Unit-like structs can be useful in situations such as when you need to implement a trait on some type, but you don’t have any data that you want to store in the type itself.

// An example program using structs

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Refactoring with tuples

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Refactoring with structs : adding more meaning

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Adding useful functionality with derived traits

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!("rect1 is {:?}", rect1);
// }

//we can use {:#?} instead of {:?} in the println! string. When we use the {:#?} style in the example, the output will look like this:

// rect1 is Rectangle {
//     width: 30,
//     height: 50
// }

// Method Syntax : Defining Methods

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// Where is the -> operator?

// Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing

// #![allow(unused_variables)]
// fn main() {
// #[derive(Debug,Copy,Clone)]
// struct Point {
//     x: f64,
//     y: f64,
// }

// impl Point {
//    fn distance(&self, other: &Point) -> f64 {
//        let x_squared = f64::powi(other.x - self.x, 2);
//        let y_squared = f64::powi(other.y - self.y, 2);

//        f64::sqrt(x_squared + y_squared)
//    }
// }
// let p1 = Point { x: 0.0, y: 0.0 };
// let p2 = Point { x: 5.0, y: 6.5 };

// // Following are the same
// p1.distance(&p2);
// (&p1).distance(&p2);
// }

// Methods with More Parameters

// #![allow(unused_variables)]
// fn main() {
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// let rect1 = Rectangle { width: 30, height: 50 };
//     let rect2 = Rectangle { width: 10, height: 40 };
//     let rect3 = Rectangle { width: 60, height: 45 };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// Associated Functions

#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
}
