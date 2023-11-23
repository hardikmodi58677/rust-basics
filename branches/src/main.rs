// Control Flow

// If Expressions
// Blocks of code associated with the conditions in if expressions are sometimes called arms,

// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// Condition must be a bool, this will not work

// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

// Instead, we can do this
// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// Multiple conditions with else if

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// Using if in a let statement

// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         6
//     };

//     println!("The value of number is: {}", number);
// }

// Types in each arm must be the same, else we get compile error

// fn main() {
//     let condition = true;

//     let number = if condition {
//         5
//     } else {
//         "six"
//     };

//     println!("The value of number is: {}", number);
// }