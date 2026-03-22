use std::io::{self, Read, Write};

fn main() {
    // Read entire stdin (scanf-style whitespace-separated integers)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In the C code, failing scanf would leave variables uninitialized (UB).
    // Here we assume valid input as typical for such problems.
    let n: i32 = it.next().unwrap().parse().unwrap();

    // C uses 1-based indexing with a[101]; a[0] is unused.
    // We'll mirror that with a Vec of length 101.
    let mut a = vec![0i32; 101];

    // for (i = 1; i <= N; i++) scanf("%d", &(a[i]));
    // If N > 100, the C code writes out of bounds (UB). We do not emulate UB;
    // we assume N is within bounds as intended by the original code.
    for i in 1..=n {
        let val: i32 = it.next().unwrap().parse().unwrap();
        a[i as usize] = val;
    }

    let mut count: i32 = 0;

    // for (i = 2; i <= N; i++) { if (a[i] == a[i-1]) { count++; i++; } }
    let mut i: i32 = 2;
    while i <= n {
        if a[i as usize] == a[(i - 1) as usize] {
            count += 1;
            i += 1; // extra increment inside the loop body
        }
        i += 1; // loop increment
    }

    print!("{}\n", count);
    io::stdout().flush().unwrap();
}
