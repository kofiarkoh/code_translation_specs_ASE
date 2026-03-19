use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // Read all input as integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let aux_desks: i64 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if aux_desks <= 0 {
            break;
        }
        let cap_i64: i64 = it.next().unwrap().parse().unwrap();
        let capacity: usize = if cap_i64 < 0 { 0 } else { cap_i64 as usize };
        let students_i64: i64 = it.next().unwrap().parse().unwrap();
        let students: usize = if students_i64 < 0 { 0 } else { students_i64 as usize };

        // Desk indices are 1..=D where 1 is primary, 2.. are auxiliary.
        let desk_count: usize = 1 + (aux_desks as usize);

        // Read per-student request lists.
        let mut reqs: Vec<Vec<i64>> = Vec::with_capacity(students);
        let mut max_len: usize = 0;
        for _ in 0..students {
            let k_i64: i64 = it.next().unwrap().parse().unwrap();
            let k: usize = if k_i64 < 0 { 0 } else { k_i64 as usize };
            let mut v = Vec::with_capacity(k);
            for _ in 0..k {
                let b: i64 = it.next().unwrap().parse().unwrap();
                v.push(b);
            }
            if v.len() > max_len {
                max_len = v.len();
            }
            reqs.push(v);
        }

        // Desks store books in order; removal compacts without reordering survivors.
        let mut desks: Vec<Vec<i64>> = vec![Vec::new(); desk_count + 1]; // 1-based
        // Track each book's current desk index.
        let mut book_loc: HashMap<i64, usize> = HashMap::new();

        // Helper: remove a book from its current desk, returning the desk index it was removed from.
        let mut remove_book = |desks: &mut [Vec<i64>], book_loc: &mut HashMap<i64, usize>, book: i64| -> usize {
            let d = *book_loc.get(&book).unwrap_or(&1);
            if let Some(pos) = desks[d].iter().position(|&x| x == book) {
                desks[d].remove(pos);
            }
            book_loc.insert(book, d); // keep mapping consistent even if it was absent
            d
        };

        // Helper: place a book onto a desk (append to preserve relative order of existing books).
        let mut place_book = |desks: &mut [Vec<i64>], book_loc: &mut HashMap<i64, usize>, desk: usize, book: i64| {
            desks[desk].push(book);
            book_loc.insert(book, desk);
        };

        // Helper: find first available non-full auxiliary desk (2..=desk_count).
        let mut first_nonfull_aux = |desks: &[Vec<i64>]| -> usize {
            for d in 2..=desk_count {
                if desks[d].len() < capacity {
                    return d;
                }
            }
            // Specification: behavior undefined if none exists; we choose to return 2 to keep code total.
            2
        };

        // Helper: remove one book from primary desk.
        // Specification does not define which book; choose the narrowest deterministic interpretation:
        // remove the first (oldest) book to preserve relative order of remaining books.
        let mut remove_one_from_primary = |desks: &mut [Vec<i64>], book_loc: &mut HashMap<i64, usize>| -> i64 {
            if desks[1].is_empty() {
                // Undefined by spec; return a dummy value.
                0
            } else {
                let b = desks[1].remove(0);
                book_loc.insert(b, 1);
                b
            }
        };

        let mut total_cost: i128 = 0;

        // Process requests in global order: increasing position within student, tie by student index.
        for pos in 0..max_len {
            for s in 0..students {
                if pos >= reqs[s].len() {
                    continue;
                }
                let book = reqs[s][pos];

                // Remove requested book from its current desk.
                let from_desk = remove_book(&mut desks, &mut book_loc, book);
                total_cost += from_desk as i128;

                // If primary is at capacity immediately before placing requested book, do replacement.
                if desks[1].len() >= capacity {
                    // Temporarily place requested book onto first available non-full auxiliary desk.
                    let temp_desk = first_nonfull_aux(&desks);
                    place_book(&mut desks, &mut book_loc, temp_desk, book);
                    total_cost += temp_desk as i128;

                    // Remove one book from primary.
                    let moved = remove_one_from_primary(&mut desks, &mut book_loc);
                    total_cost += 1; // removing from primary costs index 1

                    // Place that removed book onto first available non-full auxiliary desk.
                    // If it is the same desk used for temporary placement, apply the "specific adjustment":
                    // interpret as reusing the same desk without re-searching, but still charging that desk's index once.
                    let mut dest_desk = first_nonfull_aux(&desks);
                    if dest_desk == temp_desk {
                        // Adjustment: keep it as temp_desk (no extra behavior beyond this equality handling).
                        dest_desk = temp_desk;
                    }
                    place_book(&mut desks, &mut book_loc, dest_desk, moved);
                    total_cost += dest_desk as i128;

                    // Remove the temporarily placed requested book from that auxiliary desk.
                    // (It must be on temp_desk by construction.)
                    let _ = remove_book(&mut desks, &mut book_loc, book);
                    total_cost += temp_desk as i128;
                }

                // Place requested book onto primary desk.
                place_book(&mut desks, &mut book_loc, 1, book);
                total_cost += 1;
            }
        }

        outputs.push(total_cost.to_string());
    }

    print!("{}", outputs.join("\n"));
}
