use std::io::{self, Read};

fn main() {
    // Read entire stdin; undefined behavior on malformed/missing input is acceptable per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let query_year: i32 = it.next().unwrap().parse().unwrap();

    // Fixed storage capacity implied by spec (at most 49 records).
    // We don't rely on storing all records for correctness, but we keep bounded storage to match
    // the non-functional constraint.
    const MAX_RECORDS: usize = 49;
    const MAX_NAME_LEN: usize = 31;

    // Default name is predefined by the program; the spec does not provide its literal value.
    // Narrowest reasonable interpretation: use a fixed constant token.
    let default_name: &str = "Unknown";
    let mut selected = default_name.to_string();

    // Bounded storage (not required for selection logic, but matches fixed-memory constraint).
    let mut _years: [i32; MAX_RECORDS] = [0; MAX_RECORDS];
    let mut _names: [String; MAX_RECORDS] = std::array::from_fn(|_| String::new());

    for i in 0..n {
        let year: i32 = it.next().unwrap().parse().unwrap();
        let name = it.next().unwrap().to_string();

        // Name length constraint violations are undefined per spec; we do not enforce.
        // Still, keep behavior predictable by truncating to the fixed maximum if longer.
        let stored_name = if name.len() > MAX_NAME_LEN {
            name.chars().take(MAX_NAME_LEN).collect::<String>()
        } else {
            name
        };

        if i < MAX_RECORDS {
            _years[i] = year;
            _names[i] = stored_name.clone();
        }

        if year <= query_year {
            selected = stored_name;
        }
    }

    println!("{selected}");
}
