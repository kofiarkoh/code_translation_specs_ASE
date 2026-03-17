use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: usize = input.trim().parse().expect("Invalid input for n");
    
    // Ensure n is within the valid range
    if n < 3 {
        println!("0");
        return;
    }

    let mut num = Vec::with_capacity(n);
    
    // Read n integers
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    num.extend(input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid integer")));

    let mut count = 0;

    // Count valid triplets
    for i in 0..(n - 2) {
        if num[i] < num[i + 1] && num[i + 1] < num[i + 2] {
            count += 1;
        } else if num[i] > num[i + 1] && num[i + 1] > num[i + 2] {
            count += 1;
        }
    }

    println!("{}", count);
}
