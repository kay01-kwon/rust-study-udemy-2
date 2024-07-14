use testing::*;

// 5. Create a `tests/` directory and an integration test file `tests/more_tests.rs`
// Inside that file, create a test function that verifies:
// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
//
// `cargo test` should run your `more_tests.rs` file and pass

// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started

#[test]
fn function_test(){
    let splish1 = splish(-1, 0);
    let splish2 = splish(1, 1);
    let splihs3 = splish(3, 2);

    let sploosh_result = sploosh(splish1, splish2, splihs3);

    assert_eq!(sploosh_result, 4);
}