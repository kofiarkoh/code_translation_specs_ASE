use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf behavior for whitespace-separated integers).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // C uses signed long long arithmetic; overflow is undefined in C.
    // We use wrapping ops to provide deterministic behavior if overflow occurs.
    let nk = n / k;
    let cube1 = nk.wrapping_mul(nk).wrapping_mul(nk);

    let ans = if k % 2 != 0 {
        cube1
    } else {
        let t = (n.wrapping_add(k / 2)) / k;
        let cube2 = t.wrapping_mul(t).wrapping_mul(t);
        cube1.wrapping_add(cube2)
    };

    // Exact output formatting: one integer followed by '\n'.
    print!("{}\n", ans);
    // stdout is flushed on program exit; explicit flush not necessary.
}
