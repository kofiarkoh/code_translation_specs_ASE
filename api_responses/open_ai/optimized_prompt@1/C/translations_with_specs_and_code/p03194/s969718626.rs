fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();

    let n: i64 = inputs.next().expect("Missing first input").parse().expect("Invalid first input");
    let mut p: i64 = inputs.next().expect("Missing second input").parse().expect("Invalid second input");

    // Preconditions check
    if n < 0 || p <= 0 {
        // Behavior is not defined for invalid inputs as per the specification
        return;
    }

    let mut ans = 1;

    // Prime factorization
    let mut i = 2;
    while i * i <= p {
        let mut cnt = 0;
        while p % i == 0 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            // Multiply ans by i raised to the power of cnt/n
            ans *= i.pow((cnt / n) as u32);
        }
        i += 1;
    }

    // If p is still greater than 1 and n is 1, multiply ans by p
    if p > 1 && n == 1 {
        ans *= p;
    }

    // Output the result
    println!("{}", ans);
}
