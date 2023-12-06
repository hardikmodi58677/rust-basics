extern crate testing_rust;
mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, testing_rust::add_two(2));
}

// ----------------------------------------------

// Can run all tests in a test file using
// $ cargo test --test integration_test

// Submodules in Integration Tests

// Integration tests for binary crates
