// Understding the module visibility rules

// mod outermost {
//     pub fn middle_function() {}

//     fn middle_secret_function() {}

//     mod inside {
//         pub fn inner_function() {}

//         fn secret_function() {}
//     }
// }

// fn try_me() {
//     outermost::middle_function();
//     outermost::middle_secret_function();
//     outermost::inside::inner_function();
//     outermost::inside::secret_function();
// }

// Overall, these are the rules for item visibility:

// If an item is public, it can be accessed through any of its parent modules.
// If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules.

// Referring to Names in Different Modules
