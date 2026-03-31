use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Occ {
    seq: usize,
    stu: usize,
    id: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outs: Vec<String> = Vec::new();

    loop {
        let Some(ms) = it.next() else { break };
        let m_i: i32 = ms.parse().unwrap();
        if m_i <= 0 {
            break;
        }
        let m = m_i as usize;
        let c: usize = it.next().unwrap().parse().unwrap();
        let n: usize = it.next().unwrap().parse().unwrap();

        let holding = m + 1;

        // desks[1..=m] are capacity-limited; desks[holding] is unlimited initial holding.
        let mut desks: Vec<Vec<usize>> = vec![Vec::new(); holding + 1];

        // Spec implies small fixed range; keep fixed arrays.
        let max_book_id = 100usize;
        let mut seen = vec![false; max_book_id + 1];
        let mut loc_desk = vec![0usize; max_book_id + 1];

        let mut occs: Vec<Occ> = Vec::new();

        for stu in 0..n {
            let t: usize = it.next().unwrap().parse().unwrap();
            for seq in 0..t {
                let id: usize = it.next().unwrap().parse().unwrap();

                if id <= max_book_id && !seen[id] {
                    seen[id] = true;
                    desks[holding].push(id);
                    loc_desk[id] = holding;
                }

                occs.push(Occ { seq, stu, id });
            }
        }

        // Global order: by seq, then by student index.
        occs.sort_by(|a, b| a.seq.cmp(&b.seq).then_with(|| a.stu.cmp(&b.stu)));

        let mut cost: i64 = 0;

        for occ in occs {
            let id = occ.id;

            // A) Remove requested book from its current desk.
            let from = loc_desk[id];
            let pos = desks[from].iter().position(|&x| x == id).unwrap();
            desks[from].remove(pos);
            cost += from as i64;

            // B) Overflow if primary is full before inserting id.
            if desks[1].len() == c {
                // 1) Choose tmp: smallest non-full auxiliary desk.
                let mut tmp = 0usize;
                for d in 2..=m {
                    if desks[d].len() != c {
                        tmp = d;
                        break;
                    }
                }

                // Temporarily insert id into tmp (this affects the subsequent choice of j).
                desks[tmp].push(id);
                cost += tmp as i64;

                // 2) Evict front book from primary.
                let id2 = desks[1].remove(0);
                cost += 1;

                // 3) Choose j: smallest non-full auxiliary desk under current sizes.
                let mut j = 0usize;
                for d in 2..=m {
                    if desks[d].len() != c {
                        j = d;
                        break;
                    }
                }

                // Insert id2: normally at back; if j == tmp, insert one slot earlier than back.
                // Net effect: id is no longer kept in tmp (temporary insertion canceled).
                if j == tmp {
                    // desks[tmp] currently ends with the temporary id.
                    // Insert id2 at len-1, then remove the last element (the temporary id).
                    let insert_at = desks[j].len() - 1;
                    desks[j].insert(insert_at, id2);
                    let removed = desks[j].pop().unwrap();
                    debug_assert!(removed == id);
                } else {
                    desks[j].push(id2);
                    // Remove the temporary id from tmp (cancel it) by popping from its back.
                    let removed = desks[tmp].pop().unwrap();
                    debug_assert!(removed == id);
                }

                if id2 <= max_book_id {
                    loc_desk[id2] = j;
                }

                cost += (j + tmp) as i64;
            }

            // C) Insert requested book into primary.
            desks[1].push(id);
            if id <= max_book_id {
                loc_desk[id] = 1;
            }
            cost += 1;
        }

        outs.push(cost.to_string());
    }

    print!("{}", outs.join("\n"));
}
