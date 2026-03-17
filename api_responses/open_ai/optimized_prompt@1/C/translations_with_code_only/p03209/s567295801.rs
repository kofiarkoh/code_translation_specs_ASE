// Define a type alias for long long
type LL = i64;

// Recursive function f as defined in the C code
fn f(n: LL, x: LL) -> LL {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    }
    let p = (1 << n) - 1; // Equivalent to (1LL << n) - 1 in C
    if x <= 2 * p {
        return f(n - 1, x - 1);
    } else {
        return p + 1 + f(n - 1, x - 1 - 2 * p);
    }
}

fn main() {
    // Read input values for n and k
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: LL = iter.next().unwrap().parse().unwrap();
    let k: LL = iter.next().unwrap().parse().unwrap();

    // Print the result of the function f
    println!("{}", f(n, k));
}
