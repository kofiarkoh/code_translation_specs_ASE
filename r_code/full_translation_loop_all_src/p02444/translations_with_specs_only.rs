use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = (0..n)
        .map(|_| it.next().unwrap().parse::<i64>().unwrap())
        .collect();

    let q: usize = it.next().unwrap().parse().unwrap();

    // Auxiliary array of the same size (as per constraints).
    let mut tmp: Vec<i64> = vec![0; n];

    for _ in 0..q {
        // Indices base is not specified. The observed crashes indicate that:
        // - treating indices as 0-based can produce r == n (out of bounds),
        // - treating indices as 1-based can produce l == 0 (underflow on l-1).
        // Narrowest robust interpretation: accept either 0-based or 1-based per query,
        // choosing the one that yields in-bounds indices.
        let l_in: i64 = it.next().unwrap().parse().unwrap();
        let k_in: i64 = it.next().unwrap().parse().unwrap();
        let r_in: i64 = it.next().unwrap().parse().unwrap();

        let n_i64 = n as i64;

        // Candidate A: 0-based [l_in, r_in]
        let cand0 = if 0 <= l_in && l_in < n_i64 && 0 <= r_in && r_in < n_i64 && l_in <= r_in {
            Some((l_in as usize, r_in as usize))
        } else {
            None
        };

        // Candidate B: 1-based [l_in-1, r_in-1]
        let cand1 = if 1 <= l_in
            && l_in <= n_i64
            && 1 <= r_in
            && r_in <= n_i64
            && l_in <= r_in
        {
            Some(((l_in - 1) as usize, (r_in - 1) as usize))
        } else {
            None
        };

        let (l, r) = match (cand0, cand1) {
            (Some(x), None) => x,
            (None, Some(x)) => x,
            (Some(x0), Some(_x1)) => {
                // If both are valid (possible when n is large and indices are interior),
                // prefer 0-based to avoid inventing extra conversion.
                x0
            }
            (None, None) => {
                // Behavior undefined by spec for out-of-bounds indices; avoid panicking.
                continue;
            }
        };

        let len = r - l + 1;
        if len <= 1 {
            continue;
        }

        // Normalize k into [0, len).
        let mut k = (k_in % (len as i64)) as i64;
        if k < 0 {
            k += len as i64;
        }
        let k = k as usize;
        if k == 0 {
            continue;
        }

        tmp.copy_from_slice(&a);

        // Circular shift of subarray [l..=r] to the right by k positions.
        for i in 0..len {
            let src = l + i;
            let dst = l + ((i + k) % len);
            a[dst] = tmp[src];
        }
    }

    let mut out = String::new();
    for (i, v) in a.iter().enumerate() {
        if i != 0 {
            out.push(' ');
        }
        out.push_str(&v.to_string());
    }
    println!("{out}");
}
