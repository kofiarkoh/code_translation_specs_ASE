use std::io::{self, Read};

const MAX_K: usize = 5000;
const MAX_N: usize = 1000;

fn get_len(an: usize, bn: usize, ais: &[i32], bis: &[i32], k0: usize) -> usize {
    let mut ai: usize = 0;

    for bi in k0..bn {
        if ai >= an {
            return bi - k0;
        }
        let b = bis[bi];
        let mut aj = ai;
        while aj < an && ais[aj] != b {
            aj += 1;
        }
        if aj >= an {
            return bi - k0;
        }
        ai = aj + 1;
    }

    bn - k0
}

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The C code does not validate input; we mirror that by unwrapping.
    let an: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
    let bn: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    // Global arrays in C are fixed-size; we emulate with fixed-size buffers.
    // If an/bn exceed these maxima, the C code would overflow; we clamp reads to avoid panic,
    // but still keep semantics for valid inputs within bounds.
    let mut ais = vec![0i32; MAX_N];
    let mut bis = vec![0i32; MAX_K];

    for i in 0..an.min(MAX_N) {
        ais[i] = it.next().unwrap().parse::<i32>().unwrap();
    }
    // If an > MAX_N, consume remaining inputs to keep parsing aligned (best-effort).
    for _ in an.min(MAX_N)..an {
        let _ = it.next().unwrap().parse::<i32>().unwrap();
    }

    for i in 0..bn.min(MAX_K) {
        bis[i] = it.next().unwrap().parse::<i32>().unwrap();
    }
    for _ in bn.min(MAX_K)..bn {
        let _ = it.next().unwrap().parse::<i32>().unwrap();
    }

    let an_eff = an.min(MAX_N);
    let bn_eff = bn.min(MAX_K);

    let mut max_len: usize = 0;

    for k0 in 0..bn_eff {
        // Preserve the C condition: if (k0 + max_len > bn) break;
        // Use saturating_add to avoid overflow (C signed overflow is UB; inputs are expected small).
        if k0.saturating_add(max_len) > bn_eff {
            break;
        }

        let len = get_len(an_eff, bn_eff, &ais, &bis, k0);
        if max_len < len {
            max_len = len;
        }
    }

    print!("{max_len}\n");
}
