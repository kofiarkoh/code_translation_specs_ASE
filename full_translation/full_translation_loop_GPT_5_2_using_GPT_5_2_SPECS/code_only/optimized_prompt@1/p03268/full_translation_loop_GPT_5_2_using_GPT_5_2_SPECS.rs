use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two signed 64-bit integers (like scanf("%lld %lld", ...)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // C uses signed long long arithmetic; overflow is UB in C, but typical contest inputs avoid it.
    // We use wrapping ops to avoid Rust panics in debug and to emulate two's-complement wraparound.
    let nk = n / k;
    let cube1 = nk.wrapping_mul(nk).wrapping_mul(nk);

    let ans = if k % 2 != 0 {
        cube1
    } else {
        let t = (n.wrapping_add(k / 2)) / k;
        let cube2 = t.wrapping_mul(t).wrapping_mul(t);
        cube1.wrapping_add(cube2)
    };

    // Match printf("%lld\n", ...) exactly.
    print!("{}\n", ans);

    // fflush(stdout) equivalent: ensure output is flushed.
    use std::io::Write;
    io::stdout().flush().unwrap();
}
