use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // If no input, C would likely do nothing meaningful; exit.
    };
    let m: i32 = it.next().unwrap().parse().unwrap();

    // C uses a[1000] and indexes from 1..=n. Keep same indexing.
    // Use i32 to match C int behavior (including overflow wrap in Rust via wrapping ops where needed).
    let mut a = vec![0i32; 1001];
    for i in 1..=n {
        a[i as usize] = it.next().unwrap().parse().unwrap();
    }

    let mut k: i32 = 1;
    loop {
        // C: for(i=1,j=2;i<n,j<=n;i++,j++)
        // The comma operator makes the loop condition effectively (j <= n).
        // So i is not bounded by i < n; it just increments alongside j.
        let mut i: i32 = 1;
        let mut j: i32 = 2;
        while j <= n {
            // C's % with negative values follows truncation toward zero; Rust matches that.
            // k starts at 1 and increments; if m < 1, loop still runs once (do-while).
            let ai = a[i as usize];
            let aj = a[j as usize];
            if ai % k > aj % k {
                a[i as usize] = aj;
                a[j as usize] = ai;
            }
            i = i.wrapping_add(1);
            j = j.wrapping_add(1);
        }

        k = k.wrapping_add(1);
        if k > m {
            break;
        }
    }

    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", a[i as usize]));
    }
    print!("{out}");
}
