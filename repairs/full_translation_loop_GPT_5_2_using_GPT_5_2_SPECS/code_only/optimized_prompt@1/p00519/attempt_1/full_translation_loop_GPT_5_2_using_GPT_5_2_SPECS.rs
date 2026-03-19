use std::cmp::min;
use std::io::{self, Read};

#[derive(Copy, Clone, Default)]
struct Taxi {
    untin: i32,
    limit: i32,
}

#[derive(Default)]
struct Mati {
    loads: Vec<i32>,
}

#[derive(Copy, Clone, Default)]
struct Status {
    town: usize,
    kaisu: usize,
}

const MAX_N: usize = 5000;
const MAX_KAISU: usize = 5001;
const HEAP_CAP: usize = MAX_N * MAX_KAISU;

static mut HEAP_SIZE: usize = 0;
static mut STATUS_HEAP: [Status; HEAP_CAP] = [Status { town: 0, kaisu: 0 }; HEAP_CAP];
// C uses 0 as "unset", otherwise stores cost+1.
static mut STATUS_MINCOST: [[i32; MAX_KAISU]; MAX_N] = [[0; MAX_KAISU]; MAX_N];
static mut IN_HEAP: [[u8; MAX_KAISU]; MAX_N] = [[0; MAX_KAISU]; MAX_N];

fn read_cost(stat: Status) -> i32 {
    unsafe { STATUS_MINCOST[stat.town][stat.kaisu] - 1 }
}

fn update_cost(stat: Status, cost: i32) {
    unsafe {
        let cell = &mut STATUS_MINCOST[stat.town][stat.kaisu];
        if *cell == 0 || (*cell - 1) > cost {
            *cell = cost + 1;
        }
    }
}

fn adjust_heap(idx: usize) {
    unsafe {
        let mut min_idx = idx;
        if idx * 2 + 1 < HEAP_SIZE {
            if read_cost(STATUS_HEAP[min_idx]) > read_cost(STATUS_HEAP[idx * 2 + 1]) {
                min_idx = idx * 2 + 1;
            }
            if idx * 2 + 2 < HEAP_SIZE {
                if read_cost(STATUS_HEAP[min_idx]) > read_cost(STATUS_HEAP[idx * 2 + 2]) {
                    min_idx = idx * 2 + 2;
                }
            }
        }
        if min_idx != idx {
            let tmp = STATUS_HEAP[idx];
            STATUS_HEAP[idx] = STATUS_HEAP[min_idx];
            STATUS_HEAP[min_idx] = tmp;
            adjust_heap(min_idx);
        } else if idx > 0 {
            adjust_heap(idx / 2);
        }
    }
}

fn insert_heap(stat: Status, cost: i32) {
    let prev_cost = read_cost(stat);
    if prev_cost < 0 || cost < prev_cost {
        update_cost(stat, cost);
        unsafe {
            if IN_HEAP[stat.town][stat.kaisu] == 0 {
                IN_HEAP[stat.town][stat.kaisu] = 1;
                STATUS_HEAP[HEAP_SIZE] = stat;
                adjust_heap(HEAP_SIZE);
                HEAP_SIZE += 1;
            }
        }
    }
}

fn get_heap() -> Status {
    unsafe {
        let ret = STATUS_HEAP[0];
        IN_HEAP[ret.town][ret.kaisu] = 0;
        HEAP_SIZE -= 1;
        STATUS_HEAP[0] = STATUS_HEAP[HEAP_SIZE];
        adjust_heap(0);
        ret
    }
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let k: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut taxi = vec![Taxi::default(); n];
    for i in 0..n {
        let untin: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => return,
        };
        let limit: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => return,
        };
        taxi[i] = Taxi { untin, limit };
    }

    let mut mati = vec![Mati::default(); n];
    for _ in 0..k {
        let mut a: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => return,
        };
        let mut b: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => return,
        };
        a -= 1;
        b -= 1;
        if a >= 0 && b >= 0 {
            let au = a as usize;
            let bu = b as usize;
            mati[au].loads.push(b);
            mati[bu].loads.push(a);
        }
    }

    // "haba yuusen tansaku" (Dijkstra-like with state (town, remaining rides))
    let initial = Status { town: 0, kaisu: 0 };
    insert_heap(initial, 0);

    while unsafe { HEAP_SIZE } > 0 {
        let cur_status = get_heap();
        let cur_cost = read_cost(cur_status);

        if cur_status.town == n.saturating_sub(1) {
            // reached destination
            print!("{cur_cost}\n");
            break;
        }

        // Move to adjacent towns if we have remaining "kaisu"
        if cur_status.kaisu > 0 {
            let next_kaisu = cur_status.kaisu - 1;
            for &to in &mati[cur_status.town].loads {
                if to >= 0 {
                    let next_status = Status {
                        town: to as usize,
                        kaisu: next_kaisu,
                    };
                    insert_heap(next_status, cur_cost);
                }
            }
        }

        // Pay and take taxi at current town
        // C stores kaisu as short; here we clamp to array bounds to avoid OOB.
        // This matches intended behavior for valid inputs (limits within 0..=5000).
        let lim = taxi[cur_status.town].limit;
        let next_kaisu = if lim <= 0 {
            0usize
        } else {
            min(lim as usize, MAX_KAISU - 1)
        };
        let next_status = Status {
            town: cur_status.town,
            kaisu: next_kaisu,
        };
        insert_heap(next_status, cur_cost + taxi[cur_status.town].untin);
    }
}
