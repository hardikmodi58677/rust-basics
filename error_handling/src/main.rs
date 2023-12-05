// If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting on panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:
// [profile.release]
// panic = 'abort'

// fn main() {
//     panic!("crash and burn");
// }

// Using a panic! Backtrace

// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }

// Recoverable Errors with Result

// #![allow(unused_variables)]
// fn main() {
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// }

// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("There was a problem opening the file: {:?}", error)
//         }
//     };
// }

// Matching on Different Errors
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => {
//                 panic!("Tried to create file but there was a problem: {:?}", e)
//             }
//         },
//         Err(error) => {
//             panic!("There was a problem opening the file: {:?}", error)
//         }
//     };
// }

// Shortcuts for Panic on Error: unwrap and expect
// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").unwrap();
// }

// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// Propagating Errors

// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let f = File::open("hello.txt");

//         let mut f = match f {
//             Ok(file) => file,
//             Err(e) => {
//                 println!("{:?}", e);
//                 return Err(e);
//             }
//         };

//         let mut s = String::new();

//         match f.read_to_string(&mut s) {
//             Ok(_) => Ok(s),
//             Err(e) => Err(e),
//         }
//     }

//     let result = read_username_from_file();
//     println!("{:?}", result);
// }

// Shortcuts for Propagating Errors: the ? Operator

// #![allow(unused_variables)]
// fn main() {
// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
// }

// We could shorten this code even further by chaining method calls immediately after the ?, as shown in Listing 9-8.

// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut s = String::new();

//         File::open("hello.txt")?.read_to_string(&mut s)?;

//         Ok(s)
//     }

//     let result = read_username_from_file()
//         .map_err(|e| println!("{:?}", e))
//         .unwrap();
//     println!("{:?}", result);
// }

// (?) can only be used in functions that have a return type of Result
// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt")?;
// }

// To panic! or not to panic!
// returning Result is a good default choice when you’re defining a function that might fail.

// Examples, Prototype Code, and Tests Are All Places it’s Perfectly Fine to Panic

// Cases when you have more information than the compiler

// #![allow(unused_variables)]
// fn main() {
//     use std::net::IpAddr;

//     let home: IpAddr = "127.0.0.1".parse().unwrap();
// }

// Guidelines for Error Handling

// Creating custom types for validation

// fn main(){
//     loop {
//         // --snip--

//         let guess: i32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         if guess < 1 || guess > 100 {
//             println!("The secret number will be between 1 and 100.");
//             continue;
//         }

//         match guess.cmp(&secret_number) {
//         // --snip--
//     }

// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// Extracting into a function

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(result, 100);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(result, 6000);
// }

// Generic Data Types
