// Transliteration of the given Specification into Rust.
// Behavior notes (per Specification):
// - Reads integer triples (year, month, day) from stdin until EOF.
// - Maintains two persistent flags across records:
//     seen_decision: initially false. When a non-"pre-meiji" era is produced, it is set
//                    and thereafter era-detection is inhibited (records may produce no output).
//     print_numbers: controls whether numeric components (era-year, month, day) are printed
//                    after an era label. It persists across records. Initially true.
//   These choices match the narrow interpretation of the Specification and preserve
//   the described stateful behavior and possible "no output" cases.
// - Classification boundaries:
//     pre-meiji: year < 1868
//     meiji:     from 1868 up to 1912-07-29  (inclusive)
//     taisyo:    from 1912-07-30 up to 1926-12-24
//     showa:     from 1926-12-25 up to 1989-01-07
//     heisei:    from 1989-01-08 onward
// - For "pre-meiji" the program prints exactly "pre-meiji\n" and sets print_numbers = false.
// - For the other eras it prints the era label and, if print_numbers is true, prints
//   "<era> <era-year> <month> <day>\n" where era-year = year - base + 1, and sets
//   seen_decision = true and print_numbers = true.
// - The program assumes (as a precondition) that input provides integer triples. If not,
//   behavior is not tightly specified here; we choose the narrow interpretation that
//   input will be valid integer tokens grouped in triples.
//
// This implementation therefore reads all whitespace-separated integer tokens from stdin,
// groups them into triples, and processes each triple in order, preserving the persistent flags.

use std::io::{self, Read};

fn main() {
    // Persistent internal flags across records
    let mut seen_decision: bool = false; // initially false per Specification
    let mut print_numbers: bool = true;  // narrow reasonable interpretation: start printing numbers

    // Read entire stdin to a string and split on whitespace.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit normally with status 0 per Specification semantics.
        return;
    }

    // Parse whitespace-separated tokens as integers.
    let mut tokens = input.split_whitespace();
    loop {
        // Attempt to read three tokens. The Specification assumes triples; leftover tokens are ignored.
        let y_tok = match tokens.next() {
            Some(t) => t,
            None => break, // EOF
        };
        let m_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let d_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse them as 32-bit signed integers. Per Specification we assume integer tokens.
        // If parsing fails, skip this record (narrow interpretation). This choice avoids undefined/stale behavior.
        let year: i32 = match y_tok.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let month: i32 = match m_tok.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let day: i32 = match d_tok.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // If seen_decision is set, era-detection branches are inhibited and the record may produce no output.
        if seen_decision {
            // Inhibited: produce no output and continue to next record.
            continue;
        }

        // Classification according to the fixed sequence of checks and boundaries.
        if year < 1868 {
            // pre-meiji path: print exactly "pre-meiji\n" and set print_numbers = false.
            println!("pre-meiji");
            print_numbers = false;
            // Per Specification, pre-meiji does not set the seen_decision flag in this interpretation.
            continue;
        }

        // Helper closures to decide month/day boundaries more readably.
        let is_before = |y_cmp: i32, m_cmp: i32, d_cmp: i32| -> bool {
            if year < y_cmp {
                true
            } else if year > y_cmp {
                false
            } else {
                // year == y_cmp
                if month < m_cmp {
                    true
                } else if month > m_cmp {
                    false
                } else {
                    day < d_cmp
                }
            }
        };

        // Determine era.
        // Meiji: up to 1912-07-29 inclusive
        if is_before(1912, 7, 30) {
            // meiji
            let era_base = 1868;
            let era_year = year - era_base + 1;
            // Non-pre-meiji path: set seen_decision = true and print_numbers = true.
            seen_decision = true;
            print_numbers = true;
            if print_numbers {
                println!("meiji {} {} {}", era_year, month, day);
            } else {
                println!("meiji");
            }
            continue;
        }

        // Taisho: from 1912-07-30 up to 1926-12-24 inclusive
        if is_before(1926, 12, 25) {
            let era_base = 1912;
            let era_year = year - era_base + 1;
            seen_decision = true;
            print_numbers = true;
            if print_numbers {
                println!("taisyo {} {} {}", era_year, month, day);
            } else {
                println!("taisyo");
            }
            continue;
        }

        // Showa: from 1926-12-25 up to 1989-01-07 inclusive
        if is_before(1989, 1, 8) {
            let era_base = 1926;
            let era_year = year - era_base + 1;
            seen_decision = true;
            print_numbers = true;
            if print_numbers {
                println!("showa {} {} {}", era_year, month, day);
            } else {
                println!("showa");
            }
            continue;
        }

        // Heisei: from 1989-01-08 onward
        {
            let era_base = 1989;
            let era_year = year - era_base + 1;
            seen_decision = true;
            print_numbers = true;
            if print_numbers {
                println!("heisei {} {} {}", era_year, month, day);
            } else {
                println!("heisei");
            }
            continue;
        }
    }

    // Exit normally (drop main).
}
