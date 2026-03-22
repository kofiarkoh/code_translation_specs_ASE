use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct Taxi {
    untin: i32,
    limit: i32,
}

#[derive(Clone, Copy, Default)]
struct Status {
    town: u16,  // C: short
    kaisu: u16, // C: short
}

fn main() {
    // Read all input (scanf-like whitespace handling)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match (it.next(), it.next()) {
        (Some(a), Some(b)) => {
            let n = a.parse::<usize>().unwrap();
            let _k = b.parse::<usize>().unwrap();
            // We'll re-read K below from stored value to preserve structure.
            // But since we already consumed it, store it.
            (n, _k)
        }
        _ => return, // C returns 1; Rust just exits.
    };
    let k: usize = {
        // We stored K in the tuple above.
        // (This is just to keep parsing simple and deterministic.)
        // SAFETY: n tuple exists.
        let mut tmp_it = input.split_whitespace();
        tmp_it.next();
        tmp_it.next().unwrap().parse::<usize>().unwrap()
    };

    // Recreate iterator positioned after N K
    let mut it = input.split_whitespace();
    it.next();
    it.next();

    let mut taxi = vec![Taxi::default(); n];
    for i in 0..n {
        let untin = match it.next() {
            Some(x) => x.parse::<i32>().unwrap(),
            None => return,
        };
        let limit = match it.next() {
            Some(x) => x.parse::<i32>().unwrap(),
            None => return,
        };
        taxi[i] = Taxi { untin, limit };
    }

    // mati adjacency lists
    let mut mati: Vec<Vec<u16>> = vec![Vec::new(); n];
    for _ in 0..k {
        let a = match it.next() {
            Some(x) => x.parse::<i32>().unwrap(),
            None => return,
        };
        let b = match it.next() {
            Some(x) => x.parse::<i32>().unwrap(),
            None => return,
        };
        let a = (a - 1) as usize;
        let b = (b - 1) as usize;
        mati[a].push(b as u16);
        mati[b].push(a as u16);
    }

    // Constants from C arrays: [5000][5001] and heap [5000*5001]
    // We allocate based on N but keep kaisu dimension fixed at 5001 to match C indexing.
    const KAISU_MAX: usize = 5001;

    // status_mincost[town][kaisu] stores cost+1, 0 means unset.
    let mut status_mincost: Vec<Vec<i32>> = vec![vec![0; KAISU_MAX]; n];
    let mut in_heap: Vec<Vec<u8>> = vec![vec![0; KAISU_MAX]; n];

    // Binary heap implemented as a min-heap with explicit array + adjust_heap recursion,
    // mirroring the C code's behavior.
    let mut status_heap: Vec<Status> = Vec::new();

    let read_cost = |stat: Status, status_mincost: &Vec<Vec<i32>>| -> i32 {
        status_mincost[stat.town as usize][stat.kaisu as usize] - 1
    };

    let mut update_cost = |stat: Status, cost: i32, status_mincost: &mut Vec<Vec<i32>>| {
        let cell = &mut status_mincost[stat.town as usize][stat.kaisu as usize];
        if *cell == 0 || (*cell - 1) > cost {
            *cell = cost + 1;
        }
    };

    // adjust_heap as in C: compare children, swap with smallest, recurse; else bubble up to parent.
    fn adjust_heap(
        idx: usize,
        heap: &mut [Status],
        heap_size: usize,
        status_mincost: &Vec<Vec<i32>>,
    ) {
        if heap_size == 0 {
            return;
        }
        let mut min_idx = idx;
        let left = idx * 2 + 1;
        if left < heap_size {
            let cost_min = status_mincost[heap[min_idx].town as usize][heap[min_idx].kaisu as usize] - 1;
            let cost_left = status_mincost[heap[left].town as usize][heap[left].kaisu as usize] - 1;
            if cost_min > cost_left {
                min_idx = left;
            }
            let right = idx * 2 + 2;
            if right < heap_size {
                let cost_min2 = status_mincost[heap[min_idx].town as usize][heap[min_idx].kaisu as usize] - 1;
                let cost_right = status_mincost[heap[right].town as usize][heap[right].kaisu as usize] - 1;
                if cost_min2 > cost_right {
                    min_idx = right;
                }
            }
        }
        if min_idx != idx {
            heap.swap(idx, min_idx);
            adjust_heap(min_idx, heap, heap_size, status_mincost);
        } else if idx > 0 {
            adjust_heap(idx / 2, heap, heap_size, status_mincost);
        }
    }

    let mut insert_heap = |stat: Status,
                           cost: i32,
                           status_heap: &mut Vec<Status>,
                           status_mincost: &mut Vec<Vec<i32>>,
                           in_heap: &mut Vec<Vec<u8>>| {
        let prev_cost = read_cost(stat, status_mincost);
        if prev_cost < 0 || cost < prev_cost {
            update_cost(stat, cost, status_mincost);
            if in_heap[stat.town as usize][stat.kaisu as usize] == 0 {
                in_heap[stat.town as usize][stat.kaisu as usize] = 1;
                status_heap.push(stat);
                let heap_size = status_heap.len();
                adjust_heap(heap_size - 1, status_heap.as_mut_slice(), heap_size, status_mincost);
            }
        }
    };

    let mut get_heap = |status_heap: &mut Vec<Status>,
                        status_mincost: &Vec<Vec<i32>>,
                        in_heap: &mut Vec<Vec<u8>>|
     -> Status {
        let ret = status_heap[0];
        in_heap[ret.town as usize][ret.kaisu as usize] = 0;
        let last = status_heap.pop().unwrap();
        if !status_heap.is_empty() {
            status_heap[0] = last;
            let heap_size = status_heap.len();
            adjust_heap(0, status_heap.as_mut_slice(), heap_size, status_mincost);
        }
        ret
    };

    // haba yuusen tansaku
    let initial = Status { town: 0, kaisu: 0 };
    insert_heap(
        initial,
        0,
        &mut status_heap,
        &mut status_mincost,
        &mut in_heap,
    );

    while !status_heap.is_empty() {
        let cur_status = get_heap(&mut status_heap, &status_mincost, &mut in_heap);
        let cur_cost = read_cost(cur_status, &status_mincost);

        if cur_status.town as usize == n - 1 {
            // mokutekiti ni tuita
            print!("{}\n", cur_cost);
            break;
        }

        // tugi no mati ni iku
        if cur_status.kaisu > 0 {
            let next_kaisu = cur_status.kaisu - 1;
            let town_idx = cur_status.town as usize;
            for &to in &mati[town_idx] {
                let next_status = Status {
                    town: to,
                    kaisu: next_kaisu,
                };
                insert_heap(
                    next_status,
                    cur_cost,
                    &mut status_heap,
                    &mut status_mincost,
                    &mut in_heap,
                );
            }
        }

        // kakin site kono mati de taxi ni noru
        let t = cur_status.town as usize;
        let limit_i32 = taxi[t].limit;
        // C stores into short; emulate truncation/wrap to 16-bit.
        let next_kaisu = (limit_i32 as i16) as u16;
        let next_status = Status {
            town: cur_status.town,
            kaisu: next_kaisu,
        };
        let next_cost = cur_cost.wrapping_add(taxi[t].untin);
        insert_heap(
            next_status,
            next_cost,
            &mut status_heap,
            &mut status_mincost,
            &mut in_heap,
        );
    }

    // C frees adjacency; Rust drops automatically.
}
