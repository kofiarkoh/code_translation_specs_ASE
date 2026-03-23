use std::io::{self, Read};
use std::time::Instant;

const CATS: usize = 26;

fn score(schedule: &[usize], c: &[i64; CATS], s: &[Vec<i64>]) -> i64 {
    let d = schedule.len();
    let mut last = [0i64; CATS]; // "last scheduled" initially treated as day 0
    let mut total: i64 = 0;

    for day in 1..=d {
        let cat = schedule[day - 1]; // 0..25
        total += s[day - 1][cat];
        last[cat] = day as i64;

        // Subtract penalties for all categories based on days since last scheduled.
        // This matches the described per-day accumulation.
        for k in 0..CATS {
            let since = (day as i64) - last[k];
            total -= c[k] * since;
        }
    }
    total
}

struct XorShift64 {
    x: u64,
}
impl XorShift64 {
    fn new(seed: u64) -> Self {
        // Avoid a zero state.
        Self { x: if seed == 0 { 88172645463325252u64 } else { seed } }
    }
    fn next_u64(&mut self) -> u64 {
        // Simple xorshift; randomness quality is not specified, only non-determinism.
        let mut x = self.x;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        self.x = x;
        x
    }
    fn gen_range_usize(&mut self, upper: usize) -> usize {
        // upper must be > 0 per preconditions (days positive).
        (self.next_u64() % (upper as u64)) as usize
    }
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let d: usize = it.next().unwrap().parse().unwrap();

    let mut c = [0i64; CATS];
    for k in 0..CATS {
        c[k] = it.next().unwrap().parse().unwrap();
    }

    let mut s: Vec<Vec<i64>> = Vec::with_capacity(d);
    for _ in 0..d {
        let mut row = Vec::with_capacity(CATS);
        for _ in 0..CATS {
            row.push(it.next().unwrap().parse().unwrap());
        }
        s.push(row);
    }

    // Initialization: for each day independently, choose category with highest satisfaction.
    // Precondition: at least one category has strictly positive satisfaction each day;
    // otherwise behavior is undefined, so we do not add extra handling.
    let mut schedule: Vec<usize> = Vec::with_capacity(d);
    for day in 0..d {
        let mut best_cat = 0usize;
        let mut best_val = s[day][0];
        for k in 1..CATS {
            let v = s[day][k];
            if v > best_val {
                best_val = v;
                best_cat = k;
            }
        }
        schedule.push(best_cat);
    }

    let mut best_score = score(&schedule, &c, &s);

    // Time-bounded improvement phase (~2 seconds).
    let start = Instant::now();
    let time_limit = std::time::Duration::from_secs_f64(2.0);

    // Seed RNG from monotonic time; output is intended to be non-deterministic.
    let seed = start.elapsed().as_nanos() as u64 ^ (d as u64).wrapping_mul(0x9E3779B97F4A7C15);
    let mut rng = XorShift64::new(seed);

    while start.elapsed() < time_limit {
        let i = rng.gen_range_usize(d);
        let mut j = rng.gen_range_usize(d);
        if d > 1 {
            while j == i {
                j = rng.gen_range_usize(d);
            }
        }

        schedule.swap(i, j);
        let new_score = score(&schedule, &c, &s);

        if new_score > best_score {
            best_score = new_score; // best-so-far is non-decreasing; accepted swaps strictly improve
        } else {
            schedule.swap(i, j); // reject swap
        }
    }

    // Output categories numbered 1..26, one per day.
    let mut out = String::new();
    for &cat0 in &schedule {
        out.push_str(&(cat0 + 1).to_string());
        out.push('\n');
    }
    print!("{out}");
}
