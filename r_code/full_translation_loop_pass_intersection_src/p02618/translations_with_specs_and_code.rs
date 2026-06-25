use std::io::{self, Read};
use std::time::{Duration, Instant};

const TYPE: usize = 26;

fn max_type_for_day(day: usize, s: &Vec<Vec<i32>>) -> usize {
    // Returns the category (1..=26) with the highest satisfaction on `day`.
    // Spec precondition: at least one category has strictly positive satisfaction each day.
    let mut max_score: i32 = 0;
    let mut max_type: usize = 0;
    for t in 1..=TYPE {
        if s[day][t] > max_score {
            max_score = s[day][t];
            max_type = t;
        }
    }
    max_type
}

fn score_calc(d: usize, s: &Vec<Vec<i32>>, sched: &Vec<usize>, c: &Vec<i32>, last: &mut Vec<usize>) -> i32 {
    // Matches C semantics: updates `last` as it computes the score.
    let mut score: i32 = 0;
    for day in 1..=d {
        let t = sched[day];
        score += s[day][t];
        last[t] = day;
        for j in 1..=TYPE {
            let delta = (day - last[j]) as i32;
            score -= c[j] * delta;
        }
    }
    score
}

fn try_swap(
    d: usize,
    s: &Vec<Vec<i32>>,
    sched: &mut Vec<usize>,
    c: &Vec<i32>,
    last: &mut Vec<usize>,
    best_score: &mut i32,
    rng: &mut XorShift64,
) {
    // Randomly swap two days; keep only if strictly improves score.
    let a = (rng.next_u64() as usize % d) + 1;
    let b = (rng.next_u64() as usize % d) + 1;

    let tmp = sched[a];
    sched[a] = sched[b];
    sched[b] = tmp;

    for j in 1..=TYPE {
        last[j] = 0;
    }
    let imp = score_calc(d, s, sched, c, last);

    if imp > *best_score {
        *best_score = imp;
    } else {
        // revert
        sched[b] = sched[a];
        sched[a] = tmp;
    }
}

/// Simple deterministic PRNG (xorshift64) to avoid external crates.
/// C uses `rand()` seeded with `time(NULL)`; here we seed from current time.
struct XorShift64 {
    state: u64,
}
impl XorShift64 {
    fn new(seed: u64) -> Self {
        // Avoid zero state.
        Self { state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed } }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let d: usize = it.next().unwrap().parse().unwrap();

    // 1-based arrays to mirror C indexing
    let mut c = vec![0i32; TYPE + 1];
    for i in 1..=TYPE {
        c[i] = it.next().unwrap().parse().unwrap();
    }

    let mut s = vec![vec![0i32; TYPE + 1]; d + 1];
    for day in 1..=d {
        for t in 1..=TYPE {
            s[day][t] = it.next().unwrap().parse().unwrap();
        }
    }

    // Initial schedule: per-day argmax satisfaction
    let mut sched = vec![0usize; d + 1];
    for day in 1..=d {
        sched[day] = max_type_for_day(day, &s);
    }

    let mut last = vec![0usize; TYPE + 1];
    let mut best_score = score_calc(d, &s, &sched, &c, &mut last);

    // Improvement phase: time-bounded (~1.99 seconds), recomputing full score each attempt.
    let start = Instant::now();
    let time_limit = Duration::from_secs_f64(1.99);

    // Seed RNG from current time (nanoseconds since UNIX_EPOCH if available).
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x123456789ABCDEF0);
    let mut rng = XorShift64::new(seed);

    while start.elapsed() < time_limit {
        try_swap(d, &s, &mut sched, &c, &mut last, &mut best_score, &mut rng);
    }

    // Output schedule, one per line
    let mut out = String::new();
    for day in 1..=d {
        out.push_str(&format!("{}\n", sched[day]));
    }
    print!("{out}");
}
