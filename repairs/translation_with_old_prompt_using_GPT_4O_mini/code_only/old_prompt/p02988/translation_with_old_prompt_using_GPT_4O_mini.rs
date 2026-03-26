use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Failed to parse input");

    let mut num = vec![0; 1002]; // Create a vector with 1002 elements initialized to 0 to avoid out-of-bounds access
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        // Ensure we handle potential parsing errors gracefully
        num[i] = input.trim().parse().expect("Failed to parse number");
    }

    let mut count = 0;
    for i in 1..(n - 1) {
        // Check bounds to prevent accessing out of range
        if i + 2 <= n {
            if num[i] < num[i + 1] {
                if num[i + 1] < num[i + 2] {
                    count += 1;
                }
            } else if num[i] > num[i + 1] {
                if num[i + 1] > num[i + 2] {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
