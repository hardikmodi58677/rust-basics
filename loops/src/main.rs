// Code repeatation with loops

// fn main() {
//     let mut count : i32 = 0;
//     loop {
//         count = count + 1;
//         println!("Printed {} times",count);

//         if count == 10 {
//             break;
//         }

//     }
// }


// Conditional loops with while
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number = number - 1;
//     }

//     println!("LIFTOFF!!!");
// }

// Looping through a collection with for
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < a.len() {
//         println!("the value is: {}", a[index]);

//         index = index + 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }


// Using rev() to reverse the order of the loop

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
