use std::io::{self, BufRead};

const HASHSIZ: usize = 300017;

#[derive(Copy, Clone)]
struct HashEntry {
    b: i32,
    v: i32,
}

fn lookup(hash: &[HashEntry; HASHSIZ], b: i32) -> i32 {
    let mut p = (b % HASHSIZ as i32) as usize;
    while hash[p].b != 0 {
        if hash[p].b == b {
            return hash[p].v;
        }
        p = (p + 1) % HASHSIZ;
    }
    0
}

fn insert(hash: &mut [HashEntry; HASHSIZ], b: i32) {
    let mut p = (b % HASHSIZ as i32) as usize;
    while hash[p].b != 0 {
        if hash[p].b == b {
            hash[p].v += 1;
            return;
        }
        p = (p + 1) % HASHSIZ;
    }
    hash[p].b = b;
    hash[p].v = 1;
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut hash: [HashEntry; HASHSIZ] = [HashEntry { b: 0, v: 0 }; HASHSIZ];

    let mut tr = [[0; 6]; 10];
    tr[0][2] = 1;
    tr[2][2] = 1;
    tr[4][2] = 1;
    tr[6][2] = 1;
    tr[8][2] = 1;
    tr[0][5] = 1;
    tr[5][5] = 1;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let s = nums.next().unwrap();
        let w = nums.next().unwrap();
        let q = nums.next().unwrap();

        if n <= 0 {
            break;
        }

        let mut g = s;
        let mut a = vec![0; n as usize];
        for i in 0..n {
            a[i as usize] = (g / 7) % 10;
            if g & 1 != 0 {
                g = (g >> 1) ^ w;
            } else {
                g >>= 1;
            }
        }

        let mut ans = 0;
        if q == 2 || q == 5 {
            let mut k = 0;
            for i in 0..n {
                if a[i as usize] != 0 {
                    k += 1;
                }
                if tr[a[i as usize] as usize][q as usize] != 0 {
                    ans += k;
                }
            }
        } else {
            hash = [HashEntry { b: 0, v: 0 }; HASHSIZ];
            let mut k = 1;
            let mut b = vec![0; (n + 1) as usize];
            b[n as usize] = 0;
            for i in (0..n).rev() {
                b[i as usize] = (k * a[i as usize] + b[(i + 1) as usize]) % q;
                k = (k * 8) % q; // Changed to multiplication by 8 directly
            }
            for i in 0..n {
                if a[i as usize] != 0 {
                    insert(&mut hash, b[i as usize] + 1);
                }
                ans += lookup(&hash, b[i as usize + 1] + 1);
            }
        }
        println!("{}", ans);
    }
}
