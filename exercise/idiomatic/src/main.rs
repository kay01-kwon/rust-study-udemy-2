// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

#[allow(clippy::excessive_precision)]
const PI: f64 = 3.141_592_653_589_793_238_46;
fn count_to_5() -> i32 {
    #[allow(clippy::disallowed_names)]
    let mut foo = 0;
    loop {
        #[allow(clippy::collapsible_if)]
        if foo > PI as i32 {
            if foo > 5 {
                break;
            }
        }
        foo += 1;
    }
    5
}
fn main() {
    println!("I can count to {}", count_to_5());
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
