// Vectors

// #![allow(unused_variables)]
// fn main() {
//     let v: Vec<i32> = Vec::new();

//     // The macro will create a new vector that holds the values we give it

//     // let v = vec![1, 2, 3];
// }

// Updating a Vector

// #![allow(unused_variables)]
// fn main() {
//     let mut v = Vec::new();

//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
// }

// Dropping a Vector Drops Its Elements

// #![allow(unused_variables)]
// fn main() {
//     {
//         let v = vec![1, 2, 3, 4];

//         // do stuff with v
//     } // <- v goes out of scope and is freed here
// }

// Reading Elements of Vectors

// #![allow(unused_variables)]
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     let third: Option<&i32> = v.get(2);
// }

// #![allow(unused_variables)]
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     // use this method if accessing an element beyond the range of the vector happens occasionally under normal circumstances
//     let does_not_exist = v.get(100);

//     // This method is best used when you want your program to consider an attempt to access an element past the end of the vector to be a fatal error that crashes the program
//     let does_not_exist = &v[100];
// }

// Invalid References

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0];

//     v.push(6);

//     // print!("{}", first)
// }

// Iterating Over the Values in a Vector

// #![allow(unused_variables)]
// fn main() {
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }
// }

// #![allow(unused_variables)]
// fn main() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         println!("{}", i);
//         *i += 50;
//     }
//     print!("{:?}", v)
// }

// Using an Enum to Store Multiple Types

// #![allow(unused_variables)]
// fn main() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }

// Strings

// Creating a New String

// #![allow(unused_variables)]
// fn main() {
//     let mut s = String::new();
// }

// #![allow(unused_variables)]
// fn main() {
//     let data = "initial contents";

//     let s = data.to_string();

//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
// }

// use the function String::from to create a String from a string literal.

// #![allow(unused_variables)]
// fn main() {
//     let s = String::from("initial contents");
// }

// Updating a String
// Append a String Using push_str and push

// The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter
// #![allow(unused_variables)]
// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(&s2);
//     println!("s2 is {}", s2);
// }

// The push method takes a single character as a parameter and adds it to the String

// #![allow(unused_variables)]
// fn main() {
//     let mut s = String::from("lo");
//     s.push('l');
// }

// Concatenation with the + Operator or the format! Macro

// #![allow(unused_variables)]
// fn main() {
//     let s1 = String::from("Hello, ");
//     let mut s2 = String::from("world!");
//     let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
// Here s1 will be moved, and contents of s2 will be read and copied, and because now the scope of add is over, immutable borrow of s2 will be dropped
// So further changes to s2 will not make any difference to s3

//     s2.push('a');
//     println!("{}", s3);
// }

// The format macro is similar to println!, but instead of printing the output to the screen, it returns a String with the content
// #![allow(unused_variables)]
// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     // let s = s1 + "-" + &s2 + "-" + &s3;
//     let s = format!("{}-{}-{}", s1, s2, s3);
//     print!("{}", s)
// }

// Indexing into Strings
// fn main() {
//     let s1 = String::from("hello");
//     let h = s1[0];
// }

// Internal Representation

// #![allow(unused_variables)]
// fn main() {
//     let len = String::from("Hola").len();
// }

// #![allow(unused_variables)]
// fn main() {
// let len = String::from("Здравствуйте").len();

// let hello = "Здравствуйте";
// let answer = &hello[0];

// }

// Bytes and Scalar Values and Grapheme Clusters! Oh My!

// If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is ultimately stored as a Vec of u8 values that looks like this:

// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
// 224, 165, 135]

// If we look at them as Unicode scalar values, which are what Rust’s char type
// ['न', 'म', 'स', '्', 'त', 'े']

// if we look at them as grapheme clusters
// ["न", "म", "स्", "ते"]

// Slicing Strings

// #![allow(unused_variables)]
// fn main() {
//     let hello = "Здравствуйте";

//     let s = &hello[0..4];
// }

// Here, s will be a &str that contains the first four bytes of the string. Earlier, we mentioned that each of these characters was two bytes, which means s will be Зд.

// if we used &hello[0..1],
// Rust will panic at runtime in the same way that accessing an invalid index in a vector does:
// thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4

// Methods for Iterating Over Strings

// .chars() method returns each character

// #![allow(unused_variables)]
// fn main() {
//     for c in "नमस्ते".chars() {
//         println!("{}", c);
//     }
// }

// .bytes() method returns each raw byte

// #![allow(unused_variables)]
// fn main() {
//     for b in "नमस्ते".bytes() {
//         println!("{}", b);
//     }
// }

// Hash Maps

// Creating a New Hash Map

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// }

// Creating a hashmap using collect method

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let teams = vec![String::from("Blue"), String::from("Yellow")];
//     let initial_scores = vec![10, 50];
//     let final_scores = vec![50, 60];

//     let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
//     println!("{:?}", scores)
// }

// Hash Maps and Ownership

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//     // field_name and field_value are invalid at this point, try using them and
//     // see what compiler error you get!
//     // println!("{:?}", field_name)
//     print!("{:?}", map)
// }

// Accessing Values in a Hash Map

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);

//     println!("{:?}", score)
// }

// Iterating over each key/value pair in a hash map

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }

// Updating a Hash Map

// Overwriting a Value

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);

//     println!("{:?}", scores);
// }

// Only Inserting a Value If the Key Has No Value

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);

//     println!("{:?}", scores);
// }

// Updating a Value Based on the Old Value

// #![allow(unused_variables)]
// fn main() {
//     use std::collections::HashMap;

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();

//     for word in text.split(" ") {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }

// Hashing Functions
// you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait

// Exercise

// Given a list of integers, find mean, median and mode of the list

// fn main() {
//     use std::collections::HashMap;

//     let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

//     let mut sum = 0;
//     let mut mode = 0;

//     for i in &list {
//         sum += i;
//     }

//     let mean: f32 = sum as f32 / list.len() as f32;

//     list.sort();

//     let median = list[list.len() / 2];

//     let mut map = HashMap::new();

//     for i in &list {
//         let count = map.entry(i).or_insert(0);
//         *count += 1;
//     }

//     // // if all values are same, then panic, because there is no mode
//     // if total_count == list.len() {
//     //     panic!("No mode");
//     // }

//     for (key, value) in &map {
//         println!("{}: {}", key, value);
//         if value > &mode {
//             mode = **key;
//         }
//     }

//     println!("Mean: {}", mean);
//     println!("Median: {}", median);
//     println!("Mode: {}", mode);
// }

//Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

// fn main() {
//     let mut str = String::from("an apple a day keeps the doctor away");

//     let new_string = convert_to_pig_latin(&mut str);
//     println!("{}", new_string);

//     fn convert_to_pig_latin(s: &mut str) -> String {
//         let split_string = s.split(" ");

//         let mut new_string = String::new();

//         for word in split_string {
//             let first_char = word.chars().next().unwrap();
//             let mut new_word = word.to_string();

//             if is_vowel(first_char.to_ascii_uppercase()) {
//                 new_word.push_str("-hay ");
//             } else {
//                 let first_char = new_word.remove(0);
//                 new_word.push_str(format!("-{}ay ", first_char).as_str());
//             };
//             new_string.push_str(&new_word);

//             fn is_vowel(c: char) -> bool {
//                 match c {
//                     'A' | 'E' | 'I' | 'O' | 'U' => true,
//                     _ => false,
//                 }
//             }
//         }
//         new_string
//     }
// }

//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn main() {
    use std::collections::HashMap;

    #[derive(Eq, Hash, PartialEq, Debug, Clone)]
    #[allow(dead_code)]
    enum Department {
        Engineering,
        Sales,
        Marketing,
        Finance,
    }

    struct Company {
        employees: HashMap<Department, Vec<String>>,
    }

    impl Company {
        fn add_employee(&mut self, name: &str, department: Department) -> () {
            match department {
                Department::Engineering => {
                    self.employees
                        .entry(Department::Engineering)
                        .or_insert(vec![name.to_string()]);
                }
                Department::Sales => {
                    self.employees
                        .entry(Department::Sales)
                        .or_insert(vec![name.to_string()]);
                }
                Department::Marketing => {
                    self.employees
                        .entry(Department::Marketing)
                        .or_insert(vec![name.to_string()]);
                }
                Department::Finance => {
                    self.employees
                        .entry(Department::Finance)
                        .or_insert(vec![name.to_string()]);
                }
            }
            // Sort the vector
            self.employees
                .get_mut(&department)
                .unwrap()
                .sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        }
        fn retrieve_employees(
            &self,
            department: Option<Department>,
        ) -> HashMap<Department, Vec<String>> {
            // if department is passed, return a hashmap of department:employees
            if department.is_some() {
                let mut employees_map: HashMap<Department, Vec<String>> = HashMap::new();
                let new_department = department.unwrap();
                let employees = self.employees.get(&new_department).unwrap();
                employees_map.insert(new_department, employees.clone());
                employees_map
            } else {
                self.employees.clone()
            }
        }
    }

    let company_employees: HashMap<Department, Vec<String>> = HashMap::new();

    let mut company = Company {
        employees: company_employees,
    };
    company.add_employee("Samantha", Department::Engineering);
    company.add_employee("Amir", Department::Sales);
    let all_employees = company.retrieve_employees(None);
    let eng_employees = company.retrieve_employees(Some(Department::Engineering));

    println!("{:?}", all_employees);
    println!("{:?}", eng_employees);
}
