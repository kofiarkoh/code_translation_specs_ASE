use std::io::{self, Write};

fn get_way(x: usize, n: usize, sum: usize, s: usize, val: &mut [bool; 10]) -> usize {
    if n == 0 {
        return if sum == s { 1 } else { 0 };
    }

    let mut ret = 0;
    for i in x..10 {
        if !val[i] && sum + i <= s {
            val[i] = true; // Mark this number as used
            ret += get_way(i, n - 1, sum + i, s, val);
            val[i] = false; // Unmark this number
        }
    }
    ret
}

fn main() {
    let mut input = String::new();
    let mut val = [false; 10]; // Track used integers

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("Invalid input");
        let s: usize = iter.next().unwrap().parse().expect("Invalid input");

        if n == 0 && s == 0 {
            break;
        }

        // Ensure n does not exceed 10 and s is non-negative
        if n > 10 || s < 0 {
            continue; // Undefined behavior as per spec, simply skip
        }

        let result = get_way(0, n, 0, s, &mut val);
        println!("{}", result);
    }
}
