use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Entry {
    t: i32,
    id: usize,
}

fn check(id: usize, hands: [i32; 3], a: &[[i32; 60]; 12], tbl: &mut Vec<Entry>) {
    // Permutations of assigning the three readings to (hour, minute, second)
    const C: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    for offset in 0..60 {
        for perm in C.iter() {
            let h = (hands[perm[0]] + offset) % 60;
            let hh = (h / 5) as usize; // hour in 0..11
            let m = (hands[perm[1]] + offset) % 60;
            let s = (hands[perm[2]] + offset) % 60;

            // Hour hand position must match analog geometry for (hh, m)
            if a[hh][m as usize] == h {
                let t = ((hh as i32 * 60 + m) * 60) + s;
                tbl.push(Entry { t, id });
            }
        }
    }
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Precompute hour-hand position table a[hour][minute] -> position on 0..59 scale.
    // Matches C: a[i][j] = (60*i + j)/12
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
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n <= 0 {
            break;
        }
        let n_usize = n as usize;

        let mut tbl: Vec<Entry> = Vec::new();
        for id in 0..n_usize {
            let t0: i32 = it.next().unwrap().parse().unwrap();
            let t1: i32 = it.next().unwrap().parse().unwrap();
            let t2: i32 = it.next().unwrap().parse().unwrap();
            check(id, [t0, t1, t2], &a, &mut tbl);
        }

        // Sort by time (stable tie-breaking is irrelevant to correctness here).
        tbl.sort_by_key(|e| e.t);

        // Find minimal interval [min_t, max_t] covering all watch ids.
        // This follows the C code's O(sz^2) search order and tie behavior:
        // update only when strictly smaller length is found.
        let mut best_d: i32 = i32::MAX;
        let mut best_min: i32 = 0;
        let mut best_max: i32 = 0;

        for i in 0..tbl.len() {
            // C uses a fixed 11-byte array; spec allows up to 10,000 watches.
            // We use a Vec<u8> sized to n to preserve semantics for larger n.
            let mut seen = vec![0u8; n_usize];
            let mut k = 0usize;

            for j in i..tbl.len() {
                let id = tbl[j].id;
                if seen[id] != 0 {
                    continue;
                }
                seen[id] = 1;
                k += 1;
                if k == n_usize {
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

        // Output formatting exactly as C: "%02d:%02d:%02d %02d:%02d:%02d\n"
        let (min_t, max_t) = (best_min, best_max);
        out.push_str(&format!(
            "{:02}:{:02}:{:02} {:02}:{:02}:{:02}\n",
            min_t / 3600,
            (min_t / 60) % 60,
            min_t % 60,
            max_t / 3600,
            (max_t / 60) % 60,
            max_t % 60
        ));
    }

    print!("{out}");
}
