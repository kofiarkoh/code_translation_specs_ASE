use std::io::{self, Read};

const M: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, then parse two whitespace-separated strings (like scanf("%s%s", ...)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t = it.next().unwrap_or("");
    let b = it.next().unwrap_or("");

    let st = t.len();
    let sb = b.len();

    // a[1001][1001] in C, but we size exactly to (st+1) x (sb+1).
    // Values are always kept modulo M, so i64 is sufficient.
    let mut a = vec![vec![0i64; sb + 1]; st + 1];

    for i in 0..=st {
        a[i][0] = 1;
    }

    let tb = t.as_bytes();
    let bb = b.as_bytes();

    for i in 1..=st {
        for j in 0..sb {
            // a[i][j+1] = a[i-1][j+1] % M;
            a[i][j + 1] = a[i - 1][j + 1] % M;
            if tb[i - 1] == bb[j] {
                // a[i][j+1] = (a[i-1][j+1] + a[i-1][j]) % M;
                a[i][j + 1] = (a[i - 1][j + 1] + a[i - 1][j]) % M;
            }
        }
    }

    // Match C's printf("%lld\n", ...)
    println!("{}", a[st][sb]);
}
