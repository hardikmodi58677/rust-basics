// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

/* Function Parameters */

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }


// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

/* Function Bodies Contain Statements and Expressions */

// fn main() {
//     let y = 6;
// }

// This will throw an error because statements do not return values
// fn main() {
//     let x = (let y = 6);
// }


// fn main() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }


/* Functions with Return Values */
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }


// Example-2
fn main() {
    let x = plus_one(5);
    let x = plus_one(x);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

