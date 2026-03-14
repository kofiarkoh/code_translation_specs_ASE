fn pow10(n: i32) -> i32 {
    let mut ret = 1;
    for _ in 0..n {
        ret *= 10;
    }
    ret
}

fn solve(s: i32) -> i32 {
    let mut ret = 0;
    let mut n = s * s / 100;
    for i in 0..4 {
        ret += (n % 10) * pow10(i);
        n /= 10;
    }
    ret
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of test cases
    let d: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    for i in 1..=d {
        // Read the integer for the current test case
        let s: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        println!("Case {}:", i);
        
        let mut current_s = s;
        for _ in 0..10 {
            current_s = solve(current_s);
            println!("{}", current_s);
        }
    }
}
