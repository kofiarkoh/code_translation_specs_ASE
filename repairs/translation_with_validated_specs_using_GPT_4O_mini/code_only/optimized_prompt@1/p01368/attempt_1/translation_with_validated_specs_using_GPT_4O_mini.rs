// AOJ 2251 Merry Christmas
// 2018.3.6 bal4u

use std::io::{self, Read};

const MAX: usize = 2002;
const INF: i32 = 0x1010;

let mut hi = [0; MAX];
let mut to = [[0; MAX]; MAX];
let mut seen = [0; MAX];
let mut match_arr = [-1; MAX];

fn bpm(u: usize, hi: &[usize], to: &[[usize; MAX]; MAX], match_arr: &mut [i32; MAX], seen: &mut [u8; MAX]) -> bool {
    for i in 0..hi[u] {
        let v = to[u][i];
        if seen[v] != 0 {
            continue;
        }
        seen[v] = 1;
        if match_arr[v] < 0 || bpm(match_arr[v] as usize, hi, to, match_arr, seen) {
            match_arr[u] = v as i32;
            match_arr[v] = u as i32;
            return true;
        }
    }
    false
}

fn in_number<R: Read>(reader: &mut R) -> i32 {
    let mut n = 0;
    let mut c = reader.bytes().next().unwrap().unwrap();
    while c >= b'0' {
        n = (n << 3) + (n << 1) + (c - b'0') as i32;
        c = reader.bytes().next().unwrap().unwrap();
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut dist = [[INF; 102]; 102];
    let mut p = [0; 1002];
    let mut t = [0; 1002];

    loop {
        let n = in_number(&mut reader);
        if n == 0 {
            break;
        }
        let m = in_number(&mut reader);
        let l = in_number(&mut reader);
        
        for k in 0..n {
            dist[k][k] = 0;
        }
        
        for _ in 0..m {
            let u = in_number(&mut reader);
            let v = in_number(&mut reader);
            let d = in_number(&mut reader);
            dist[u][v] = d;
            dist[v][u] = d;
        }
        
        for k in 0..n {
            for u in 0..n {
                if dist[u][k] != INF {
                    for v in 0..n {
                        if dist[k][v] != INF {
                            let d = dist[u][k] + dist[k][v];
                            if dist[u][v] > d {
                                dist[u][v] = d;
                            }
                        }
                    }
                }
            }
        }

        hi.fill(0);
        for k in 0..l {
            p[k] = in_number(&mut reader);
            t[k] = in_number(&mut reader);
        }
        
        for u in 0..l {
            for v in 0..l {
                if u != v {
                    if dist[p[u]][p[v]] <= t[v] - t[u] {
                        to[u][hi[u]] = v + l;
                        hi[u] += 1;
                    }
                }
            }
        }

        match_arr.fill(-1);
        let mut k = l;
        for u in 0..l {
            seen.fill(0);
            if bpm(u, &hi, &to, &mut match_arr, &mut seen) {
                k -= 1;
            }
        }
        println!("{}", k);
    }
}
