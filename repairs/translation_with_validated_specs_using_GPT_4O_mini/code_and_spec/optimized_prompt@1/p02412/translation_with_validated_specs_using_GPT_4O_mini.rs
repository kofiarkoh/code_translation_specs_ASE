use std::io;

fn main() {
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let x: i32 = iter.next().unwrap().parse().unwrap();

        if n == 0 && x == 0 {
            break;
        }

        let mut count = 0;

        // Iterate over all combinations of three distinct integers
        for k in 1..=n {
            for j in 1..=n {
                for i in 1..=n {
                    // Ensure i, j, k are distinct
                    if i != j && j != k && i != k {
                        if i + j + k == x {
                            count += 1;
                        }
                    }
                }
            }
        }

        // Output the count divided by 6
        println!("{}", count / 6);
    }
}
