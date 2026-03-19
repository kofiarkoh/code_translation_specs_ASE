use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Item {
    // delta = attribute2[i] - attribute3[0], guaranteed > 0 for collected items
    delta: i64,
    // count = ceil(attribute1[i] / (attribute2[0] - attribute3[i])) for collected items
    count: i64,
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Fixed capacity constraint is stated but not enforced by spec beyond "within capacity".
    // We still allocate dynamically; behavior for out-of-range n is unspecified.
    let mut a1: Vec<i64> = Vec::with_capacity(n + 1);
    let mut a2: Vec<i64> = Vec::with_capacity(n + 1);
    let mut a3: Vec<i64> = Vec::with_capacity(n + 1);
    let mut a4: Vec<i64> = Vec::with_capacity(n + 1);

    for _ in 0..=n {
        let x1: i64 = it.next().unwrap().parse().unwrap();
        let x2: i64 = it.next().unwrap().parse().unwrap();
        let x3: i64 = it.next().unwrap().parse().unwrap();
        let x4: i64 = it.next().unwrap().parse().unwrap();
        a1.push(x1);
        a2.push(x2);
        a3.push(x3);
        a4.push(x4);
    }

    // Entity 0 provides thresholds/baselines.
    let base_a1_0 = a1[0];
    let base_a2_0 = a2[0];
    let base_a3_0 = a3[0];

    let mut items: Vec<Item> = Vec::new();
    let mut turn: i64 = 0;

    // Feasibility-check state: collect only entities with positive delta.
    for i in 1..=n {
        let delta = a2[i] - base_a3_0;
        if delta <= 0 {
            continue;
        }

        // Infeasibility condition: attribute2[0] <= attribute3[i]
        if base_a2_0 <= a3[i] {
            print!("-1\n");
            return;
        }

        // Divisor derived from entity 0 and i: (attribute2[0] - attribute3[i])
        // Spec assumption: for feasible records, this divisor is strictly positive.
        let div = base_a2_0 - a3[i];
        // Compute count as ceil(attribute1[i] / div) with non-negative integers.
        // Inputs are non-negative; div > 0 here.
        let num = a1[i];
        let count = (num + div - 1) / div;

        items.push(Item { delta, count });
        turn += delta;
    }

    // Deterministic ordering based on ratio involving count and delta.
    // Narrowest reasonable interpretation: sort by increasing (count / delta),
    // using exact cross-multiplication to avoid floating error.
    items.sort_by(|x, y| {
        // Compare x.count/x.delta vs y.count/y.delta:
        // x.count * y.delta ? y.count * x.delta
        let left = (x.count as i128) * (y.delta as i128);
        let right = (y.count as i128) * (x.delta as i128);
        match left.cmp(&right) {
            Ordering::Equal => {
                // Deterministic tie-breaker: keep stable by delta then count.
                // (Spec only requires deterministic ordering.)
                match x.delta.cmp(&y.delta) {
                    Ordering::Equal => x.count.cmp(&y.count),
                    o => o,
                }
            }
            o => o,
        }
    });

    // Aggregation state
    let mut acc: i64 = 0;
    for item in items {
        // Contribution: count * current turn
        // Spec notes overflow is not handled; we use i128 for intermediate and cast back.
        let contrib = (item.count as i128) * (turn as i128);
        acc = (acc as i128 + contrib) as i64;

        // Unsafe condition: accumulated reaches or exceeds attribute1[0]
        if acc >= base_a1_0 {
            print!("-1\n");
            return;
        }

        // Reduce turn by this delta exactly once.
        turn -= item.delta;
    }

    print!("{}\n", acc);
}
