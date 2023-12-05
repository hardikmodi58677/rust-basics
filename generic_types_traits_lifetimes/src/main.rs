// fn main() {
//     println!("Hello, world!");
// }

// Using generic data type in function definition

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
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

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//    assert_eq!(result, 100);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
//    assert_eq!(result, 'y');
// }

// ---------------------------------------------------------------

// fn largest<T>(list: &[T]) -> T {
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

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// Using generic types in struct definition

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// ---------------------------------------------------------------
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }

// ---------------------------------------------------------------

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }

// Using generic types in enum definition

// #![allow(unused_variables)]
// fn main() {
// enum Option<T> {
//     Some(T),
//     None,
// }
// }

// ---------------------------------------------------------------

// #![allow(unused_variables)]
// fn main() {
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// }

// Using generic types in method definition

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

// ---------------------------------------------------------------

// we could choose to implement methods on Point<f32> instances rather than Point instances with any generic type.

// #![allow(unused_variables)]
// fn main() {
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// }

// ---------------------------------------------------------------

// Mixing generic types in struct definition

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// Performance of code using generics

// #![allow(unused_variables)]
// fn main() {
// let integer = Some(5);
// let float = Some(5.0);
// }

// // ---------------------------------------------------------------
// enum Option_i32 {
//     Some(i32),
//     None,
// }

// enum Option_f64 {
//     Some(f64),
//     None,
// }

// fn main() {
//     let integer = Option_i32::Some(5);
//     let float = Option_f64::Some(5.0);
// }

// Traits: Defining Shared Behavior - similar to interfaces in other languages

// Defining a Trait

// #![allow(unused_variables)]
// fn main() {
//     pub trait Summarizable {
//         fn summary(&self) -> String;
//     }
// }

// Implementing a Trait on a Type

// Filename : lib.rs

// #![allow(unused_variables)]
// fn main() {
// pub trait Summarizable {
//     fn summary(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summarizable for NewsArticle {
//     fn summary(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summarizable for Tweet {
//     fn summary(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }
// }

// One restriction to note with trait implementations: we may implement a trait on a type as long as either the trait or the type are local to our crate
// In other words, we aren’t allowed to implement external traits on external types.

// Default Implementations

// #![allow(unused_variables)]
// fn main() {
//     pub trait Summarizable {
//         fn summary(&self) -> String {
//             String::from("(Read more...)")
//         }
//     }
// }

// impl Summarizable for NewsArticle {}

// let article = NewsArticle {
//     headline: String::from("Penguins win the Stanley Cup Championship!"),
//     location: String::from("Pittsburgh, PA, USA"),
//     author: String::from("Iceburgh"),
//     content: String::from("The Pittsburgh Penguins once again are the best
//     hockey team in the NHL."),
// };

// println!("New article available! {}", article.summary());

// ---------------------------------------------------------------

// #![allow(unused_variables)]
// fn main() {
//     pub struct Tweet {
//         pub username: String,
//         pub content: String,
//         pub reply: bool,
//         pub retweet: bool,
//     }

//     pub trait Summarizable {
//         fn author_summary(&self) -> String;

//         fn summary(&self) -> String {
//             format!("(Read more from {}...)", self.author_summary())
//         }
//     }

//     impl Summarizable for Tweet {
//         fn author_summary(&self) -> String {
//             format!("@{}", self.username)
//         }
//     }

//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summary());
// }

// Trait bounds

// Specifying trait bounds with generic data types

// pub fn notify<T: Summarizable>(item: T) {
//     println!("Breaking news! {}", item.summary());
// }

// // For functions that have multiple generic type parameters, each generic has its own trait bounds.
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//     0
// }

// ---------------------------------------------------------------

// We can write the same function using where clauses, like this:

// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {
//     0
// }

// Fixing the largest Function with Trait Bounds

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }
//---------------------------------------------------------------

// Implementing with clone trait bound

// fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
//         let mut largest = list[0].clone();

//         for item in list.iter() {
//             let cloned_item = item.clone();
//             if cloned_item > largest {
//                 largest = cloned_item
//             }
//         }

//         largest

//     }

//     fn main() {
//         let number_list = vec![34, 50, 25, 100, 65];

//         let result = largest(&number_list);
//         println!("The largest number is {}", result);

//         let char_list = vec!['y', 'm', 'a', 'q'];

//         let result = largest(&char_list);
//         println!("The largest char is {}", result);
//     }

// ---------------------------------------------------------------

// Implementing with return type as a reference, to remove the Copy trait bound

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//         let mut largest = &list[0];

//         for item in list.iter() {
//             if item > largest {
//                 largest = item;
//             }
//         }

//         largest

//     }

//     fn main() {
//         let number_list = vec![34, 50, 25, 100, 65];

//         let result = largest(&number_list);
//         println!("The largest number is {}", result);

//         let char_list = vec!['y', 'm', 'a', 'q'];

//         let result = largest(&char_list);
//         println!("The largest char is {}", result);
//     }

// ---------------------------------------------------------------

// Using Trait Bounds to Conditionally Implement Methods

// #![allow(unused_variables)]
// fn main() {
// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }
// }

// Validating references with lifetimes
// fn main(){
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }

// The borrow checker

// #![allow(unused_variables)]
// fn main() {
// {
//     let x = 5;            // -----+-- 'b
//                           //      |
//     let r = &x;           // --+--+-- 'a
//                           //   |  |
//     println!("r: {}", r); //   |  |
//                           // --+  |
// }                         // -----+
// }

// Generic Lifetimes in Functions

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime annotations

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Lifetime Annotations in Function Signatures

// #![allow(unused_variables)]
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);

//     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//         if x.len() > y.len() {
//             x
//         } else {
//             y
//         }
//     }
// }

// Lifetime annotations in struct definitions

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.')
//         .next()
//         .expect("Could not find a '.'");
//     let i = ImportantExcerpt { part: first_sentence };

//     println!("{}", i.part);
// }

// Lifetime Elision

// #![allow(unused_variables)]
// fn main() {
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }
// }

// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes

// Rules
// 1. Each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32), a function with two arguments gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), and so on.

// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, then the lifetime of self is assigned to all output lifetime parameters. This makes writing methods much nicer.

// Lifetime Annotations in Method Definitions

// Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, since those lifetimes are part of the struct’s type.

// The lifetime parameter declaration after impl and use after the type name is required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
// #![allow(unused_variables)]
// fn main() {
//     struct ImportantExcerpt<'a> {
//         part: &'a str,
//     }

//     impl<'a> ImportantExcerpt<'a> {
//         fn level(&self) -> i32 {
//             3
//         }
//     }
// }

// ---------------------------------------------------------------

// #![allow(unused_variables)]
// fn main() {
//     struct ImportantExcerpt<'a> {
//         part: &'a str,
//     }

//     impl<'a> ImportantExcerpt<'a> {
//         fn announce_and_return_part(&self, announcement: &str) -> &str {
//             println!("Attention please: {}", announcement);
//             self.part
//         }
//     }
// }

// ---------------------------------------------------------------

// The Static Lifetime

// #![allow(unused_variables)]
// fn main() {
//     let s: &'static str = "I have a static lifetime.";
// }
// Most of the time, the problem in the code is an attempt to create a dangling reference or a mismatch of the available lifetimes, and the solution is fixing those problems, not specifying the 'static lifetime.

// Generic Type Parameters, Trait Bounds, and Lifetimes Together

#![allow(unused_variables)]
fn main() {
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
