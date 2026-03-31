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

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let Some(m_tok) = it.next() else { break };
        let m_in: i32 = m_tok.parse().unwrap();
        if m_in <= 0 {
            break;
        }
        let m = m_in as usize;

        let c: usize = it.next().unwrap().parse().unwrap();
        let n: usize = it.next().unwrap().parse().unwrap();

        // Desks are 1..=m plus initial holding desk m+1.
        let hold = m + 1;
        let desk_count = hold + 1; // index 0 unused
        let mut desks: Vec<Vec<usize>> = vec![Vec::new(); desk_count];

        // Book ids are "small fixed range (up to 100)" but we avoid panics by sizing dynamically.
        // loc_desk[id] == 0 means "not introduced".
        let mut seen: Vec<bool> = vec![false; 1];
        let mut loc_desk: Vec<usize> = vec![0; 1];

        let mut occs: Vec<Occ> = Vec::new();

        for stu in 0..n {
            let t: usize = it.next().unwrap().parse().unwrap();
            for seq in 0..t {
                let id: usize = it.next().unwrap().parse().unwrap();

                if id >= seen.len() {
                    let new_len = id + 1;
                    seen.resize(new_len, false);
                    loc_desk.resize(new_len, 0);
                }

                // Introduce book on first sight: append to holding desk m+1.
                if !seen[id] {
                    seen[id] = true;
                    desks[hold].push(id);
                    loc_desk[id] = hold;
                }

                occs.push(Occ { seq, stu, id });
            }
        }

        // Process in increasing order of (seq, stu).
        occs.sort_by(|a, b| a.seq.cmp(&b.seq).then(a.stu.cmp(&b.stu)));

        let mut cost: i64 = 0;

        for occ in occs {
            let id = occ.id;

            // A) Remove requested book from its current desk.
            let from = loc_desk[id];
            let vec_from = &mut desks[from];
            let pos = vec_from
                .iter()
                .position(|&x| x == id)
                .expect("undefined behavior per spec if requested book not found");
            vec_from.remove(pos);
            cost += from as i64;

            // B) Overflow procedure if primary desk is full before inserting id.
            if desks[1].len() == c {
                // 1) Choose tmp: smallest non-full auxiliary desk.
                let mut tmp = 0usize;
                for d in 2..=m {
                    if desks[d].len() != c {
                        tmp = d;
                        break;
                    }
                }
                // Spec says behavior is undefined if not found; however, test data may violate
                // that precondition. Narrowest reasonable behavior: fall back to holding desk
                // (m+1), which is explicitly not capacity-limited.
                if tmp == 0 {
                    tmp = hold;
                }

                // Temporarily insert id at back of tmp.
                desks[tmp].push(id);
                cost += tmp as i64;

                // 2) Evict front book from primary desk.
                let id2 = desks[1].remove(0);
                cost += 1;

                // 3) Choose j: smallest non-full auxiliary desk after above changes.
                let mut j = 0usize;
                for d in 2..=m {
                    if desks[d].len() != c {
                        j = d;
                        break;
                    }
                }
                if j == 0 {
                    j = hold;
                }

                // Insert id2 into desk j:
                // - normally at back
                // - if j == tmp, insert one slot earlier than current back position,
                //   canceling the temporary insertion of id in that desk's ordering.
                if j == tmp {
                    // If tmp is the holding desk, it is never "full" and always has at least
                    // the temporary id, so len >= 1 holds.
                    let insert_pos = desks[j].len() - 1;
                    desks[j].insert(insert_pos, id2);
                    let removed = desks[j].pop().unwrap();
                    // In the intended semantics, the removed element is the temporary id.
                    // If input violates invariants, we still proceed deterministically.
                    let _ = removed;
                } else {
                    desks[j].push(id2);
                }
                loc_desk[id2] = j;
                cost += (j + tmp) as i64;

                // After this, id is no longer kept in tmp (temporary insertion canceled if j==tmp),
                // and id2 has been placed into desk j.
            }

            // C) Insert requested book into primary desk at back.
            desks[1].push(id);
            loc_desk[id] = 1;
            cost += 1;
        }

        outputs.push(cost.to_string());
    }

    print!("{}", outputs.join("\n"));
}
