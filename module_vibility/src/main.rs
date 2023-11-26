// Referring to Names in Different Modules

// pub mod a {
//     pub mod series {
//         pub mod of {
//             pub fn nested_modules() {}
//         }
//     }
// }

// fn main() {
//     a::series::of::nested_modules();
// }

// Bringing Names into Scope with the use Keyword

// pub mod a {
//     pub mod series {
//         pub mod of {
//             pub fn nested_modules() {
//                 println!("called a::series::of::nested_modules()");
//             }
//         }
//     }
// }

// use a::series::of;

// fn main() {
//     of::nested_modules();
// }

// We could have chosen to bring the function into scope by instead specifying the function in the use as follows:

// pub mod a {
//     pub mod series {
//         pub mod of {
//             pub fn nested_modules() {
//                 println!("called a::series::of::nested_modules()");
//             }
//         }
//     }
// }

// use a::series::of::nested_modules;

// fn main() {
//     nested_modules();
// }

// Use with enums

// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// use TrafficLight::{Red, Yellow};

// fn main() {
//     let red = Red;
//     let yellow = Yellow;
//     let green = TrafficLight::Green;
// }

// Bringing All Names into Scope with a Glob

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
