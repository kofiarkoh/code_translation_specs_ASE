use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

const TYPE: usize = 26;

// Simple LCG to emulate C's rand()/srand() behavior without unsafe/libc.
// Exact sequence won't match a particular libc, but semantics of usage are preserved.
struct LcgRng {
    state: u32,
}
impl LcgRng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    fn next_u32(&mut self) -> u32 {
        // Common LCG parameters (like ANSI C style). Wrapping to emulate 32-bit overflow.
        self.state = self
            .state
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        self.state
    }
    fn rand_i32(&mut self) -> i32 {
        // Produce a non-negative-ish value similar to many rand() implementations.
        ((self.next_u32() >> 1) as i32)
    }
}

fn max_type(day: usize, s: &Vec<Vec<i32>>) -> usize {
    // Mirrors C: max_score starts at 0, max_type is uninitialized if all s[day][i] <= 0.
    // In typical contest inputs s are non-negative; we keep behavior close by defaulting to 0.
    let mut max_score: i32 = 0;
    let mut max_t: usize = 0;
    for i in 1..=TYPE {
        if s[day][i] > max_score {
            max_score = s[day][i];
            max_t = i;
        }
    }
    max_t
}

fn score_calc(D: usize, s: &Vec<Vec<i32>>, schedul: &Vec<usize>, c: &Vec<i32>, last: &mut Vec<i32>) -> i32 {
    let mut score: i32 = 0;
    for i in 1..=D {
        let t = schedul[i];
        score = score.wrapping_add(s[i][t]);
        last[t] = i as i32;
        for j in 1..=TYPE {
            let penalty = c[j].wrapping_mul((i as i32).wrapping_sub(last[j]));
            score = score.wrapping_sub(penalty);
        }
    }
    score
}

fn do_swap(
    D: usize,
    s: &Vec<Vec<i32>>,
    schedul: &mut Vec<usize>,
    c: &Vec<i32>,
    last: &mut Vec<i32>,
    ini: &mut i32,
    rng: &mut LcgRng,
) {
    let a = (rng.rand_i32().rem_euclid(D as i32) + 1) as usize;
    let b = (rng.rand_i32().rem_euclid(D as i32) + 1) as usize;

    let tmp = schedul[a];
    schedul[a] = schedul[b];
    schedul[b] = tmp;

    for i in 1..=TYPE {
        last[i] = 0;
    }
    let imp = score_calc(D, s, schedul, c, last);

    if imp > *ini {
        *ini = imp;
    } else {
        // revert (matches C logic)
        schedul[b] = schedul[a];
        schedul[a] = tmp;
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let D: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // 1-based arrays to match C indexing
    let mut c = vec![0i32; TYPE + 1];
    for i in 1..=TYPE {
        c[i] = it.next().unwrap().parse().unwrap();
    }

    let mut s = vec![vec![0i32; TYPE + 1]; D + 1];
    for i in 1..=D {
        for j in 1..=TYPE {
            s[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    let mut schedul = vec![0usize; D + 1];
    for i in 1..=D {
        schedul[i] = max_type(i, &s);
    }

    let mut last = vec![0i32; TYPE + 1];
    let mut ini = score_calc(D, &s, &schedul, &c, &mut last);

    // Timing loop: emulate C's clock() based loop with a wall-clock approximation.
    // C uses CPU time; here we use elapsed wall time, which is the closest safe/portable approach.
    let start = std::time::Instant::now();

    // Seed with time(NULL) equivalent (seconds since epoch)
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    let mut rng = LcgRng::new(seed);

    while start.elapsed().as_secs_f64() < 1.990000 {
        do_swap(D, &s, &mut schedul, &c, &mut last, &mut ini, &mut rng);
    }

    // Output
    let mut out = String::new();
    for i in 1..=D {
        out.push_str(&format!("{}\n", schedul[i]));
    }
    print!("{out}");
}
