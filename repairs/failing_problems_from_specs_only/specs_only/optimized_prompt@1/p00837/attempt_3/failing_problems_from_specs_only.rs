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
        let Some(m_tok) = it.next() else { break };
        let m_in: i32 = m_tok.parse().unwrap();
        if m_in <= 0 {
            break;
        }
        let m = m_in as usize;
        let c: usize = it.next().unwrap().parse().unwrap();
        let n: usize = it.next().unwrap().parse().unwrap();

        let hold = m + 1;
        let desk_count = hold + 1; // index 0 unused
        let mut desks: Vec<Vec<usize>> = vec![Vec::new(); desk_count];

        // Keep ids in a small fixed range as implied by the spec (<= 100).
        // This matches the original program's fixed storage assumptions.
        const MAX_ID: usize = 101;
        let mut seen = vec![false; MAX_ID];
        let mut loc_desk = vec![0usize; MAX_ID];

        let mut occs: Vec<Occ> = Vec::new();

        for stu in 0..n {
            let t: usize = it.next().unwrap().parse().unwrap();
            for seq in 0..t {
                let id: usize = it.next().unwrap().parse().unwrap();

                // Under spec preconditions, ids are within range; otherwise behavior is undefined.
                if id < MAX_ID && !seen[id] {
                    seen[id] = true;
                    desks[hold].push(id);
                    loc_desk[id] = hold;
                }

                occs.push(Occ { seq, stu, id });
            }
        }

        occs.sort_by(|a, b| a.seq.cmp(&b.seq).then(a.stu.cmp(&b.stu)));

        let mut cost: i64 = 0;

        for occ in occs {
            let id = occ.id;

            // A) Remove requested book from its current desk.
            let from = loc_desk[id];
            let pos = desks[from]
                .iter()
                .position(|&x| x == id)
                .expect("undefined behavior per spec if requested book not found");
            desks[from].remove(pos);
            cost += from as i64;

            // B) Overflow procedure if primary desk is full before inserting id.
            if desks[1].len() == c {
                // 1) Choose tmp: smallest index in 2..=m that is not full.
                let mut tmp = 0usize;
                for d in 2..=m {
                    if desks[d].len() != c {
                        tmp = d;
                        break;
                    }
                }
                if tmp == 0 {
                    // Undefined behavior per spec; do not invent fallback behavior.
                    panic!("undefined behavior: no non-full auxiliary desk for tmp");
                }

                // Temporarily insert id at back of tmp.
                desks[tmp].push(id);
                cost += tmp as i64;

                // 2) Evict front book from primary desk.
                let id2 = desks[1].remove(0);
                cost += 1;

                // 3) Choose j: smallest index in 2..=m that is not full now.
                // Note: the original logic can select the same desk as tmp; in that case,
                // it performs a special insertion that cancels the temporary insertion.
                let mut j = 0usize;
                for d in 2..=m {
                    if desks[d].len() != c {
                        j = d;
                        break;
                    }
                }
                if j == 0 {
                    panic!("undefined behavior: no non-full auxiliary desk for j");
                }

                if j == tmp {
                    // Insert id2 one slot earlier than the current back position.
                    // Then remove the last element (the temporary id), canceling it.
                    let insert_pos = desks[j].len() - 1;
                    desks[j].insert(insert_pos, id2);
                    let removed = desks[j].pop().unwrap();
                    debug_assert!(removed == id);
                } else {
                    desks[j].push(id2);
                }
                loc_desk[id2] = j;
                cost += (j + tmp) as i64;
            }

            // C) Insert requested book into primary desk.
            desks[1].push(id);
            loc_desk[id] = 1;
            cost += 1;
        }

        outs.push(cost.to_string());
    }

    print!("{}", outs.join("\n"));
}
