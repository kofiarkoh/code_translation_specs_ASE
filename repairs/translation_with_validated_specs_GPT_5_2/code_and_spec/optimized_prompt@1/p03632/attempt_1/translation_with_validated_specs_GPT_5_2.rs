use std::io::{self, Read};

fn main() {
    // Read all input (expects four integers).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();
    let d: i32 = it.next().unwrap().parse().unwrap();

    // SPEC: overlap length for closed intervals [a,b] and [c,d] is:
    // if intersection is [L,R] with L <= R, output R - L, else 0.
    let l = a.max(c);
    let r = b.min(d);

    let ans = if l <= r { r - l } else { 0 };
    print!("{}", ans);

    // Note: The provided C code counts integer points in the intersection and prints (count - 1),
    // which equals R - L when endpoints are integers and within range. This matches the SPEC.
}
