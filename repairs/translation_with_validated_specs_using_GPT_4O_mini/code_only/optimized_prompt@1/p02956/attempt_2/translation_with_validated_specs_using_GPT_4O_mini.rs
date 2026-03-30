use std::io::{self, BufRead};

const MOD: i64 = 998244353;

#[derive(Clone, Copy)]
struct Data {
    key: i64,
    id: i64,
}

fn merge_sort(x: &mut [Data], n: usize) {
    let mut y = vec![Data { key: 0, id: 0 }; n];
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    merge_sort(&mut x[0..mid], mid);
    merge_sort(&mut x[mid..n], n - mid);

    let (mut p, mut q) = (0, mid);
    for i in 0..n {
        if p >= mid {
            y[i] = x[q];
            q += 1;
        } else if q >= n {
            y[i] = x[p];
            p += 1;
        } else {
            if x[p].key < x[q].key {
                y[i] = x[p];
                p += 1;
            } else {
                y[i] = x[q];
                q += 1;
            }
        }
    }
    x.copy_from_slice(&y[0..n]);
}

fn add_bit(bit: &mut [i64], n: usize, i: usize, k: i64) {
    let mut idx = i;
    while idx <= n {
        bit[idx] += k;
        idx += idx & idx.wrapping_neg(); // Use wrapping_neg to handle unsigned types
    }
}

fn sum_bit(bit: &[i64], r: usize) -> i64 {
    let mut sum = 0;
    let mut idx = r;
    while idx > 0 {
        sum += bit[idx];
        idx -= idx & idx.wrapping_neg(); // Use wrapping_neg to handle unsigned types
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut d: Vec<Data> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let key: i64 = parts.next().unwrap().parse().unwrap();
        let id: i64 = parts.next().unwrap().parse().unwrap();
        d.push(Data { key, id });
    }

    merge_sort(&mut d, n);

    let mut m = 1;
    for i in 0..n {
        let tmp = d[i].key;
        let mut j = i;
        while j < n && d[j].key == tmp {
            d[j].key = d[j].id;
            d[j].id = m;
            j += 1;
        }
        i = j - 1;
        m += 1;
    }

    merge_sort(&mut d, n);

    let mut n = 1;
    for i in 0..n {
        let tmp = d[i].key;
        let mut j = i;
        while j < n && d[j].key == tmp {
            d[j].key = n;
            j += 1;
        }
        i = j - 1;
        n += 1;
    }

    let mut pow = vec![0; 200001];
    pow[0] = 1;
    for i in 1..=n {
        pow[i] = (pow[i - 1] * 2) % MOD;
    }

    let mut bit = vec![0; 200001];
    let mut ans = (pow[n] - 1 + MOD) % MOD * n as i64 % MOD; // Cast n to i64

    let mut i = 0;
    while i < n {
        let tmp = d[i].key;
        let mut j = i;
        while j < n && d[j].key == tmp {
            ans = (ans + pow[sum_bit(&bit, d[j].id as usize - 1) as usize] + pow[(i as i64 - sum_bit(&bit, d[j].id as usize)) as usize]) % MOD) % MOD;
            j += 1;
        }
        ans = (ans + (MOD - pow[i]) * (j as i64 - i as i64) % MOD) % MOD;

        for j in i..j {
            add_bit(&mut bit, m, d[j].id as usize, 1);
        }
        i = j;
    }

    for i in 1..=m {
        bit[i] = 0;
    }

    for i in (0..n).rev() {
        let tmp = d[i].key;
        let mut j = i;
        while j >= 0 && d[j].key == tmp {
            ans = (ans + pow[sum_bit(&bit, d[j].id as usize - 1) as usize] + pow[(n as i64 - i as i64 - 1 - sum_bit(&bit, d[j].id as usize)) as usize]) % MOD) % MOD;
            j -= 1;
        }
        ans = (ans + (MOD - pow[n - i - 1]) * (i as i64 - j as i64) % MOD) % MOD;

        for j in (j + 1)..=i {
            add_bit(&mut bit, m, d[j].id as usize, 1);
        }
        i = j + 1;
    }

    for i in 0..n {
        d[i].key ^= d[i].id;
        d[i].id ^= d[i].key;
        d[i].key ^= d[i].id;
    }

    merge_sort(&mut d, n);

    for i in 0..n {
        let mut j = i;
        while j < n && d[j].key == d[i].key {
            j += 1;
        }
        ans = (ans + (MOD - pow[i]) * (j as i64 - i as i64) % MOD) % MOD;
        i = j - 1;
    }

    for i in (0..n).rev() {
        let mut j = i;
        while j >= 0 && d[j].key == d[i].key {
            j -= 1;
        }
        ans = (ans + (MOD - pow[n - i - 1]) * (i as i64 - j as i64) % MOD) % MOD;
        i = j + 1;
    }

    println!("{}", ans);
}
