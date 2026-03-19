use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // M == 0 special-case outputs.
    if m == 0 {
        let out = match n {
            1 => 0,
            2 => 10,
            3 => 100,
            _ => {
                // Behavior for other N is not inferable; keep narrow and do nothing.
                return;
            }
        };
        print!("{out}");
        return;
    }

    // Track digits; None = unconstrained, Some(d) = constrained to digit d.
    let mut digits: Vec<Option<i32>> = vec![None; n];

    // Leading-zero rule depends ONLY on the first provided constraint pair.
    let first_pos: usize = it.next().unwrap().parse().unwrap();
    let first_digit: i32 = it.next().unwrap().parse().unwrap();
    if n > 1 && first_pos == 1 && first_digit == 0 {
        print!("-1");
        return;
    }

    // Apply first constraint.
    {
        let idx = first_pos - 1;
        digits[idx] = Some(first_digit);
    }

    // Apply remaining constraints; contradictions invalidate.
    let mut valid = true;
    for _ in 1..m {
        let pos: usize = it.next().unwrap().parse().unwrap();
        let dig: i32 = it.next().unwrap().parse().unwrap();
        let idx = pos - 1;
        match digits[idx] {
            None => digits[idx] = Some(dig),
            Some(existing) => {
                if existing != dig {
                    valid = false;
                }
            }
        }
    }

    if !valid {
        print!("-1");
        return;
    }

    // Fill defaults:
    // - Leading digit: if unconstrained and N>1 => 1, else if N==1 => 0.
    // - Other unconstrained digits => 0.
    if n == 1 {
        if digits[0].is_none() {
            digits[0] = Some(0);
        }
    } else {
        if digits[0].is_none() {
            digits[0] = Some(1);
        }
        for i in 1..n {
            if digits[i].is_none() {
                digits[i] = Some(0);
            }
        }
    }

    // Construct number for N in {1,2,3} using place values.
    let result: i32 = match n {
        1 => digits[0].unwrap(),
        2 => digits[0].unwrap() * 10 + digits[1].unwrap(),
        3 => digits[0].unwrap() * 100 + digits[1].unwrap() * 10 + digits[2].unwrap(),
        _ => {
            // Behavior for other N is not inferable; keep narrow and do nothing.
            return;
        }
    };

    print!("{result}");
}
