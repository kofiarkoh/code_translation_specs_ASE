use std::io;

fn num(n: usize, yaku: &[usize], a: usize) -> usize {
    let mut count = 0;
    for i in 2..=n {
        if yaku[i] >= a - 1 {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a valid integer");

    // Preconditions check
    if n < 2 {
        panic!("Input must be at least 2");
    }

    let mut yaku = vec![0; 102]; // Array to store counts of prime factors

    for i in 2..=n {
        let mut a = i;
        let mut waru = 2;
        while a > 1 {
            while a % waru == 0 {
                a /= waru;
                yaku[waru] += 1;
            }
            waru += 1;
        }
    }

    let ans = num(n, &yaku, 75)
        + num(n, &yaku, 25) * (num(n, &yaku, 3) - 1)
        + num(n, &yaku, 15) * (num(n, &yaku, 5) - 1)
        + num(n, &yaku, 5) * (num(n, &yaku, 5) - 1) * (num(n, &yaku, 3) - 2) / 2;

    println!("{}", ans);
}
