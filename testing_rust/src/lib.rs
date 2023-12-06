// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

// Checking Results with the assert! Macro

// #![allow(unused_variables)]
// #![allow(dead_code)]

// #[derive(Debug)]
// pub struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     pub fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length < other.length && self.width < other.width
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             length: 8,
//             width: 7,
//         };
//         let smaller = Rectangle {
//             length: 5,
//             width: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle { length: 8, width: 7 };
//         let smaller = Rectangle { length: 5, width: 1 };

//         assert!(!smaller.can_hold(&larger));
//     }
// }

// fn main() {
//     // Code inside the main function
// }

// Testing Equality with the assert_eq! and assert_ne! Macros

// #![allow(unused_variables)]

// pub fn add_two(a: i32) -> i32 {
//     a + 3
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }

// // fn main() {}

// Adding Custom Failure Messages

// #![allow(unused_variables)]
// pub fn greeting(name: &str) -> String {
//     // format!("Hello {}!", name)
//     format!("Hello!")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was `{}`",
//             result
//         );
//     }
// }

// Checking for Panics with should_panic

// #![allow(unused_variables)]
// #![allow(dead_code)]
// pub struct Guess {
//     value: u32,
// }

// impl Guess {
//     pub fn new(value: u32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be greater than 1, got {}.", value);
//         }
//         else if value > 100 {
//             panic!("Guess value must be less than or equal to 100, got {}.", value);
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "Guess value must be less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(250);
//     }
// }

// Controlling How Tests Are Run

// Running cargo test --help displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator --

// Running Tests in Parallel or Consecutively

// To make the tests not run in parallel, use the --test-threads flag and set the value to 1
// $ cargo test -- --test-threads=1

// Showing Function Output

// pub fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }

// // we can disable output capture, by
// // $ cargo test -- --nocapture

// Running a Subset of Tests by Name

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn add_two_and_two() {
//         assert_eq!(4, add_two(2));
//     }

//     #[test]
//     fn add_three_and_two() {
//         assert_eq!(5, add_two(3));
//     }

//     #[test]
//     fn one_hundred() {
//         assert_eq!(102, add_two(100));
//     }
// }

// Running Single Tests
// ❯ cargo test one_hundred
//     Finished test [unoptimized + debuginfo] target(s) in 0.00s
//      Running unittests src/lib.rs (target/debug/deps/testing_rust-be1efe197d3c56a1)

// running 1 test
// test tests::one_hundred ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

// Filtering to Run Multiple Tests
// cargo test add

// Ignoring Some Tests Unless Specifically Requested
// annotate the test function with the ignore attribute

// #![allow(unused_variables)]
// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }

// #[test]
// #[ignore]
// fn expensive_test() {
//     // code that takes an hour to run
// }

// To run the ignored tests, use the --ignored flag
// cargo test -- --ignored

// Test organization

// Unit Tests
// The Tests Module and #[cfg(test)]

// Testing Private Functions

#![allow(unused_variables)]
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}


// Integration Tests
