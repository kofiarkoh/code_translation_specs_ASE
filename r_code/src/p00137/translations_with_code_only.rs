use std::io::{self, Write};

fn pow10(n: i32) -> i32 {
    let mut ret = 1;
    for _ in 0..n {
        ret *= 10;
    }
    ret
}

fn solve(s: i32) -> i32 {
    let mut n = s * s / 100;
    let mut ret = 0;

    for i in 0..4 {
        ret += (n % 10) * pow10(i);
        n /= 10;
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let mut d: i32 = input.trim().parse().unwrap();
    let mut case_number = 1;

    while d > 0 {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut s: i32 = input.trim().parse().unwrap();

        writeln!(stdout, "Case {}:", case_number).unwrap();
        for _ in 0..10 {
            s = solve(s);
            writeln!(stdout, "{}", s).unwrap();
        }
        case_number += 1;
        d -= 1;
    }
}
