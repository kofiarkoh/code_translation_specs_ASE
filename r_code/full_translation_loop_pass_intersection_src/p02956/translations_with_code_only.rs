use std::io::{self, Read};

const MOD: i64 = 998_244_353;

#[derive(Clone, Copy, Default)]
struct Data {
    key: i32,
    id: i32,
}

// Stable merge sort matching the C code's comparison:
// y[i] = (x[p].key < x[q].key) ? x[p++] : x[q++]
// i.e., ties take from the right half (NOT stable).
fn merge_sort(x: &mut [Data], y: &mut [Data]) {
    let n = x.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    let (left, right) = x.split_at_mut(mid);
    merge_sort(left, y);
    merge_sort(right, y);

    let mut p = 0usize;
    let mut q = mid;
    for i in 0..n {
        if p >= mid {
            y[i] = x[q];
            q += 1;
        } else if q >= n {
            y[i] = x[p];
            p += 1;
        } else {
            // ties go to right half
            if x[p].key < x[q].key {
                y[i] = x[p];
                p += 1;
            } else {
                y[i] = x[q];
                q += 1;
            }
        }
    }
    x.copy_from_slice(&y[..n]);
}

fn add_bit(n: usize, bit: &mut [i32], mut i: usize, k: i32) {
    while i <= n {
        bit[i] = bit[i].wrapping_add(k); // mimic C int overflow behavior
        let lsb = i & (!i + 1);
        i += lsb;
    }
}

fn sum_bit(bit: &[i32], mut r: usize) -> i32 {
    let mut sum: i32 = 0;
    while r > 0 {
        sum = sum.wrapping_add(bit[r]); // mimic C int overflow behavior
        let lsb = r & (!r + 1);
        r -= lsb;
    }
    sum
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_usize: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let n = n_usize;

    let mut d: Vec<Data> = vec![Data::default(); n];
    for i in 0..n {
        let key: i32 = it.next().unwrap().parse().unwrap();
        let id: i32 = it.next().unwrap().parse().unwrap();
        d[i] = Data { key, id };
    }

    // temp buffer for merge sort (like static y[200001])
    let mut y: Vec<Data> = vec![Data::default(); n.max(1)];

    merge_sort(&mut d, &mut y);

    // First grouping by original key; assign id = m (group index), key = old id
    let mut m: i32 = 1;
    let mut i: usize = 0;
    while i < n {
        let tmp = d[i].key;
        let mut j = i;
        while j < n && d[j].key == tmp {
            d[j].key = d[j].id;
            d[j].id = m;
            j += 1;
        }
        // C code: for (i=0,m=1; i<N; i++,m++) { ...; j=i-1; }
        // The inner loop advances j, but outer i increments by 1 only; net effect:
        // m increments once per i, not per group. We must preserve that.
        // So we do exactly: i += 1; m += 1; (not i=j).
        i += 1;
        m += 1;
    }

    merge_sort(&mut d, &mut y);

    // Second grouping by key (which is old id); assign key = n (rank by i)
    let mut rank: i32 = 1;
    i = 0;
    while i < n {
        let tmp = d[i].key;
        let mut j = i;
        while j < n && d[j].key == tmp {
            d[j].key = rank;
            j += 1;
        }
        i += 1;
        rank += 1;
    }

    // pow[0..=N]
    let mut pow: Vec<i64> = vec![0; n + 1];
    pow[0] = 1;
    for i in 1..=n {
        pow[i] = (pow[i - 1] * 2) % MOD;
    }

    // BIT size uses m (as in C). m can be up to N+1.
    let m_usize = m as usize;
    let mut bit: Vec<i32> = vec![0; m_usize + 2];

    let mut ans: i64 = ((pow[n] - 1 + MOD) % MOD) * (n as i64) % MOD;

    // Forward pass
    i = 0;
    while i < n {
        let key = d[i].key;
        let mut j = i;
        while j < n && d[j].key == key {
            let id = d[j].id as usize;
            let s1 = sum_bit(&bit, id - 1) as usize;
            let s2 = sum_bit(&bit, id) as i32;
            let s2u = (i as i32 - s2) as usize; // i - sum_BIT(...)
            ans += pow[s1] + pow[s2u];
            j += 1;
        }
        ans = (ans + (MOD - pow[i]) * ((j - i) as i64)) % MOD;

        let mut k = i;
        while k < n && d[k].key == key {
            add_bit(m_usize, &mut bit, d[k].id as usize, 1);
            k += 1;
        }
        i = j;
    }

    // Clear BIT[1..=m]
    for idx in 1..=m_usize {
        bit[idx] = 0;
    }

    // Backward pass
    let mut ii: i64 = n as i64 - 1;
    while ii >= 0 {
        let i_us = ii as usize;
        let key = d[i_us].key;
        let mut j: i64 = ii;
        while j >= 0 && d[j as usize].key == key {
            let id = d[j as usize].id as usize;
            let s1 = sum_bit(&bit, id - 1) as usize;
            let s2 = sum_bit(&bit, id) as i32;
            let s2u = ((n as i32 - i_us as i32 - 1) - s2) as usize;
            ans += pow[s1] + pow[s2u];
            j -= 1;
        }
        ans = (ans + (MOD - pow[n - i_us - 1]) * ((ii - j) as i64)) % MOD;

        let mut k: i64 = ii;
        while k >= 0 && d[k as usize].key == key {
            add_bit(m_usize, &mut bit, d[k as usize].id as usize, 1);
            k -= 1;
        }
        ii = j;
    }

    // Swap key and id using XOR trick (as in C)
    for v in &mut d {
        v.key ^= v.id;
        v.id ^= v.key;
        v.key ^= v.id;
    }
    merge_sort(&mut d, &mut y);

    // Forward grouping subtract
    i = 0;
    while i < n {
        let key = d[i].key;
        let mut j = i;
        while j < n && d[j].key == key {
            j += 1;
        }
        ans = (ans + (MOD - pow[i]) * ((j - i) as i64)) % MOD;
        i = j;
    }

    // Backward grouping subtract
    ii = n as i64 - 1;
    while ii >= 0 {
        let i_us = ii as usize;
        let key = d[i_us].key;
        let mut j: i64 = ii;
        while j >= 0 && d[j as usize].key == key {
            j -= 1;
        }
        ans = (ans + (MOD - pow[n - i_us - 1]) * ((ii - j) as i64)) % MOD;
        ii = j;
    }

    // Match C's %lld\n
    println!("{}", ans);
}
