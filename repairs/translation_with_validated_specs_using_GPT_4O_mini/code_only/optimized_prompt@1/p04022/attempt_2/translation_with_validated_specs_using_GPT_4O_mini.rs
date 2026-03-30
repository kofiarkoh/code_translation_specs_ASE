use std::cmp::{max, min};
use std::io::{self, BufRead};
use rand::Rng; // Importing the Rng trait for random number generation

const INF: i64 = 1072114514;
const LINF: i64 = 4154118101919364364;
const MOD: i64 = 1000000007;
const PI: f64 = 3.1415926535897932384;

fn zt(a: i64, b: i64) -> i64 {
    max(a, b) - min(a, b)
}

fn round(a: i64, b: i64) -> i64 {
    if (a % b) * 2 >= b {
        a / b + 1
    } else {
        a / b
    }
}

fn ceil(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    let c = gcd(a, b);
    a / c * b
}

fn nCr(a: i64, b: i64) -> i64 {
    let mut r = 1;
    for i in 1..=b {
        r *= a + 1 - i;
        r /= i;
    }
    r
}

fn nHr(a: i64, b: i64) -> i64 {
    nCr(a + b - 1, b)
}

fn fact(a: i64) -> i64 {
    (1..=a).product()
}

fn pow(a: i64, b: i64) -> i64 {
    (1..=b).fold(1, |r, _| r * a)
}

fn dsum(mut x: i64) -> i64 {
    let mut r = 0;
    while x != 0 {
        r += x % 10;
        x /= 10;
    }
    r
}

fn dsumb(mut x: i64, b: i64) -> i64 {
    let mut r = 0;
    while x != 0 {
        r += x % b;
        x /= b;
    }
    r
}

fn sankaku(x: i64) -> i64 {
    (1 + x) * x / 2
}

fn shuffled_get(x: &mut [i64], n: usize) {
    let mut rng = rand::thread_rng();
    let mut b: Vec<usize> = (0..n).collect();
    for i in (1..=n).rev() {
        let p = rng.gen_range(0..i); // Using Rng to generate a random number
        b.swap(i - 1, p);
    }
    for i in 0..n {
        x[b[i]] = read_i64();
    }
}

fn read_i64() -> i64 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn search(x: i64, a: &[i64], n: usize) -> usize {
    let mut st = 0;
    let mut fi = n as isize - 1;
    while st <= fi {
        let te = (st + fi) / 2;
        if a[te as usize] < x {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    st as usize
}

fn prarr(arr: &[i64], n: usize) {
    for (i, &value) in arr.iter().enumerate().take(n) {
        if i > 0 {
            print!(" ");
        }
        print!("{}", value);
    }
    println!();
}

#[derive(Debug, Clone)] // Added Clone trait to Sd
struct Sd {
    val: i64,
    node: i64,
}

fn sdsortfnc(a: &Sd, b: &Sd) -> std::cmp::Ordering {
    a.val.cmp(&b.val)
}

fn make_pl() -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut fl = vec![false; 2222];
    let mut pl = Vec::new();
    let mut dpl = Vec::new();
    let mut trpl = Vec::new();
    for i in 2..2222 {
        if fl[i] {
            continue;
        }
        pl.push(i as i64); // Ensure i is pushed as i64
        dpl.push((i * i) as i64); // Ensure i*i is pushed as i64
        trpl.push((i * i * i) as i64); // Ensure i*i*i is pushed as i64
        for j in (2 * i..2222).step_by(i) {
            fl[j] = true;
        }
    }
    (pl, dpl, trpl)
}

fn llsqr(x: i64) -> i64 {
    let mut st = 1;
    let mut fi = 1_000_000_000;
    while st <= fi {
        let te = (st + fi) / 2;
        if te * te < x {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    st
}

fn main() {
    let (pl, dpl, trpl) = make_pl();
    let n = read_i64();
    let mut a = vec![0; 100005];
    let mut c = 0;
    let mut r = 0;
    let mut tfl = vec![false; 100005];
    let mut dat = vec![Sd { val: 0, node: 0 }; 100005];

    for _ in 0..n {
        let mut t = read_i64();
        let mut w = t;
        for j in 0..pl.len() {
            while t % pl[j] == 0 {
                t /= pl[j];
            }
            while w % trpl[j] == 0 {
                w /= trpl[j];
            }
        }
        if t > 100000 {
            let l = llsqr(t);
            if l * l != t {
                r += 1;
                continue;
            }
        }
        a[c] = w;
        c += 1;
    }

    if c == 0 {
        println!("{}", r);
        return;
    }

    a[..c].sort_unstable();
    let mut t = 0;
    let mut w = 1;
    dat[0].val = a[0];

    for i in 1..c {
        if a[i - 1] != a[i] {
            dat[t].node = w;
            t += 1;
            w = 1;
            dat[t].val = a[i];
        } else {
            w += 1;
        }
    }
    dat[t].node = w;
    t += 1;
    dat[t].val = LINF;

    for i in 0..t {
        if tfl[i] {
            continue;
        }
        tfl[i] = true;
        if dat[i].val == 1 {
            r += 1;
            continue;
        }
        w = 1;
        let mut h = dat[i].val;
        for j in 0..pl.len() {
            while h % pl[j] == 0 {
                h /= pl[j];
            }
            if dat[i].val % pl[j] == 0 {
                if dat[i].val % dpl[j] == 0 {
                    w *= pl[j];
                    if w > 10_000_000_000 {
                        w = -1;
                        break;
                    }
                } else {
                    w *= pl[j];
                    if w > 10_000_000_000 {
                        w = -1;
                        break;
                    }
                    w *= pl[j];
                    if w > 10_000_000_000 {
                        w = -1;
                        break;
                    }
                }
            }
        }
        if w == -1 {
            r += dat[i].node;
            continue;
        }
        if h > 100000 {
            let l = llsqr(h);
            w *= l;
            if w > 10_000_000_000 {
                w = -1;
            }
        } else {
            w *= h;
            if w > 10_000_000_000 {
                w = -1;
            }
            w *= h;
            if w > 10_000_000_000 {
                w = -1;
            }
        }
        if w == -1 {
            r += dat[i].node;
            continue;
        }
        let st = search(w, &dat.iter().map(|d| d.val).collect::<Vec<_>>(), t);
        if dat[st].val != w {
            r += dat[i].node;
            continue;
        }
        tfl[st] = true;
        r += max(dat[i].node, dat[st].node);
    }
    println!("{}", r);
}
