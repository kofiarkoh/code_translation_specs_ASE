fn f(n: i64, x: i64) -> i64 {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    }
    let p = (1 << n) - 1; // This computes 2^n - 1
    if x <= 2 * p {
        f(n - 1, x - 1)
    } else {
        p + 1 + f(n - 1, x - 1 - 2 * p)
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");

    // Preconditions check
    assert!(n >= 0, "n must be a non-negative integer");
    assert!(k >= 0, "k must be a non-negative integer");

    let result = f(n, k);
    // Postconditions check
    assert!(result >= 0, "Result must be a non-negative integer");

    println!("{}", result);
}
