use std::io::{self, Read};

fn main() {
    // Read entire stdin and split on whitespace to tokens
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, emulate undefined/implementation-defined behavior by exiting silently.
        return;
    }
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        // No input: nothing to do (mirrors unspecified behavior on bad input).
        return;
    }

    // Parse n as C 'int' -> i32
    let n: i32 = match tokens[0].parse() {
        Ok(x) => x,
        Err(_) => return, // malformed input -> unspecified in spec; exit quietly
    };

    // Spec expects 0 <= n <= 100 to avoid out-of-bounds in original C.
    if n < 0 || n > 100 {
        // Out-of-range n produces undefined behavior in the original; we choose to exit.
        return;
    }
    let n_usize = n as usize;

    // Check there are enough tokens for n pairs: 1 + 2*n tokens total
    if tokens.len() < 1 + 2 * n_usize {
        // Not enough input tokens: behave as unspecified -> exit.
        return;
    }

    // Read pairs into vectors of i32
    let mut c1: Vec<i32> = Vec::with_capacity(n_usize);
    let mut c2: Vec<i32> = Vec::with_capacity(n_usize);
    let mut idx = 1;
    for _ in 0..n_usize {
        let a: i32 = match tokens[idx].parse() {
            Ok(x) => x,
            Err(_) => return,
        };
        let b: i32 = match tokens[idx + 1].parse() {
            Ok(x) => x,
            Err(_) => return,
        };
        c1.push(a);
        c2.push(b);
        idx += 2;
    }

    // If n <= 1, no merges; print 0.
    if n_usize <= 1 {
        println!("0");
        return;
    }

    // Accumulator s, using wrapping arithmetic to emulate C int wraparound behavior.
    let mut s: i32 = 0;

    // Perform n-1 merges
    for _merge_step in 0..(n_usize - 1) {
        let m = c1.len();
        // Selection baseline as in spec: amax initialized to 0, bmin initialized to 0.
        // If no j ever satisfies the condition, original C code used an uninitialized index (undefined).
        // Here we follow the same logic but detect the case and panic to indicate undefined behavior.
        let mut amax: i32 = 0;
        let mut bmin: i32 = 0;
        let mut chosen: Option<usize> = None;

        for j in 0..(m - 1) {
            // Compute key1 = c2[j] * c1[j+1] and key2 = c1[j] * c2[j+1]
            // Use wrapping_mul to emulate typical C two's-complement wraparound on overflow.
            let a = c2[j].wrapping_mul(c1[j + 1]);
            let b = c1[j].wrapping_mul(c2[j + 1]);

            // Selection condition: choose j that maximizes key1; ties broken by minimal key2.
            if a > amax || (a == amax && b < bmin) {
                chosen = Some(j);
                amax = a;
                bmin = b;
            }
        }

        // If no index was ever chosen, original behavior was undefined (uninitialized index).
        // We reflect that by panicking with a descriptive message.
        let j = match chosen {
            Some(v) => v,
            None => panic!("selection produced no index (undefined behavior per specification)"),
        };

        // Add product c1[j]*c2[j]*c1[j+1]*c2[j+1] to accumulator s, using wrapping arithmetic.
        let term = c1[j]
            .wrapping_mul(c2[j])
            .wrapping_mul(c1[j + 1])
            .wrapping_mul(c2[j + 1]);
        s = s.wrapping_add(term);

        // Replace pair at j and j+1 with (c1[j], c2[j+1]) and compact sequence.
        // We set c2[j] = c2[j+1], then remove element at j+1 from both vectors.
        c2[j] = c2[j + 1];
        // Remove element j+1 (this shifts subsequent elements left).
        c1.remove(j + 1);
        c2.remove(j + 1);
        // c1[j] already holds original c1[j], so no assignment needed.
    }

    // Print final accumulated sum s followed by newline.
    println!("{}", s);
}
