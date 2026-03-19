use std::io::{self, Read};
use std::time::{Duration, Instant};

const K: usize = 26;

fn compute_score(d: usize, c: &[i64; K], s: &Vec<[i64; K]>, t: &Vec<usize>) -> i64 {
    // Score definition:
    // For each day i (1-based in the spec, but we use 0-based indices):
    //   + satisfaction for scheduled category
    //   - for every category j: c[j] * (days since last scheduled), with last scheduled initially day 0.
    //
    // We implement "last scheduled initially treated as day 0" by tracking last day index in 1..=d,
    // with initial last = 0. For day i (1..=d), "days since last" = i - last[j].
    let mut last = [0i64; K];
    let mut score: i64 = 0;
    for day0 in 0..d {
        let day = (day0 as i64) + 1;
        let cat = t[day0]; // 0..25
        score += s[day0][cat];
        for j in 0..K {
            score -= c[j] * (day - last[j]);
        }
        last[cat] = day;
    }
    score
}

// Simple deterministic PRNG (xorshift64*) to avoid external dependencies.
// Seeded from wall-clock time; spec allows time-based seeding and non-determinism.
#[derive(Clone)]
struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        let mut s = seed;
        if s == 0 {
            s = 0x9E3779B97F4A7C15;
        }
        Self { state: s }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn gen_usize(&mut self, upper: usize) -> usize {
        // upper must be > 0 per preconditions (days positive).
        (self.next_u64() as usize) % upper
    }
}

fn main() {
    // Read all input; no explicit error handling per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let d: usize = it.next().unwrap().parse().unwrap();

    let mut c = [0i64; K];
    for j in 0..K {
        c[j] = it.next().unwrap().parse().unwrap();
    }

    let mut s: Vec<[i64; K]> = Vec::with_capacity(d);
    for _ in 0..d {
        let mut row = [0i64; K];
        for j in 0..K {
            row[j] = it.next().unwrap().parse().unwrap();
        }
        s.push(row);
    }

    // Initial schedule: for each day independently, choose a category with maximal satisfaction.
    // Spec notes undefined behavior if no strictly positive satisfaction exists; we do not guard.
    let mut t: Vec<usize> = vec![0; d];
    for day in 0..d {
        let mut best_j = 0usize;
        let mut best_v = s[day][0];
        for j in 1..K {
            let v = s[day][j];
            if v > best_v {
                best_v = v;
                best_j = j;
            }
        }
        t[day] = best_j;
    }

    let mut cur_score = compute_score(d, &c, &s, &t);

    // Improvement phase: time-bounded to approximately 1.99 seconds.
    // Spec says CPU time; we use wall-clock Instant as the narrowest portable interpretation in Rust std.
    let start = Instant::now();
    let limit = Duration::from_micros(1_990_000);

    let seed = start.elapsed().as_nanos() as u64
        ^ (Instant::now().elapsed().as_nanos() as u64)
        ^ (d as u64).wrapping_mul(0x9E3779B97F4A7C15);
    let mut rng = Rng::new(seed);

    while start.elapsed() < limit {
        let a = rng.gen_usize(d);
        let b = rng.gen_usize(d);
        if a == b {
            continue;
        }

        t.swap(a, b);
        let new_score = compute_score(d, &c, &s, &t);
        if new_score > cur_score {
            cur_score = new_score;
        } else {
            t.swap(a, b); // revert
        }
    }

    // Output schedule as 1..26, one per line.
    let mut out = String::new();
    for day in 0..d {
        out.push_str(&(t[day] + 1).to_string());
        out.push('\n');
    }
    print!("{out}");
}
