use std::io::{self, BufRead};

const MOD: i64 = 998244353;

#[derive(Debug)]
struct Data {
    key: i64,
    id: i64,
}

fn merge_sort(data: &mut [Data]) {
    let n = data.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    merge_sort(&mut data[0..mid]);
    merge_sort(&mut data[mid..]);

    let mut temp = data.to_vec();
    let (mut p, mut q) = (0, mid);
    for i in 0..n {
        if p >= mid {
            temp[i] = data[q];
            q += 1;
        } else if q >= n {
            temp[i] = data[p];
            p += 1;
        } else {
            temp[i] = if data[p].key < data[q].key {
                data[p].clone()
            } else {
                data[q].clone()
            };
            if temp[i].key == data[p].key {
                p += 1;
            } else {
                q += 1;
            }
        }
    }
    data.copy_from_slice(&temp);
}

fn add_bit(bit: &mut Vec<i64>, n: usize, i: usize, k: i64) {
    let mut idx = i;
    while idx <= n {
        bit[idx] += k;
        idx += idx & (idx as isize).wrapping_neg();
    }
}

fn sum_bit(bit: &Vec<i64>, r: usize) -> i64 {
    let mut sum = 0;
    let mut idx = r;
    while idx > 0 {
        sum += bit[idx];
        idx -= idx & (idx as isize).wrapping_neg();
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut data: Vec<Data> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let key: i64 = parts.next().unwrap().parse().unwrap();
        let id: i64 = parts.next().unwrap().parse().unwrap();
        data.push(Data { key, id });
    }

    merge_sort(&mut data);

    let mut m = 1;
    for i in 0..n {
        let tmp = data[i].key;
        let mut j = i;
        while j < n && data[j].key == tmp {
            data[j].key = data[j].id;
            data[j].id = m;
            j += 1;
        }
        i = j - 1;
        m += 1;
    }

    merge_sort(&mut data);

    let mut n = 1;
    for i in 0..n {
        let tmp = data[i].key;
        let mut j = i;
        while j < n && data[j].key == tmp {
            data[j].key = n;
            j += 1;
        }
        i = j - 1;
        n += 1;
    }

    let mut pow = vec![0; n + 1];
    pow[0] = 1;
    for i in 1..=n {
        pow[i] = (pow[i - 1] * 2) % MOD;
    }

    let mut bit = vec![0; n + 1];
    let mut ans = (pow[n] - 1 + MOD) % MOD * n as i64 % MOD;

    let mut i = 0;
    while i < n {
        let tmp = data[i].key;
        let mut j = i;
        while j < n && data[j].key == tmp {
            ans = (ans + pow[sum_bit(&bit, data[j].id as usize - 1)] + pow[i as usize - sum_bit(&bit, data[j].id as usize)]) % MOD;
            j += 1;
        }
        ans = (ans + (MOD - pow[i as usize]) * (j as i64 - i as i64)) % MOD;

        for j in i..j {
            add_bit(&mut bit, m, data[j].id as usize, 1);
        }
        i = j;
    }

    for i in 1..=m {
        bit[i] = 0;
    }

    for i in (0..n).rev() {
        let tmp = data[i].key;
        let mut j = i;
        while j >= 0 && data[j].key == tmp {
            ans = (ans + pow[sum_bit(&bit, data[j].id as usize - 1)] + pow[n - i - 1 - sum_bit(&bit, data[j].id as usize)]) % MOD;
            j -= 1;
        }
        ans = (ans + (MOD - pow[n - i - 1]) * (i as i64 - j as i64)) % MOD;

        for j in (j + 1)..=i {
            add_bit(&mut bit, m, data[j].id as usize, 1);
        }
        i = j + 1;
    }

    for i in 0..n {
        data[i].key ^= data[i].id;
        data[i].id ^= data[i].key;
        data[i].key ^= data[i].id;
    }

    merge_sort(&mut data);

    for i in 0..n {
        let tmp = data[i].key;
        let mut j = i;
        while j < n && data[j].key == tmp {
            j += 1;
        }
        ans = (ans + (MOD - pow[i as usize]) * (j as i64 - i as i64)) % MOD;
        i = j - 1;
    }

    for i in (0..n).rev() {
        let tmp = data[i].key;
        let mut j = i;
        while j >= 0 && data[j].key == tmp {
            j -= 1;
        }
        ans = (ans + (MOD - pow[n - i - 1]) * (i as i64 - j as i64)) % MOD;
        i = j + 1;
    }

    println!("{}", ans);
}
