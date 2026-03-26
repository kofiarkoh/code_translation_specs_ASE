use std::cmp::{max, min};
use std::io::{self, BufRead, Write};
use rand::Rng; // Ensure the rand crate is included in your Cargo.toml

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
    let mut r = 1;
    for i in 1..=a {
        r *= i;
    }
    r
}

fn pow(a: i64, b: i64) -> i64 {
    let mut r = 1;
    for _ in 1..=b {
        r *= a;
    }
    r
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

fn llmax(a: i64, b: i64) -> i64 {
    max(a, b)
}

fn llmin(a: i64, b: i64) -> i64 {
    min(a, b)
}

fn llzt(a: i64, b: i64) -> i64 {
    llmax(a, b) - llmin(a, b)
}

fn llround(a: i64, b: i64) -> i64 {
    if (a % b) * 2 >= b {
        a / b + 1
    } else {
        a / b
    }
}

fn llceil(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}

fn llgcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn lllcm(a: i64, b: i64) -> i64 {
    let c = llgcd(a, b);
    a / c * b
}

fn llnCr(a: i64, b: i64) -> i64 {
    let mut r = 1;
    for i in 1..=b {
        r *= a + 1 - i;
        r /= i;
    }
    r
}

fn llnHr(a: i64, b: i64) -> i64 {
    llnCr(a + b - 1, b)
}

fn llfact(a: i64) -> i64 {
    let mut r = 1;
    for i in 1..=a {
        r *= i;
    }
    r
}

fn llpow(a: i64, b: i64) -> i64 {
    let mut r = 1;
    for _ in 1..=b {
        r *= a;
    }
    r
}

fn lldsum(mut x: i64) -> i64 {
    let mut r = 0;
    while x != 0 {
        r += x % 10;
        x /= 10;
    }
    r
}

fn lldsumb(mut x: i64, b: i64) -> i64 {
    let mut r = 0;
    while x != 0 {
        r += x % b;
        x /= b;
    }
    r
}

fn llsankaku(x: i64) -> i64 {
    (1 + x) * x / 2
}

fn prarr(arr: &[i64]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, &value) in arr.iter().enumerate() {
        if i > 0 {
            write!(handle, " ").unwrap();
        }
        write!(handle, "{}", value).unwrap();
    }
    writeln!(handle).unwrap();
}

fn search(x: i64, a: &[i64]) -> usize {
    let mut st = 0;
    let mut fi = a.len() as i64 - 1;
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

fn shuffledget(x: &mut [i64], n: usize) {
    let mut rng = rand::thread_rng();
    let mut b: Vec<usize> = (0..n).collect();
    for i in (1..=n).rev() {
        let p = rng.gen_range(0..i);
        b.swap(i - 1, p);
    }
    for i in 0..n {
        x[b[i]] = read_input();
    }
}

fn read_input() -> i64 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

#[derive(Debug)]
struct Sd {
    val: i64,
    node: i64,
}

fn sdsortfnc(a: &Sd, b: &Sd) -> std::cmp::Ordering {
    a.val.cmp(&b.val)
}

fn makepl() -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut fl = vec![false; 2222];
    let mut pl = Vec::new();
    let mut dpl = Vec::new();
    let mut trpl = Vec::new();
    for i in 2..2222 {
        if fl[i] {
            continue;
        }
        pl.push(i as i64);
        dpl.push((i * i) as i64);
        trpl.push((i * i * i) as i64);
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
    let (pl, dpl, trpl) = makepl();
    let n = read_input();
    let mut a = Vec::new();
    let mut r = 0;
    let mut c = 0;
    let mut dat: Vec<Sd> = Vec::new();
    let mut tfl = vec![false; 100005];

    for _ in 0..n {
        let mut t = read_input();
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
        a.push(w);
        c += 1;
    }

    if c == 0 {
        println!("{}", r);
        return;
    }

    a.sort_unstable();
    let mut t = 0;
    let mut w = 1;
    dat.push(Sd { val: a[0], node: 1 });

    for i in 1..c {
        if a[i - 1] != a[i] {
            dat[t].node = w;
            t += 1;
            w = 1;
            dat.push(Sd { val: a[i], node: 1 });
        } else {
            w += 1;
        }
    }
    dat[t].node = w;
    t += 1;
    dat.push(Sd { val: LINF, node: 0 });

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
        let st = search(w, &dat.iter().map(|d| d.val).collect::<Vec<_>>());
        if dat[st].val != w {
            r += dat[i].node;
            continue;
        }
        tfl[st] = true;
        r += llmax(dat[i].node, dat[st].node);
    }
    println!("{}", r);
}
``` 

Make sure to include the `rand` crate in your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8"  # or the latest version
