#![allow(non_snake_case)]

use std::io::{self, Write, BufRead};
use std::str::FromStr;

const INF: i64 = 9223372036854775807;
const EPS: f64 = 1e-10;
const MOD: i64 = 1_000_000_007;

fn in_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn in_long() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i64>().unwrap()
}

fn print_int(a: i32) {
    println!("{}", a);
}

fn print_long(a: i64) {
    println!("{}", a);
}

fn main() {
    let mut c = [0; 50];
    
    loop {
        let n = in_int();
        let p = in_int();
        if n == 0 {
            break;
        }
        
        let mut i = 0;
        let mut w = p;
        c.fill(0); // Reset array to zero

        loop {
            if w > 0 {
                w -= 1;
                c[i as usize % n as usize] += 1;
                if w == 0 {
                    let mut j = 0;
                    while j < n {
                        if j != (i as usize % n as usize) && c[j] > 0 {
                            break;
                        }
                        j += 1;
                    }
                    if j == n {
                        print_int(i as i32 % n as i32);
                        break;
                    }
                }
            } else {
                w += c[i as usize % n as usize];
                c[i as usize % n as usize] = 0;
            }
            i += 1;
        }
    }
}
