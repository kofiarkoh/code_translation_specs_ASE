use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse first integer as n (matches scanf("%d",&n))
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // no input; C would have undefined behavior, but we just exit
    };

    // yaku[101] initialized to 0
    let mut yaku = vec![0i32; 101];

    // Factorize all i in [2..=n] and accumulate prime exponents into yaku
    // This is a direct translation of the C loops (including the naive waru increment).
    for i in 2..=n {
        let mut a = i;
        let mut waru = 2;
        while a > 1 {
            while a % waru == 0 {
                a /= waru;
                if (waru as usize) < yaku.len() {
                    yaku[waru as usize] += 1;
                }
            }
            waru += 1;
        }
    }

    // num(a): count i in [2..=n] with yaku[i] >= a-1
    let num = |a: i32| -> i32 {
        let mut c = 0i32;
        for i in 2..=n {
            let idx = i as usize;
            if idx < yaku.len() && yaku[idx] >= a - 1 {
                c += 1;
            }
        }
        c
    };

    // Compute ans with i32 arithmetic (as in C int). Values are small for typical constraints.
    let ans: i32 = num(75)
        + num(25) * (num(3) - 1)
        + num(15) * (num(5) - 1)
        + num(5) * (num(5) - 1) * (num(3) - 2) / 2;

    print!("{}\n", ans);
}
