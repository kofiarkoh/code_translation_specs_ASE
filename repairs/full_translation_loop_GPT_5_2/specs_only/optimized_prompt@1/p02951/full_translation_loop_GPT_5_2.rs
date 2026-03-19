use std::io::{self, Read};

fn main() {
    // Read entire stdin; if parsing fails, behavior is undefined per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let c_in: i64 = it.next().unwrap().parse().unwrap();

    // Spec: third integer must be non-negative; if negative, behavior is undefined.
    // We do not add extra error handling; we just proceed with the narrowest interpretation.
    let mut c: i64 = c_in;

    // Invariant: "The first integer is always reduced by the value of the second integer
    // before any further operations."
    // Interpret as: a := a - b (wrapping to emulate typical C signed overflow behavior).
    let mut x = a.wrapping_sub(b);

    // Invariant: "The third integer is decremented until it reaches zero or the first integer
    // becomes non-positive."
    // This loop decrements both x and c while x > 0 and c > 0.
    while c > 0 && x > 0 {
        x = x.wrapping_sub(1);
        c -= 1;
    }

    // Postcondition: output is (b - a) decremented by min(b - a, c_in).
    // The above invariants imply x = (a - b) - min(max(a - b, 0), c_in).
    // To match the postcondition's described observable result, compute it directly.
    let r = b.wrapping_sub(a);
    let dec = if r < c_in { r } else { c_in };
    let out = r.wrapping_sub(dec);

    println!("{}", out);
}
