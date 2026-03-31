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

        // Use i32 with wrapping to emulate typical C signed overflow behavior if it occurs.
        let mut cost: i32 = 0;

        for occ in occs {
            let id = occ.id;

            // A) Remove requested book from its current desk.
            let from = loc_desk[id];
            let pos = desks[from].iter().position(|&x| x == id).unwrap();
            desks[from].remove(pos);
            cost = cost.wrapping_add(from as i32);

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

                // Temporarily insert id into tmp (affects choice of j).
                desks[tmp].push(id);
                cost = cost.wrapping_add(tmp as i32);

                // 2) Evict front book from primary.
                let id2 = desks[1].remove(0);
                cost = cost.wrapping_add(1);

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
                    let insert_at = desks[j].len() - 1;
                    desks[j].insert(insert_at, id2);
                    let removed = desks[j].pop().unwrap();
                    debug_assert!(removed == id);
                } else {
                    desks[j].push(id2);
                    let removed = desks[tmp].pop().unwrap();
                    debug_assert!(removed == id);
                }

                if id2 <= max_book_id {
                    loc_desk[id2] = j;
                }

                // The spec's cost accounting for step (3) is "Add j + tmp".
                cost = cost.wrapping_add((j + tmp) as i32);
            }

            // C) Insert requested book into primary.
            desks[1].push(id);
            if id <= max_book_id {
                loc_desk[id] = 1;
            }
            cost = cost.wrapping_add(1);
        }

        outs.push(cost.to_string());
    }

    print!("{}", outs.join("\n"));
}
