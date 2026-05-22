use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let h = match it.next() {
            Some(v) => match v.parse::<i32>() {
                Ok(n) => n,
                Err(_) => break, // inability to read an integer => terminate
            },
            None => break, // end-of-input
        };
        let m = match it.next() {
            Some(v) => match v.parse::<i32>() {
                Ok(n) => n,
                Err(_) => break,
            },
            None => break, // incomplete triple
        };
        let s = match it.next() {
            Some(v) => match v.parse::<i32>() {
                Ok(n) => n,
                Err(_) => break,
            },
            None => break, // incomplete triple
        };

        // Sentinel: terminate immediately without output.
        if h == -1 && m == -1 && s == -1 {
            break;
        }

        // Use i32 to mirror C int behavior/assumptions from the spec.
        let t: i32 = h * 3600 + m * 60 + s;
        let mut r: i32 = 7200 - t;

        // Decomposition uses C-like integer division/remainder semantics.
        out.push_str(&format!(
            "{:02}:{:02}:{:02}\n",
            r / 3600,
            (r % 3600) / 60,
            r % 60
        ));

        r *= 3;
        out.push_str(&format!(
            "{:02}:{:02}:{:02}\n",
            r / 3600,
            (r % 3600) / 60,
            r % 60
        ));
    }

    print!("{out}");
}
