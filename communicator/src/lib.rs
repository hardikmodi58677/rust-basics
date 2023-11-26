// Module Definitions

// pub mod client;
// pub mod network;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

/*

Privacy Rules
Overall, these are the rules for item visibility:

If an item is public, it can be accessed through any of its parent modules.
If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules.

*/

pub mod client;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
