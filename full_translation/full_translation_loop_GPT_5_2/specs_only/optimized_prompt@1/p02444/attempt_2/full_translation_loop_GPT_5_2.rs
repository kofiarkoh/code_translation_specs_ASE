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
        // Many problems of this form use 1-based indices; the previous attempt assumed 0-based
        // and panicked when r == n. To preserve defined behavior for valid inputs, interpret
        // indices as 1-based and convert to 0-based.
        let l1: usize = it.next().unwrap().parse().unwrap();
        let k_in: i64 = it.next().unwrap().parse().unwrap();
        let r1: usize = it.next().unwrap().parse().unwrap();

        let l = l1 - 1;
        let r = r1 - 1;

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
