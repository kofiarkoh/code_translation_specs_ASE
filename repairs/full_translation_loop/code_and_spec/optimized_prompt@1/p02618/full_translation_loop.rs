use std::io::{self, BufRead};
use std::time::{Duration, Instant};
use rand::Rng; // Ensure the rand crate is included in your Cargo.toml

const TYPE: usize = 26;

fn max(D: usize, s: &[[i32; TYPE + 1]; 101]) -> usize {
    let mut max_score = 0;
    let mut max_type = 0;

    for i in 1..=TYPE {
        if s[D][i] > max_score {
            max_score = s[D][i];
            max_type = i;
        }
    }
    max_type
}

fn score_calc(D: usize, s: &[[i32; TYPE + 1]; 101], schedul: &[usize], c: &[i32; TYPE + 1], last: &mut [usize; TYPE + 1]) -> i32 {
    let mut score = 0;

    for i in 1..=D {
        score += s[i][schedul[i]];
        last[schedul[i]] = i;
        for j in 1..=TYPE {
            score -= c[j] * (i as i32 - last[j] as i32); // Cast to i32 for multiplication
        }
    }

    score
}

fn swap<R: Rng>(rng: &mut R, D: usize, s: &[[i32; TYPE + 1]; 101], schedul: &mut [usize], c: &[i32; TYPE + 1], last: &mut [usize; TYPE + 1], ini: &mut i32) {
    let a = rng.gen_range(1..=D); // Generate random index in range
    let b = rng.gen_range(1..=D);

    schedul.swap(a, b);

    last.fill(0);
    let imp = score_calc(D, s, schedul, c, last);

    if imp > *ini {
        *ini = imp;
    } else {
        schedul.swap(a, b); // revert the swap
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let D: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut c = [0; TYPE + 1];
    let mut last = [0; TYPE + 1];
    let mut schedul = vec![0; D + 1];
    let mut s = [[0; TYPE + 1]; 101];

    for i in 1..=TYPE {
        c[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }
    for i in 1..=D {
        let scores: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        s[i][1..=TYPE].copy_from_slice(&scores[0..TYPE]);
    }

    let start = Instant::now();
    for i in 1..=D {
        schedul[i] = max(i, &s);
    }
    last.fill(0);
    let mut ini = score_calc(D, &s, &schedul, &c, &mut last);

    let mut rng = rand::thread_rng(); // Create a random number generator
    while start.elapsed() < Duration::new(1, 990_000_000) {
        swap(&mut rng, D, &s, &mut schedul, &c, &mut last, &mut ini);
    }

    for i in 1..=D {
        println!("{}", schedul[i]);
    }
}
```

Make sure to include the `rand` crate in your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8"  # or the latest version
