use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-like whitespace handling)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C code uses fixed-size array p[100][2]; keep same capacity.
    // If n > 100, the C code would write out of bounds (UB). We clamp to 100
    // to avoid panic while still reading input; behavior beyond that is not
    // well-defined in C anyway.
    let mut p = [[0i32; 2]; 100];

    let limit = n.min(100);
    for i in 0..limit {
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        p[i][0] = x;
        p[i][1] = y;
    }
    // If n > 100, consume remaining coordinates to match scanf consumption.
    for _ in limit..n {
        let _x: i32 = it.next().unwrap().parse().unwrap();
        let _y: i32 = it.next().unwrap().parse().unwrap();
    }

    let mut menseki: f64 = 0.0;

    // The C expression does signed int arithmetic; overflow is UB in C.
    // We emulate typical two's-complement wraparound using wrapping_* ops.
    if n != 0 {
        for i in 0..limit {
            let j = (i + 1) % limit;
            let dx = p[i][0].wrapping_sub(p[j][0]);
            let sy = p[i][1].wrapping_add(p[j][1]);
            let prod = dx.wrapping_mul(sy);
            menseki += prod as f64;
        }
    }

    // Match printf("%.1lf\n", menseki/2.0);
    println!("{:.1}", menseki / 2.0);
}
