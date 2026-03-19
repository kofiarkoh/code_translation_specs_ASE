use std::io::{self, Read};

const HASHSIZ: usize = 300_017;

#[derive(Clone, Copy)]
struct HashEnt {
    b: i32,
    v: i32,
}

fn lookup(hash: &[HashEnt], b: i32) -> i32 {
    // In the C code, b is always positive (b[i]+1 where b[i] in [0,q-1]).
    let mut idx = (b as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.b == 0 {
            return 0;
        }
        if e.b == b {
            return e.v;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

fn insert(hash: &mut [HashEnt], b: i32) {
    let mut idx = (b as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.b == 0 {
            hash[idx] = HashEnt { b, v: 1 };
            return;
        }
        if e.b == b {
            hash[idx].v += 1;
            return;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

fn main() {
    // Read all input and parse as integers (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // tr[10][6] initialized to 0, then specific entries set to 1.
    let mut tr = [[0i32; 6]; 10];
    tr[0][2] = 1;
    tr[2][2] = 1;
    tr[4][2] = 1;
    tr[6][2] = 1;
    tr[8][2] = 1;
    tr[0][5] = 1;
    tr[5][5] = 1;

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let s: i32 = it.next().unwrap().parse().unwrap();
        let w: i32 = it.next().unwrap().parse().unwrap();
        let q: i32 = it.next().unwrap().parse().unwrap();

        if n <= 0 {
            break;
        }
        let n_usize = n as usize;

        let mut a = vec![0i32; n_usize];
        let mut b = vec![0i32; n_usize + 1];

        // Generate a[] exactly as in C, using 32-bit wrapping shifts/xors.
        let mut g: i32 = s;
        for i in 0..n_usize {
            a[i] = ((g / 7) % 10);
            if (g & 1) != 0 {
                // In C, right shift of signed negative is implementation-defined.
                // Here we use arithmetic shift (Rust's >> on i32), matching common C behavior.
                g = (g >> 1) ^ w;
            } else {
                g >>= 1;
            }
        }

        let mut ans: i32 = 0;

        if q == 2 || q == 5 {
            let mut k: i32 = 0;
            for i in 0..n_usize {
                if a[i] != 0 {
                    k += 1;
                }
                let ai = a[i] as usize;
                if tr[ai][q as usize] != 0 {
                    ans += k;
                }
            }
        } else {
            // memset(hash,0,sizeof(hash))
            let mut hash = vec![HashEnt { b: 0, v: 0 }; HASHSIZ + 2];

            // for (k = 1, b[n] = 0, i = n-1; i >= 0; i--)
            let mut k: i32 = 1;
            b[n_usize] = 0;
            for i in (0..n_usize).rev() {
                // b[i] = (k*a[i] + b[i+1]) % q;
                // Use i64 to avoid overflow differences; C int overflow is UB, but values are small here.
                let val = (k as i64) * (a[i] as i64) + (b[i + 1] as i64);
                b[i] = (val % (q as i64)) as i32;

                // k = ((k<<3)+(k<<1)) % q;  // k = (k*10) % q
                let kval = ((k as i64) * 10) % (q as i64);
                k = kval as i32;
            }

            for i in 0..n_usize {
                if a[i] != 0 {
                    insert(&mut hash, b[i] + 1);
                }
                ans += lookup(&hash, b[i + 1] + 1);
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
