use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Ensure n is at least 3 to form triplets
    if n < 3 {
        println!("0");
        return;
    }

    let mut num = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    num.extend(input.trim().split_whitespace().map(|s| s.parse::<i32>().expect("Invalid integer")));

    let mut count = 0;

    // Iterate through the array to count valid triplets
    for i in 0..(n - 2) {
        if num[i] < num[i + 1] && num[i + 1] < num[i + 2] {
            count += 1;
        } else if num[i] > num[i + 1] && num[i + 1] > num[i + 2] {
            count += 1;
        }
    }

    println!("{}", count);
}
