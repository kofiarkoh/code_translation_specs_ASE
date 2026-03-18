use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three ints like scanf("%d %d %d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    let mut sum: i32 = 0;
    // In the C code, `l` is uninitialized and printed even if never assigned.
    // To preserve that corner-case behavior, we keep it as Option and print 0
    // if it was never set (Rust cannot read uninitialized memory safely).
    let mut l: Option<i32> = None;

    if a > b {
        let mut i = b;
        while i > 0 {
            if a % i == 0 && b % i == 0 {
                sum += 1;
            }
            if sum == c {
                l = Some(i);
            }
            if sum == c {
                break;
            } else {
                // `continue;` in C; here it's implicit by loop structure.
            }
            i -= 1;
        }
        println!("{}", l.unwrap_or(0));
    } else if a < b {
        let mut i = a;
        while i > 0 {
            if a % i == 0 && b % i == 0 {
                sum += 1;
            }
            if sum == c {
                l = Some(i);
            }
            if sum == c {
                break;
            } else {
                // continue
            }
            i -= 1;
        }
        println!("{}", l.unwrap_or(0));
    } else {
        // a == b
        let mut i = a;
        while i > 0 {
            if a % i == 0 && b % i == 0 {
                sum += 1;
            }
            if sum == c {
                l = Some(i);
            }
            if sum == c {
                break;
            } else {
                // continue
            }
            i -= 1;
        }
        println!("{}", l.unwrap_or(0));
    }
}
