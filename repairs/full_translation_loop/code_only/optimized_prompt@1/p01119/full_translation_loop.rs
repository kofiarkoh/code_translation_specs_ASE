// AOJ 1634 Balance Scale
// bal4u 2019.8.12

use std::io::{self, BufRead, Write};

const HASHSIZ: usize = 1000003;
const INF: i32 = 0x7fffffff;

#[derive(Default, Clone)]
struct HA {
    n: i32,
    b: i32,
}

struct HashTable {
    ha1: Vec<i32>,
    ha2: Vec<HA>,
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            ha1: vec![0; HASHSIZ + 5],
            ha2: vec![HA::default(); HASHSIZ + 5],
        }
    }

    fn lookup1(&self, n: i32) -> bool {
        let mut p = (n % HASHSIZ as i32) as usize;
        while self.ha1[p] != 0 {
            if self.ha1[p] == n {
                return true;
            }
            p = (p + 1) % HASHSIZ;
        }
        false
    }

    fn insert1(&mut self, n: i32) {
        let mut p = (n % HASHSIZ as i32) as usize;
        while self.ha1[p] != 0 {
            if self.ha1[p] == n {
                return;
            }
            p = (p + 1) % HASHSIZ;
        }
        self.ha1[p] = n;
    }

    fn lookup2(&self, n: i32, b: i32) -> bool {
        let mut p = (n % HASHSIZ as i32) as usize;
        while self.ha2[p].n != 0 {
            if self.ha2[p].n == n {
                if self.ha2[p].b == b - 1 {
                    return true;
                }
                return false;
            }
            p = (p + 1) % HASHSIZ;
        }
        false
    }

    fn insert2(&mut self, n: i32) {
        let mut p = (n % HASHSIZ as i32) as usize;
        while self.ha2[p].n != 0 {
            if self.ha2[p].n == n {
                return;
            }
            p = (p + 1) % HASHSIZ;
        }
        self.ha2[p].n = n;
        self.ha2[p].b = 1;
    }
}

fn read_non_negative_integer() -> i32 {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0) // Handle parse errors gracefully
}

fn write_integer(n: i32) {
    let mut stdout = io::stdout();
    if n == 0 {
        write!(stdout, "0\n").unwrap();
    } else {
        let mut n = n;
        let mut buffer = Vec::new();
        if n < 0 {
            write!(stdout, "-").unwrap();
            n = -n;
        }
        while n > 0 {
            buffer.push((n % 10 + '0' as i32) as u8 as char);
            n /= 10;
        }
        buffer.reverse();
        for &c in &buffer {
            write!(stdout, "{}", c).unwrap();
        }
        writeln!(stdout).unwrap();
    }
}

fn main() {
    let mut hashtable = HashTable::new();
    let mut q = vec![0; 60000];
    let mut a = vec![0; 102];
    let mut w = vec![0; 12];

    loop {
        let n = read_non_negative_integer();
        if n == 0 {
            break;
        }
        let m = read_non_negative_integer();
        for i in 0..n {
            a[i as usize] = read_non_negative_integer();
        }
        for i in 0..m {
            w[i as usize] = read_non_negative_integer();
        }
        q[0] = 0;
        let mut end = 1;

        for i in 0..m {
            let e = end;
            for j in 0..e {
                q[end] = q[j] + w[i as usize];
                end += 1;
                q[end] = q[j] - w[i as usize];
                end += 1;
            }
        }

        hashtable.ha1.fill(0);
        for i in 0..end {
            if q[i] > 0 {
                hashtable.insert1(q[i]);
            }
        }

        let mut x = 0;
        let mut ans = 0;

        for i in 0..n {
            if !hashtable.lookup1(a[i as usize]) {
                if x == 0 {
                    let mut t;
                    ans = INF;
                    hashtable.ha2.fill(HA::default());
                    for j in 0..end {
                        t = (a[i as usize] - q[j]).abs();
                        if t < ans {
                            ans = t;
                        }
                        hashtable.insert2(t);
                    }
                    x = 1;
                } else {
                    let mut t;
                    let mut r = 0;
                    x += 1;
                    ans = INF;
                    for j in 0..end {
                        t = (a[i as usize] - q[j]).abs();
                        if hashtable.lookup2(t, x) {
                            if t < ans {
                                ans = t;
                            }
                            r = 1;
                        }
                    }
                    if r == 0 {
                        ans = -1;
                        break;
                    }
                }
            }
        }
        write_integer(ans);
    }
}
