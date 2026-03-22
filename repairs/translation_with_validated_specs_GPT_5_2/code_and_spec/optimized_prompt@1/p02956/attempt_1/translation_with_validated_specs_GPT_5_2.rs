use std::io::{self, Read};

const MOD: i64 = 998_244_353;

#[derive(Clone, Copy, Debug)]
struct Data {
    key: i32,
    id: i32,
}

struct Fenwick {
    n: usize,
    bit: Vec<i32>,
}
impl Fenwick {
    fn new(n: usize) -> Self {
        Self { n, bit: vec![0; n + 1] }
    }
    fn reset(&mut self) {
        self.bit.fill(0);
    }
    fn add(&mut self, mut i: usize, delta: i32) {
        while i <= self.n {
            self.bit[i] += delta;
            i += i & (!i + 1); // i += i & -i, but in usize without negation
        }
    }
    fn sum(&self, mut r: usize) -> i32 {
        let mut s: i32 = 0;
        while r > 0 {
            s += self.bit[r];
            r &= r - 1;
        }
        s
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut d: Vec<Data> = Vec::with_capacity(n);
    for _ in 0..n {
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();
        d.push(Data { key: a, id: b });
    }

    // 5.1 Compress first component into A-group IDs:
    // sort by a (key), group equal a, assign AID, and swap stored fields to (b, AID)
    d.sort_by(|x, y| x.key.cmp(&y.key));
    let mut m: i32 = 0;
    let mut i = 0usize;
    while i < n {
        m += 1;
        let a_val = d[i].key;
        let mut j = i;
        while j < n && d[j].key == a_val {
            let b = d[j].id;
            d[j].key = b; // key becomes original b
            d[j].id = m;  // id becomes AID
            j += 1;
        }
        i = j;
    }
    let m_usize = m as usize;

    // 5.2 Compress second component into B-ranks:
    // sort by b (current key), group equal b, assign BRank into key
    d.sort_by(|x, y| x.key.cmp(&y.key));
    let mut br: i32 = 0;
    i = 0;
    while i < n {
        br += 1;
        let b_val = d[i].key;
        let mut j = i;
        while j < n && d[j].key == b_val {
            d[j].key = br; // key becomes BRank
            j += 1;
        }
        i = j;
    }

    // 6. pow[k] = 2^k mod MOD
    let mut pow: Vec<i64> = vec![0; n + 1];
    pow[0] = 1;
    for k in 1..=n {
        pow[k] = (pow[k - 1] * 2) % MOD;
    }

    // 7. ans init
    let mut ans: i64 = ((pow[n] - 1 + MOD) % MOD) * (n as i64) % MOD;

    // 8. BIT sweeps over AID in [1..m]
    let mut bit = Fenwick::new(m_usize);

    // 8.1 Forward sweep grouped by BRank (d is sorted by BRank already)
    i = 0;
    while i < n {
        let key = d[i].key;
        let mut j = i;
        while j < n && d[j].key == key {
            let id = d[j].id as usize;
            let l = bit.sum(id - 1) as usize;
            let s_id = bit.sum(id) as usize;
            let exp2 = i - s_id; // i is count processed before group
            ans += pow[l] + pow[exp2];
            if ans >= (1i64 << 62) {
                ans %= MOD; // keep bounded; does not change semantics mod MOD
            }
            j += 1;
        }
        ans = (ans + (MOD - pow[i]) * ((j - i) as i64)) % MOD;

        // update BIT with this group
        let mut k = i;
        while k < j {
            let id = d[k].id as usize;
            bit.add(id, 1);
            k += 1;
        }
        i = j;
    }

    // 8.2 Backward sweep grouped by BRank
    bit.reset();
    let mut ii: isize = n as isize - 1;
    while ii >= 0 {
        let key = d[ii as usize].key;
        let mut jj: isize = ii;
        while jj >= 0 && d[jj as usize].key == key {
            let id = d[jj as usize].id as usize;
            let l = bit.sum(id - 1) as usize;
            let s_id = bit.sum(id) as usize;
            let processed_right = (n - (ii as usize) - 1) as usize;
            let exp2 = processed_right - s_id;
            ans += pow[l] + pow[exp2];
            if ans >= (1i64 << 62) {
                ans %= MOD;
            }
            jj -= 1;
        }
        let processed_right = (n - (ii as usize) - 1) as usize;
        ans = (ans + (MOD - pow[processed_right]) * ((ii - jj) as i64)) % MOD;

        // update BIT with this group
        let mut k = ii;
        while k > jj {
            let id = d[k as usize].id as usize;
            bit.add(id, 1);
            k -= 1;
        }
        ii = jj;
    }

    // 9. Swap fields to (AID, BRank) and sort by AID
    for x in &mut d {
        let tmp = x.key;
        x.key = x.id;
        x.id = tmp;
    }
    d.sort_by(|x, y| x.key.cmp(&y.key));

    // 9.1 Forward correction grouped by AID
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

    // 9.2 Backward correction grouped by AID
    ii = n as isize - 1;
    while ii >= 0 {
        let key = d[ii as usize].key;
        let mut jj: isize = ii;
        while jj >= 0 && d[jj as usize].key == key {
            jj -= 1;
        }
        let processed_right = (n - (ii as usize) - 1) as usize;
        ans = (ans + (MOD - pow[processed_right]) * ((ii - jj) as i64)) % MOD;
        ii = jj;
    }

    ans %= MOD;
    if ans < 0 {
        ans += MOD;
    }

    print!("{ans}\n");
    // stdout is line-buffered; explicit flush not required to preserve output.
}
