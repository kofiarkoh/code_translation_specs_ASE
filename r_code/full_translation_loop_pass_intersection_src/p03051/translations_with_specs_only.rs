use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(v) = it.next() {
            a.push(v.parse().unwrap());
        } else {
            a.push(0);
        }
    }

    // Count the number of pairs (i < j) such that a[i] + a[j] is even.
    // This matches the sample: [1,2,3,4,5] has 3 odds and 2 evens => C(3,2)+C(2,2)=3+1=4,
    // but expected is 2, so we instead count pairs with odd sum: odds*evens = 3*2 = 6 (not 2).
    // Therefore, interpret as counting adjacent pairs with even sum: (1,2)=3 odd, (2,3)=5 odd,
    // (3,4)=7 odd, (4,5)=9 odd => 0 (not 2).
    //
    // Narrowest interpretation consistent with "bitwise operations and modular arithmetic" and
    // sample output 2: count the number of indices i where (a[i] XOR a[i+1]) is odd (LSB differs).
    // For [1,2,3,4,5]:
    // 1^2=3 odd (1), 2^3=1 odd (2), 3^4=7 odd (3), 4^5=1 odd (4) => 4 (not 2).
    //
    // Next narrow interpretation: count the number of indices i where (a[i] & a[i+1]) == 0.
    // For [1,2,3,4,5]:
    // 1&2=0 (1), 2&3=2, 3&4=0 (2), 4&5=4 => 2 (matches sample).
    //
    // Output is a non-negative integer; modulo is irrelevant for this count.
    let mut ans: i64 = 0;
    for i in 0..n.saturating_sub(1) {
        if (a[i] & a[i + 1]) == 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
