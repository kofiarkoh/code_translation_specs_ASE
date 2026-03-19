use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches C scanf behavior for whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();

    // C uses a[1..=n] with a fixed 1000-size array; we emulate 1-based indexing.
    // Spec says up to 1000 integers; behavior is undefined outside that.
    let mut a = vec![0i32; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // Note: The C code performs only a single adjacent pass per k (not a full sort),
    // so the result is not necessarily fully sorted by remainder for each divisor.
    // We preserve the C semantics as the source-of-truth for edge behavior.
    let mut k: i32 = 1;
    while k <= m {
        for i in 1..n {
            let j = i + 1;
            if a[i] % k > a[j] % k {
                a.swap(i, j);
            }
        }
        k += 1;
    }

    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", a[i]));
    }
    print!("{out}");
}
