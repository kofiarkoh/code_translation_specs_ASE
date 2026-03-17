use std::io::{self, BufRead};

fn max(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

const MAX_V: usize = 100010;
const MAX_E: usize = 200010;
const INF: i32 = 1_000_000_000;

static mut Q: [usize; 1_000_000] = [0; 1_000_000];
static mut C: [i32; MAX_V] = [0; MAX_V];
static mut TA: [i32; MAX_V] = [-1; MAX_V];
static mut NT: [i32; MAX_E] = [0; MAX_E]; // Changed from usize to i32
static mut FR: [usize; MAX_E] = [0; MAX_E];
static mut TO: [usize; MAX_E] = [0; MAX_E];
static mut CO: [i32; MAX_E] = [0; MAX_E];
static mut F: [i32; MAX_V] = [0; MAX_V];

fn dfs(v: usize, e: usize, s: usize, d: i32) -> i32 {
    let mut max = d;
    let mut i = unsafe { TA[s] };
    
    while i != -1 {
        if unsafe { F[TO[i as usize]] } != 0 {
            i = unsafe { NT[i as usize] };
            continue;
        }
        
        unsafe { F[TO[i as usize]] = 1; }
        unsafe { C[TO[i as usize]] = C[s] + CO[i as usize]; }
        max = max(max, dfs(v, e, unsafe { TO[i as usize] }, d + CO[i as usize]));
        i = unsafe { NT[i as usize] };
    }
    
    max
}

fn bfs(v: usize, e: usize, s: usize, g: &mut usize) -> i32 {
    let mut t = 0;
    let mut r = 1;
    let mut max = 0;

    unsafe {
        for i in 0..v {
            C[i] = INF;
        }
        C[Q[0]] = s as i32; // Fixed assignment

        while t < r {
            let current = Q[t];
            let mut i = TA[current as usize];

            while i != -1 {
                if C[TO[i as usize]] > C[FR[i as usize]] + CO[i as usize] {
                    C[TO[i as usize]] = C[FR[i as usize]] + CO[i as usize];
                    if max < C[TO[i as usize]] {
                        max = C[TO[i as usize]];
                        *g = TO[i as usize];
                    }
                    Q[r] = TO[i as usize];
                    r += 1;
                }
                i = NT[i as usize];
            }
            t += 1;
        }
    }
    
    max
}

fn tyokkei(v: usize, e: usize, a: &[usize], b: &[usize], c: &[i32]) -> i32 {
    let mut max = 0;
    let mut g = 0;

    unsafe {
        for i in 0..v {
            TA[i] = -1;
            C[i] = INF;
            F[i] = 0;
        }
        C[0] = 0;
        F[0] = 1;

        for i in 0..e {
            NT[i] = TA[a[i]]; // No change needed
            FR[i] = a[i];
            TO[i] = b[i];
            CO[i] = c[i];
            TA[a[i]] = i as i32;
            NT[i + e] = TA[b[i]];
            FR[i + e] = b[i];
            TO[i + e] = a[i];
            CO[i + e] = c[i];
            TA[b[i]] = (i + e) as i32;
        }

        max = dfs(v, e * 2, 0, 0);
        for i in 0..v {
            if max == C[i] {
                g = i;
            }
            C[i] = INF;
            F[i] = 0;
        }
        C[g] = 0;
        F[g] = 1;
        dfs(v, e * 2, g, 0)
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n - 1];
    let mut b = vec![0; n - 1];
    let mut c = vec![0; n - 1];

    for i in 0..n - 1 {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = nums[0] as usize;
        b[i] = nums[1] as usize;
        c[i] = nums[2];
    }

    println!("{}", tyokkei(n, n - 1, &a, &b, &c));
}
