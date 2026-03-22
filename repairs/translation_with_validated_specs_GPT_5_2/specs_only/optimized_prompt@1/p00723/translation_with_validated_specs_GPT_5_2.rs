use std::collections::HashSet;
use std::io::{self, Read};

fn reverse_range(v: &mut [u8], start: usize, end_exclusive: usize) {
    if end_exclusive <= start + 1 {
        return;
    }
    v[start..end_exclusive].reverse();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut outputs = Vec::with_capacity(n);

    for _case in 0..n {
        let t = it.next().unwrap_or("");
        let bytes = t.as_bytes();
        let l = bytes.len();

        if l <= 1 {
            outputs.push("0".to_string());
            continue;
        }

        // Per-test-case unique set; capacity constraints are assumed by the spec.
        let mut seen: HashSet<Vec<u8>> = HashSet::new();

        for i in 1..=l - 1 {
            // A. Reversal-based sequence on split (i, L-i)
            let mut s = bytes.to_vec();

            // 2. Consider s itself as a candidate (once per split position).
            seen.insert(s.clone());

            // 3. Reverse prefix [0..i)
            reverse_range(&mut s, 0, i);
            seen.insert(s.clone());

            // 4. Reverse suffix [i..L)
            reverse_range(&mut s, i, l);
            seen.insert(s.clone());

            // 5. Reverse prefix [0..i) again
            reverse_range(&mut s, 0, i);
            seen.insert(s.clone());

            // 6. Reset s := t (implicit by reinitialization below)

            // B. Rotation-then-reversal sequence
            // 1-2. Rotate: t[i..L) + t[0..i)
            let mut s = Vec::with_capacity(l);
            s.extend_from_slice(&bytes[i..]);
            s.extend_from_slice(&bytes[..i]);
            seen.insert(s.clone());

            // 3. Reverse prefix of length L-i => [0..L-i)
            let left_len = l - i;
            reverse_range(&mut s, 0, left_len);
            seen.insert(s.clone());

            // 4. Reverse suffix of length i => [L-i..L)
            reverse_range(&mut s, left_len, l);
            seen.insert(s.clone());

            // 5. Reverse prefix of length L-i again
            reverse_range(&mut s, 0, left_len);
            seen.insert(s.clone());

            // 6. Reset s := t (end for this i)
        }

        outputs.push(seen.len().to_string());
    }

    print!("{}", outputs.join("\n"));
}
