use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let rows: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // unspecified; choose to do nothing if no input
    };
    let cols: usize = it.next().unwrap().parse().unwrap();

    // Working buffer for one row; memory proportional to columns.
    let mut row: Vec<u8> = vec![0; cols];

    let mut acc: u64 = 0;

    for r in 0..rows {
        // Load row
        for c in 0..cols {
            let v: i64 = it.next().unwrap().parse().unwrap();
            // Values are "intended" binary; for non-binary, spec only guarantees
            // "bitwise toggling semantics". We interpret the stored state as the
            // least significant bit, so complement/toggle is well-defined.
            row[c] = (v as u64 & 1) as u8;
        }

        // Complement for every row after the first (0 <-> 1).
        if r != 0 {
            for x in row.iter_mut() {
                *x ^= 1;
            }
        }

        // Row-processing rule:
        // Scan left-to-right. When encountering a 1 at position i, count an event
        // and toggle adjacent positions within the row (i and i+1). This is a
        // deterministic, finite process and only touches the current row.
        //
        // Boundary behavior: at the last column, only position i exists; toggling
        // "adjacent positions within the row" means toggling i only.
        for i in 0..cols {
            if row[i] == 1 {
                acc += 1;
                row[i] ^= 1;
                if i + 1 < cols {
                    row[i + 1] ^= 1;
                }
            }
        }
    }

    print!("{acc}");
}
