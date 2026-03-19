use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH, Instant};

const TYPE: usize = 26;

// Simple C-like RNG (LCG) to emulate rand()/srand() behavior deterministically within Rust.
// Note: C's rand() is implementation-defined; this is a reasonable stand-in without assuming libc.
struct LcgRand {
    state: u32,
}
impl LcgRand {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    fn next_i32(&mut self) -> i32 {
        // Common LCG parameters (same as many libc implementations).
        self.state = self
            .state
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        // Produce a 31-bit non-negative value like many rand()s.
        ((self.state >> 1) as i32) & 0x7fffffff
    }
    fn rand_mod(&mut self, m: i32) -> i32 {
        // C's rand()%m behavior for positive m.
        self.next_i32() % m
    }
}

fn max_day(d: usize, s: &Vec<Vec<i32>>) -> usize {
    // Mirrors C: max_score starts at 0, max_type is uninitialized if all s[d][i] <= 0.
    // In typical contest inputs s are non-negative; to avoid UB in Rust, default to 1.
    let mut max_score: i32 = 0;
    let mut max_type: usize = 1;
    for i in 1..=TYPE {
        if s[d][i] > max_score {
            max_score = s[d][i];
            max_type = i;
        }
    }
    max_type
}

fn score_calc(d: usize, s: &Vec<Vec<i32>>, schedul: &Vec<usize>, c: &Vec<i32>, last: &mut Vec<usize>) -> i32 {
    let mut score: i32 = 0;
    for i in 1..=d {
        score = score.wrapping_add(s[i][schedul[i]]);
        last[schedul[i]] = i;
        for j in 1..=TYPE {
            let diff = (i - last[j]) as i32;
            score = score.wrapping_sub(c[j].wrapping_mul(diff));
        }
    }
    score
}

fn swap_try(
    d: usize,
    s: &Vec<Vec<i32>>,
    schedul: &mut Vec<usize>,
    c: &Vec<i32>,
    last: &mut Vec<usize>,
    ini: &mut i32,
    rng: &mut LcgRand,
) {
    let a = (rng.rand_mod(d as i32) as usize) + 1;
    let b = (rng.rand_mod(d as i32) as usize) + 1;

    let tmp = schedul[a];
    schedul[a] = schedul[b];
    schedul[b] = tmp;

    for i in 1..=TYPE {
        last[i] = 0;
    }
    let imp = score_calc(d, s, schedul, c, last);

    if imp > *ini {
        *ini = imp;
    } else {
        // revert
        schedul[b] = schedul[a];
        schedul[a] = tmp;
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let d: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // 1-based arrays to match C indexing
    let mut c = vec![0i32; TYPE + 1];
    for i in 1..=TYPE {
        c[i] = it.next().unwrap().parse().unwrap();
    }

    let mut s = vec![vec![0i32; TYPE + 1]; d + 1];
    for i in 1..=d {
        for j in 1..=TYPE {
            s[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    let start = Instant::now();

    // initial schedule: per-day max satisfaction
    let mut schedul = vec![0usize; d + 1];
    for i in 1..=d {
        schedul[i] = max_day(i, &s);
    }

    let mut last = vec![0usize; TYPE + 1];
    let mut ini = score_calc(d, &s, &schedul, &c, &mut last);

    // seed like srand((unsigned int)time(NULL))
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    let mut rng = LcgRand::new(seed);

    // Improve for ~1.99 seconds (as in C: (stop-start)/CLOCKS_PER_SEC < 1.99)
    while start.elapsed().as_secs_f64() < 1.990000 {
        swap_try(d, &s, &mut schedul, &c, &mut last, &mut ini, &mut rng);
    }

    // Output
    let mut out = String::new();
    for i in 1..=d {
        out.push_str(&format!("{}\n", schedul[i]));
    }
    print!("{out}");
}
