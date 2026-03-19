use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap(); // 1000-unit cost
    let b: i32 = it.next().unwrap().parse().unwrap(); // 500-unit cost
    let x: i32 = it.next().unwrap().parse().unwrap(); // required quantity (positive)

    let ans: i32;
    if a >= 2 * b {
        // ceil(x/500) * b, implemented as ((x-1)/500 + 1) * b
        ans = ((x - 1) / 500 + 1) * b;
    } else {
        // floor(x/1000) * a plus optional extra package
        let mut total = (x / 1000) * a;
        let c = x % 1000;
        if c != 0 {
            if c <= 500 && b < a {
                total += b;
            } else {
                total += a;
            }
        }
        ans = total;
    }

    print!("{ans}\n");
}
