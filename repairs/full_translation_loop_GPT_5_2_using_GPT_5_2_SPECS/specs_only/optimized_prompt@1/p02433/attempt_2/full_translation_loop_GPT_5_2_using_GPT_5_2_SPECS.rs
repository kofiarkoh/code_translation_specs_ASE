use std::io::{self, Read};
use std::process;

fn main() {
    // The observed tests use numeric command types:
    // 0 x => Insert(x)
    // 1 k => Move(k)
    // 2   => Erase
    //
    // The original spec describes these operations but not the concrete encoding;
    // we implement the narrowest behavior consistent with the failing sample.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Internal end marker is always present as the last element.
    // Cursor is an index into `seq` and may point to the marker.
    let mut seq: Vec<i32> = vec![0]; // marker value is irrelevant
    let mut cursor: usize = 0; // start at marker

    for _ in 0..n {
        let cmd: i32 = it.next().unwrap().parse().unwrap();
        match cmd {
            0 => {
                let val: i32 = it.next().unwrap().parse().unwrap();
                // Insert immediately before cursor; cursor becomes the new element.
                seq.insert(cursor, val);
                // cursor remains the index of the inserted element.
            }
            1 => {
                let offset: i64 = it.next().unwrap().parse().unwrap();
                if offset < 0 {
                    let mut k = -offset;
                    while k > 0 {
                        if cursor == 0 {
                            // Diagnostic message content is not specified; keep minimal.
                            println!("Error");
                            process::exit(1);
                        }
                        cursor -= 1;
                        k -= 1;
                    }
                } else if offset > 0 {
                    let mut k = offset;
                    while k > 0 {
                        // No successor means cursor is at the last element (the marker).
                        if cursor + 1 >= seq.len() {
                            println!("Error");
                            process::exit(1);
                        }
                        cursor += 1;
                        k -= 1;
                    }
                }
            }
            2 => {
                // Erase element at cursor if it is not the marker; cursor becomes successor.
                if cursor + 1 == seq.len() {
                    continue; // at marker: no-op
                }
                seq.remove(cursor);
                // cursor stays at same index, now pointing to successor (or marker).
            }
            _ => {
                // Unrecognized command types: behavior not inferable; ignore.
            }
        }
    }

    // Output stored integers excluding the internal end marker, one per line.
    for &v in &seq[..seq.len().saturating_sub(1)] {
        println!("{v}");
    }
}
