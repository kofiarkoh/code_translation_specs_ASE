use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

const LIM_PRECOMP: u8 = 16;
const LIM_QUERY: u8 = 4;

// Adjacency list exactly as in the spec/C (undirected, fixed).
const MOVES: [&[u8]; 13] = [
    /* 0  */ &[2],
    /* 1  */ &[2, 5],
    /* 2  */ &[0, 1, 3, 6],
    /* 3  */ &[2, 7],
    /* 4  */ &[5],
    /* 5  */ &[1, 4, 6, 9],
    /* 6  */ &[2, 5, 7, 10],
    /* 7  */ &[3, 6, 8, 11],
    /* 8  */ &[7],
    /* 9  */ &[5, 10],
    /* 10 */ &[6, 9, 11, 12],
    /* 11 */ &[7, 10],
    /* 12 */ &[10],
];

fn get_nibble(s: u64, pos: u8) -> u64 {
    let shift = (12 - pos) * 4;
    (s >> shift) & 0xF
}

fn set_nibble(mut s: u64, pos: u8, val: u64) -> u64 {
    let shift = (12 - pos) * 4;
    s &= !(0xFu64 << shift);
    s |= (val & 0xF) << shift;
    s
}

/// Matches the C `swap`: move nibble at `from` into `to_empty` and clear `from`.
fn swap_into_empty(s: u64, from: u8, to_empty: u8) -> u64 {
    let v = get_nibble(s, from);
    let s = set_nibble(s, from, 0);
    set_nibble(s, to_empty, v)
}

#[derive(Clone, Copy)]
struct Node {
    s: u64,
    b1: u8,
    b2: u8,
    d: u8,
}

/// Precompute exact distances from the goal for all states within 16 moves.
///
/// IMPORTANT: The provided C code seeds the BFS with blank positions (0,12) even though
/// the goal state's zeros are at (11,12). This affects the generated state space and thus
/// the program's observable outputs (e.g., the goal input prints "NA" in the sample).
/// Although the spec describes the goal as having zeros at (11,12), the dataset behavior
/// matches the C program; we therefore follow the C code here to preserve semantics.
fn precompute_from_goal_c_semantics() -> HashMap<u64, u8> {
    let goal: u64 = 0x123456789ab0u64; // exactly as in C
    let (b1, b2) = (0u8, 12u8); // exactly as in C

    let mut dist: HashMap<u64, u8> = HashMap::new();
    dist.insert(goal, 0);

    let mut q: VecDeque<Node> = VecDeque::new();
    q.push_back(Node {
        s: goal,
        b1,
        b2,
        d: 0,
    });

    while let Some(cur) = q.pop_front() {
        if cur.d >= LIM_PRECOMP {
            continue;
        }
        let blanks = [cur.b1, cur.b2];
        for (i, &b) in blanks.iter().enumerate() {
            for &x in MOVES[b as usize] {
                if x == cur.b1 || x == cur.b2 {
                    continue; // never swap empty with empty
                }
                let s2 = swap_into_empty(cur.s, x, b);
                if dist.contains_key(&s2) {
                    continue;
                }
                let nd = cur.d + 1;
                dist.insert(s2, nd);

                let (nb1, nb2) = if i == 0 { (x, cur.b2) } else { (cur.b1, x) };
                q.push_back(Node {
                    s: s2,
                    b1: nb1,
                    b2: nb2,
                    d: nd,
                });
            }
        }
    }

    dist
}

/// From a query state, BFS outward up to 4 moves; if any reached state is in `precomp`,
/// return (steps_from_query + precomp_distance). Otherwise None.
fn query_bounded(
    start_s: u64,
    start_b1: u8,
    start_b2: u8,
    precomp: &HashMap<u64, u8>,
) -> Option<u8> {
    if let Some(&d) = precomp.get(&start_s) {
        return Some(d);
    }

    let mut seen: HashMap<u64, u8> = HashMap::new();
    seen.insert(start_s, 0);

    let mut q: VecDeque<Node> = VecDeque::new();
    q.push_back(Node {
        s: start_s,
        b1: start_b1,
        b2: start_b2,
        d: 0,
    });

    while let Some(cur) = q.pop_front() {
        if cur.d >= LIM_QUERY {
            continue;
        }
        let blanks = [cur.b1, cur.b2];
        for (i, &b) in blanks.iter().enumerate() {
            for &x in MOVES[b as usize] {
                if x == cur.b1 || x == cur.b2 {
                    continue;
                }
                let s2 = swap_into_empty(cur.s, x, b);
                let nd = cur.d + 1;

                if let Some(&goal_d) = precomp.get(&s2) {
                    return Some(nd + goal_d);
                }

                if seen.contains_key(&s2) {
                    continue;
                }
                seen.insert(s2, nd);

                let (nb1, nb2) = if i == 0 { (x, cur.b2) } else { (cur.b1, x) };
                q.push_back(Node {
                    s: s2,
                    b1: nb1,
                    b2: nb2,
                    d: nd,
                });
            }
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let precomp = precompute_from_goal_c_semantics();

    let mut out = String::new();
    loop {
        let first = match it.next() {
            Some(x) => x,
            None => break,
        };
        let p0: i32 = first.parse().unwrap();
        if p0 < 0 {
            break;
        }

        let mut vals = [0u8; 13];
        vals[0] = p0 as u8;
        for i in 1..13 {
            let v: i32 = it.next().unwrap().parse().unwrap();
            vals[i] = v as u8;
        }

        // Pack into 13 nibbles, position 0 first (highest nibble).
        let mut s: u64 = 0;
        let mut b1: i8 = -1;
        let mut b2: i8 = -1;
        for (i, &v) in vals.iter().enumerate() {
            if v == 0 {
                if b1 < 0 {
                    b1 = i as i8;
                } else {
                    b2 = i as i8;
                }
            }
            s = (s << 4) | (v as u64);
        }

        let ans = if let Some(&d) = precomp.get(&s) {
            Some(d)
        } else {
            query_bounded(s, b1 as u8, b2 as u8, &precomp)
        };

        match ans {
            Some(d) => out.push_str(&format!("{d}\n")),
            None => out.push_str("NA\n"),
        }
    }

    print!("{out}");
}
