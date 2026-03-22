use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Rect {
    fn normalize(mut self) -> Self {
        if self.x1 > self.x2 {
            std::mem::swap(&mut self.x1, &mut self.x2);
        }
        if self.y1 > self.y2 {
            std::mem::swap(&mut self.y1, &mut self.y2);
        }
        self
    }
}

fn rect_cmp(a: &Rect, b: &Rect) -> Ordering {
    // Deterministic total order based only on normalized corner coordinates.
    // Lexicographic: (x1, y1, x2, y2)
    match a.x1.cmp(&b.x1) {
        Ordering::Equal => {}
        o => return o,
    }
    match a.y1.cmp(&b.y1) {
        Ordering::Equal => {}
        o => return o,
    }
    match a.x2.cmp(&b.x2) {
        Ordering::Equal => {}
        o => return o,
    }
    a.y2.cmp(&b.y2)
}

fn main() {
    // Parser: assumes nonnegative integers; malformed/negative input is unspecified by spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut first_dataset = true;

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        // Fixed capacity is 10 by spec; behavior for N>10 is undefined.
        // We still allocate dynamically to keep Rust safe; semantics for N>10 are not specified.
        let mut rects: Vec<Rect> = Vec::with_capacity(n);
        for _ in 0..n {
            let x1: i64 = it.next().unwrap().parse().unwrap();
            let y1: i64 = it.next().unwrap().parse().unwrap();
            let x2: i64 = it.next().unwrap().parse().unwrap();
            let y2: i64 = it.next().unwrap().parse().unwrap();
            rects.push(Rect { x1, y1, x2, y2 }.normalize());
        }

        // The specification references a "geometric rule based on leftmost position and a
        // recursively derived partitioning of the remaining rectangles", but does not define it.
        // Narrowest deterministic interpretation justified by the spec:
        // - Produce a total order depending only on normalized corner coordinates.
        // - "leftmost position" suggests primary key x1.
        // We therefore rank by lexicographic order of (x1, y1, x2, y2).
        let mut idxs: Vec<usize> = (0..n).collect();
        idxs.sort_by(|&i, &j| rect_cmp(&rects[i], &rects[j]).then_with(|| i.cmp(&j)));
        // Tie-break by input index to ensure a strict total order even for identical rectangles.

        let mut ranks = vec![0usize; n];
        for (rank0, &i) in idxs.iter().enumerate() {
            ranks[i] = rank0 + 1;
        }

        if !first_dataset {
            print!("\n");
        }
        first_dataset = false;

        for i in 0..n {
            println!("{}", ranks[i]);
        }
    }
}
