/*
  AOJ 1110
  Title: Patience
  @kankichi573
*/

use std::io::{self, BufRead};
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_CARDS: usize = 20;
static MAX_DEPTH: AtomicUsize = AtomicUsize::new(0);

fn remove2(cards: &[u8; MAX_CARDS], out: &mut [u8; MAX_CARDS], m1: usize, m2: usize) {
    let mut j = 0;
    for i in 0..MAX_CARDS {
        if i != m1 && i != m2 {
            out[j] = cards[i];
            j += 1;
        }
    }
}

fn dump(buf: &[u8; MAX_CARDS], depth: usize) {
    for i in 0..5 {
        for j in 0..4 {
            if i * 4 + j >= MAX_CARDS - depth * 2 {
                break;
            }
            print!("{}|", buf[i * 4 + j]);
        }
        if i < 4 {
            println!();
        }
    }
    println!("------- [{}]", depth);
}

fn solve(buf: &[u8; MAX_CARDS], depth: usize) {
    let last = MAX_CARDS - 2 * depth;
    let mut mybuf = [[0u8; MAX_CARDS]; 4];

    // Uncomment for debugging
    // dump(buf, depth);

    for i in 0..last {
        if i < 16 && i + 4 < last && buf[i] == buf[i + 4] {
            remove2(buf, &mut mybuf[0], i, i + 4);
            solve(&mybuf[0], depth + 1);
        }
        if (i % 4) < 3 && i + 1 < last && buf[i] == buf[i + 1] {
            remove2(buf, &mut mybuf[1], i, i + 1);
            solve(&mybuf[1], depth + 1);
        }
        if i < 16 && (i % 4) < 3 && i + 5 < last && buf[i] == buf[i + 5] {
            remove2(buf, &mut mybuf[2], i, i + 5);
            solve(&mybuf[2], depth + 1);
        }
        if i < 16 && (i % 4) > 0 && i + 3 < last && buf[i] == buf[i + 3] {
            remove2(buf, &mut mybuf[3], i, i + 3);
            solve(&mybuf[3], depth + 1);
        }
    }
    MAX_DEPTH.store(MAX_DEPTH.load(Ordering::SeqCst).max(depth), Ordering::SeqCst);
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..n {
        let mut buf = [0u8; MAX_CARDS];
        for j in 0..MAX_CARDS {
            let c: char = lines.next().unwrap().unwrap().trim().chars().next().unwrap();
            buf[j] = c as u8 - b'0';
        }
        MAX_DEPTH.store(0, Ordering::SeqCst);
        solve(&buf, 0);
        println!("{}", MAX_CARDS - MAX_DEPTH.load(Ordering::SeqCst) * 2);
    }
}
