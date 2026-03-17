use std::io;

fn num(a: usize, n: usize, yaku: &[usize; 101]) -> usize {
    let mut c = 0;
    for i in 2..=n {
        if yaku[i] >= a - 1 {
            c += 1;
        }
    }
    c
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    let mut yaku = [0; 101];

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

    let ans = num(75, n, &yaku)
        + num(25, n, &yaku) * (num(3, n, &yaku) - 1)
        + num(15, n, &yaku) * (num(5, n, &yaku) - 1)
        + num(5, n, &yaku) * (num(5, n, &yaku) - 1) * (num(3, n, &yaku) - 2) / 2;

    println!("{}", ans);
}
