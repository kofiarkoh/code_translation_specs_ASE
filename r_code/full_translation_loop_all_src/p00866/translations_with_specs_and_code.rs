use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Entry {
    t: i32,
    id: usize,
}

fn check(id: usize, t: [i32; 3], a: &[[i32; 60]; 12], tbl: &mut Vec<Entry>) {
    // Permutations of mapping the three observed hand positions to (hour, minute, second)
    const C: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    for j in 0..60 {
        for perm in C.iter() {
            let h = (t[perm[0]] + j) % 60;
            let hh = (h / 5) as usize; // hour in 0..11
            let m = (t[perm[1]] + j) % 60;
            let s = (t[perm[2]] + j) % 60;

            // Validity condition derived from the clock-hand geometry table `a`.
            if a[hh][m as usize] == h {
                let tt = ((hh as i32 * 60 + m) * 60 + s) as i32;
                tbl.push(Entry { t: tt, id });
            }
        }
    }
}

fn main() {
    // Read all input; behavior on malformed input is unspecified by the spec (and C code),
    // so we simply stop if parsing fails.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Precompute table a[12][60] exactly as in C:
    // a[i][j] = (60*i + j)/12
    let mut a = [[0i32; 60]; 12];
    for i in 0..12 {
        for j in 0..60 {
            a[i][j] = ((60 * i + j) / 12) as i32;
        }
    }

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n == 0 {
            // C code stops when n <= 0.
            break;
        }

        let mut tbl: Vec<Entry> = Vec::new();
        for id in 0..n {
            let x: i32 = match it.next().and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => return,
            };
            let y: i32 = match it.next().and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => return,
            };
            let z: i32 = match it.next().and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => return,
            };
            check(id, [x, y, z], &a, &mut tbl);
        }

        // Sort by time (stable/unstable doesn't matter for semantics here).
        tbl.sort_by_key(|e| e.t);

        // Find minimum interval [i..j] covering all ids.
        // This is O(sz^2) as in the C code; spec allows up to 10,000 configurations,
        // but we preserve the original algorithm/semantics.
        let mut best_d: i32 = i32::MAX;
        let mut best_min: i32 = 0;
        let mut best_max: i32 = 0;

        for i in 0..tbl.len() {
            // f is a per-start bitmap of which ids are covered.
            // C uses char f[11], implying n <= 10 in the original problem.
            // We generalize safely to n, preserving behavior for valid inputs.
            let mut seen = vec![false; n];
            let mut k = 0usize;

            for j in i..tbl.len() {
                let id = tbl[j].id;
                if seen[id] {
                    continue;
                }
                seen[id] = true;
                k += 1;
                if k == n {
                    let d = tbl[j].t - tbl[i].t;
                    if d < best_d {
                        best_d = d;
                        best_min = tbl[i].t;
                        best_max = tbl[j].t;
                    }
                    break;
                }
            }
        }

        // Spec says: "If no valid interval exists, the output will not be generated."
        // The C code always prints; however, if tbl is empty or no cover exists, best_d
        // would remain MAX and times would be uninitialized in C (undefined).
        // We follow the spec: only print if we found a valid covering interval.
        if best_d != i32::MAX {
            out.push_str(&format!(
                "{:02}:{:02}:{:02} {:02}:{:02}:{:02}\n",
                best_min / 3600,
                (best_min / 60) % 60,
                best_min % 60,
                best_max / 3600,
                (best_max / 60) % 60,
                best_max % 60
            ));
        }
    }

    print!("{out}");
}
