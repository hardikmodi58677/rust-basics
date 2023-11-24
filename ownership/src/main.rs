// Ownership

// Variable Scope

// fn main() {
// {                      // s is not valid here, it’s not yet declared
//     let s = "hello";   // s is valid from this point forward

//     // do stuff with s
// }                      // this scope is now over, and s is no longer valid
// }

// String type, and type of string, that can be mutated

// fn main() {
// let mut s = String::from("hello");

// s.push_str(", world!"); // push_str() appends a literal to a String

// println!("{}", s); // This will print `hello, world!`
// }

// Memory and Allocation

// fn main() {
// {
//     let s = String::from("hello"); // s is valid from this point forward

//     // do stuff with s
// }                                  // this scope is now over, and s is no
//                                    // longer valid
// }

// Ways Variables and Data Interact: Move

// this wont compile, because s1 is no longer valid

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);

// }

// Ways Variables and Data Interact: Clone

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);

// }

// Stack-Only Data: Copy

// fn main() {
// let x = 5;
// let y = x;

// println!("x = {}, y = {}", x, y);
// }

// Ownership and Functions
// fn main() {
//     let s = String::from("hello");  // s comes into scope.

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here.

//     let x = 5;                      // x comes into scope.

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it’s okay to still
//                                     // use x afterward.

// } // Here, x goes out of scope, then s. But since s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope.
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope.
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// Return Values and Scope
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1.

//     let s2 = String::from("hello");     // s2 comes into scope.

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3.
// } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
//   // moved, so nothing happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it.

//     let some_string = String::from("hello"); // some_string comes into scope.

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function.
// }

// // takes_and_gives_back will take a String and return one.
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope.

//     a_string  // a_string is returned and moves out to the calling function.
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String.

//     (s, length)
// }

// References and Borrowing
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Attempting to modify a borrowed value
// this will fail to compile
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// Mutable References

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// Mutable references restrictions

// this wont compile, because we can only have one mutable reference to a particular piece of data in a particular scope

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// We can use curly braces to create a new scope, allowing for multiple mutable references, just not simultaneous ones
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;
// }

// Combining mutable and immutable references
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);
// }

// Dangling references

// This wont compile, because reference is pointing to a value that is out of scope at the end of the function
// fn main() {
//     let reference_to_nothing = dangle();
//     print!("{}", reference_to_nothing);
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// Instead we can return the String directly

// fn main() {
//     let reference_to_nothing = no_dangle();
//     print!("{}", reference_to_nothing);
// }
// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }

// Slices

// Here we are only returning a usize, which is not correct because, it is just an index, and not the reference to the actual value
// We might face issues if the value of the string changes, and then index will no longer be of any use

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5.

//     s.clear(); // This empties the String, making it equal to "".

//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }

// String Slices

// #![allow(unused_variables)]
// fn main() {
// let s = String::from("hello world");

// let hello = &s[0..5];
// let world = &s[6..11];
// }

// Implementing first_word using slices

// #![allow(unused_variables)]
// fn main() {
//     let my_string = String::from("hello world");
//     let word = first_word(&my_string);
//     // let word = first_word(&my_string[..]);
//     print!("{}", word);
// }

// fn first_word(s: &String) -> &str {
//     // fn first_word(s: &str) -> &str {  // this will also work because we can pass a string slice as an argument
//         let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// Other slices

#![allow(unused_variables)]
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    print!("{}", slice[0]);
}
