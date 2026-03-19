use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin; the spec says line-based with a fixed buffer, but the exact
    // truncation behavior is not inferable. Using full read preserves semantics for
    // all inputs that fit within the assumed buffer.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    // Preconditions guarantee this is present and parseable.
    let n: usize = match lines.next() {
        Some(s) => s.trim().parse().unwrap(),
        None => return,
    };

    // Sequence with an internal end marker. We represent the marker as a dedicated
    // element at the end of the Vec. Cursor is an index into this Vec.
    //
    // Invariant: seq is never empty; last element is the end marker.
    // The marker is not printed and is not removed by erase.
    let mut seq: Vec<i32> = vec![0]; // marker value is irrelevant/hidden
    let mut cursor: usize = 0; // start at end marker (only element)

    for _ in 0..n {
        let line = match lines.next() {
            Some(l) => l,
            None => break, // input validity assumed; narrow behavior: stop if missing
        };
        let mut parts = line.split_whitespace();
        let cmd = match parts.next() {
            Some(c) => c,
            None => continue, // empty line: ignore (not specified; narrow)
        };

        // Command recognition is not fully specified; we accept common forms.
        // Unrecognized commands are effectively ignored per spec note.
        match cmd {
            "I" | "i" | "Insert" | "insert" => {
                if let Some(arg) = parts.next() {
                    let val: i32 = arg.parse().unwrap();
                    // Insert immediately before cursor; cursor becomes the new element.
                    seq.insert(cursor, val);
                    // Cursor now points to inserted element at same index.
                }
            }
            "M" | "m" | "Move" | "move" => {
                if let Some(arg) = parts.next() {
                    let offset: i64 = arg.parse().unwrap();
                    if offset < 0 {
                        let mut k = -offset;
                        while k > 0 {
                            if cursor == 0 {
                                // Diagnostic message content is not specified; print a minimal one.
                                println!("Error");
                                process::exit(1);
                            }
                            cursor -= 1;
                            k -= 1;
                        }
                    } else if offset > 0 {
                        let mut k = offset;
                        while k > 0 {
                            // "no successor" means cursor is at the last element (the marker).
                            if cursor + 1 >= seq.len() {
                                println!("Error");
                                process::exit(1);
                            }
                            cursor += 1;
                            k -= 1;
                        }
                    }
                }
            }
            "E" | "e" | "Erase" | "erase" => {
                // If cursor is at end marker (last element), do nothing.
                if cursor + 1 == seq.len() {
                    continue;
                }
                // Remove element at cursor; cursor becomes the next element (same index).
                seq.remove(cursor);
                // If we removed the last non-marker element, cursor now points to marker,
                // which is correct (successor of removed element).
            }
            _ => {
                // Unrecognized command: ignore (input validity assumed).
            }
        }
    }

    // Output all stored integers excluding the internal end marker, one per line.
    // Marker is always the last element.
    if !seq.is_empty() {
        for &v in &seq[..seq.len() - 1] {
            println!("{v}");
        }
    }
}
