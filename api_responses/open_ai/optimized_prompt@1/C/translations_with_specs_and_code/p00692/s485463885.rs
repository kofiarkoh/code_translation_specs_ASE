use std::io::{self, BufRead};

fn remove2(cards: &[u8], out: &mut Vec<u8>, m1: usize, m2: usize) {
    for (i, &card) in cards.iter().enumerate() {
        if i != m1 && i != m2 {
            out.push(card);
        }
    }
}

fn solve(buf: &[u8], depth: usize) -> usize {
    let last = 20 - 2 * depth;
    let mut max_depth = depth;

    for i in 0..last {
        if i < 16 && i + 4 < last && buf[i] == buf[i + 4] {
            let mut mybuf = Vec::new();
            remove2(buf, &mut mybuf, i, i + 4);
            max_depth = max_depth.max(solve(&mybuf, depth + 1));
        }
        if (i % 4) < 3 && i + 1 < last && buf[i] == buf[i + 1] {
            let mut mybuf = Vec::new();
            remove2(buf, &mut mybuf, i, i + 1);
            max_depth = max_depth.max(solve(&mybuf, depth + 1));
        }
        if i < 16 && (i % 4) < 3 && i + 5 < last && buf[i] == buf[i + 5] {
            let mut mybuf = Vec::new();
            remove2(buf, &mut mybuf, i, i + 5);
            max_depth = max_depth.max(solve(&mybuf, depth + 1));
        }
        if i < 16 && (i % 4) > 0 && i + 3 < last && buf[i] == buf[i + 3] {
            let mut mybuf = Vec::new();
            remove2(buf, &mut mybuf, i, i + 3);
            max_depth = max_depth.max(solve(&mybuf, depth + 1));
        }
    }
    max_depth
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let buf: Vec<u8> = line.trim().chars().map(|c| c as u8 - b'0').collect();

        let max_depth = solve(&buf, 0);
        println!("{}", 20 - max_depth * 2);
    }
}
