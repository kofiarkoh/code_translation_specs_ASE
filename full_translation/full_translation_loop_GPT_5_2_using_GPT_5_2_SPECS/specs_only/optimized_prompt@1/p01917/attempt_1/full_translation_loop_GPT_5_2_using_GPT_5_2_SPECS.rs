use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Fixed internal capacity per spec assumption; behavior undefined if exceeded.
    // We do not enforce it beyond allocating based on n.
    let mut score_map: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        score_map.push(it.next().unwrap().parse::<i64>().unwrap());
    }

    let mut a = vec![vec![0i64; n]; 3];
    for c in 0..3 {
        for i in 0..n {
            a[c][i] = it.next().unwrap().parse::<i64>().unwrap();
        }
    }

    // Compute total scores given current category values.
    // Category score is score_map[count_strictly_greater_in_that_category].
    fn compute_totals(a: &Vec<Vec<i64>>, score_map: &Vec<i64>, n: usize) -> Vec<i64> {
        let mut total = vec![0i64; n];
        for c in 0..3 {
            for i in 0..n {
                let mut cnt = 0usize;
                let vi = a[c][i];
                for j in 0..n {
                    if a[c][j] > vi {
                        cnt += 1;
                    }
                }
                // Spec precondition: score_map length is at least n, so cnt is in-bounds.
                total[i] = total[i].wrapping_add(score_map[cnt]);
            }
        }
        total
    }

    // Check if first participant has at most 7 participants strictly ahead by total score.
    fn ok(a: &Vec<Vec<i64>>, score_map: &Vec<i64>, n: usize) -> bool {
        let total = compute_totals(a, score_map, n);
        let t0 = total[0];
        let mut ahead = 0usize;
        for i in 0..n {
            if total[i] > t0 {
                ahead += 1;
            }
        }
        ahead <= 7
    }

    // Candidate generation:
    // For each category, consider setting a[cat][0] to each existing value in that category
    // (including its own), and also to (value + 1) for each existing value.
    // This matches a narrow, bounded search consistent with cubic-time behavior and
    // "search constraints" mentioned in the spec.
    let mut best: Option<i64> = None;

    for cat in 0..3 {
        let orig = a[cat][0];

        for j in 0..n {
            let base = a[cat][j];

            for &new_val in &[base, base.wrapping_add(1)] {
                if new_val < orig {
                    continue; // Only nonnegative increase allowed.
                }
                a[cat][0] = new_val;
                if ok(&a, &score_map, n) {
                    let inc = new_val.wrapping_sub(orig);
                    best = Some(match best {
                        Some(b) => if inc < b { inc } else { b },
                        None => inc,
                    });
                }
            }
        }

        a[cat][0] = orig; // Restore before next category.
    }

    match best {
        Some(ans) => {
            println!("{}", ans);
        }
        None => {
            println!("Saiko");
        }
    }
}
