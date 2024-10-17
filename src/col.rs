/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut steps = 1;
    loop {
        if n == 1 {
            return steps;
        } else if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

pub(crate) fn main() {
    println!("Length: {}", collatz_length(11));
}