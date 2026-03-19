use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outs: Vec<String> = Vec::new();

    loop {
        let aux: i64 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if aux <= 0 {
            break;
        }
        let cap: usize = it.next().unwrap().parse::<i64>().unwrap().max(0) as usize;
        let n: usize = it.next().unwrap().parse::<i64>().unwrap().max(0) as usize;

        let desk_count = 1 + aux as usize; // desks are 1..=desk_count, 1 is primary

        let mut reqs: Vec<Vec<i64>> = Vec::with_capacity(n);
        let mut max_len = 0usize;
        for _ in 0..n {
            let k: usize = it.next().unwrap().parse::<i64>().unwrap().max(0) as usize;
            let mut v = Vec::with_capacity(k);
            for _ in 0..k {
                v.push(it.next().unwrap().parse::<i64>().unwrap());
            }
            max_len = max_len.max(v.len());
            reqs.push(v);
        }

        let mut desks: Vec<Vec<i64>> = vec![Vec::new(); desk_count + 1]; // 1-based
        let mut loc: HashMap<i64, usize> = HashMap::new();

        fn remove_book(desks: &mut [Vec<i64>], loc: &mut HashMap<i64, usize>, book: i64) -> usize {
            let d = *loc.get(&book).unwrap_or(&1);
            if let Some(pos) = desks[d].iter().position(|&x| x == book) {
                desks[d].remove(pos);
            }
            loc.insert(book, d);
            d
        }

        fn place_book(desks: &mut [Vec<i64>], loc: &mut HashMap<i64, usize>, desk: usize, book: i64) {
            desks[desk].push(book);
            loc.insert(book, desk);
        }

        fn first_nonfull_aux(desks: &[Vec<i64>], cap: usize, desk_count: usize) -> usize {
            for d in 2..=desk_count {
                if desks[d].len() < cap {
                    return d;
                }
            }
            // Undefined by spec if none exists.
            2
        }

        // Spec doesn't define which book is removed from primary during replacement.
        // Choose the first (oldest) to preserve the relative order of remaining books in the most direct way.
        fn remove_one_from_primary(desks: &mut [Vec<i64>], loc: &mut HashMap<i64, usize>) -> i64 {
            if desks[1].is_empty() {
                0
            } else {
                let b = desks[1].remove(0);
                loc.insert(b, 1);
                b
            }
        }

        let mut total: i128 = 0;

        for pos in 0..max_len {
            for s in 0..n {
                if pos >= reqs[s].len() {
                    continue;
                }
                let book = reqs[s][pos];

                // Remove requested book from its current desk.
                let from = remove_book(&mut desks, &mut loc, book);
                total += from as i128;

                // Replacement check: "primary desk was at capacity immediately before placing the requested book".
                if desks[1].len() >= cap {
                    // Temporarily place requested book onto first available non-full auxiliary desk.
                    let temp = first_nonfull_aux(&desks, cap, desk_count);
                    place_book(&mut desks, &mut loc, temp, book);
                    total += temp as i128;

                    // Remove one book from primary.
                    let moved = remove_one_from_primary(&mut desks, &mut loc);
                    total += 1;

                    // Place removed book onto the first available non-full auxiliary desk.
                    // Adjustment if it is the same auxiliary desk used for temporary placement:
                    // interpret as: the search for the destination desk starts *after* the temp desk,
                    // wrapping around, so it will pick a different desk if possible.
                    let mut dest = first_nonfull_aux(&desks, cap, desk_count);
                    if dest == temp {
                        let mut found = None;
                        for d in (temp + 1)..=desk_count {
                            if desks[d].len() < cap {
                                found = Some(d);
                                break;
                            }
                        }
                        if found.is_none() {
                            for d in 2..temp {
                                if desks[d].len() < cap {
                                    found = Some(d);
                                    break;
                                }
                            }
                        }
                        if let Some(d) = found {
                            dest = d;
                        }
                    }
                    place_book(&mut desks, &mut loc, dest, moved);
                    total += dest as i128;

                    // Remove the temporarily placed requested book from that auxiliary desk.
                    let _ = remove_book(&mut desks, &mut loc, book);
                    total += temp as i128;
                }

                // Place requested book onto primary.
                place_book(&mut desks, &mut loc, 1, book);
                total += 1;
            }
        }

        outs.push(total.to_string());
    }

    print!("{}", outs.join("\n"));
}
