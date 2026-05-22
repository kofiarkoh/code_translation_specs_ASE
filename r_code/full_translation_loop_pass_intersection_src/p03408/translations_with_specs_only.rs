use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior is unspecified for malformed/incomplete input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n1: usize = it.next().unwrap().parse().unwrap();
    let mut list1: Vec<String> = Vec::with_capacity(n1);
    for _ in 0..n1 {
        // Spec says each string is a single token and length <= 10; otherwise unspecified.
        list1.push(it.next().unwrap().to_string());
    }

    let n2: usize = it.next().unwrap().parse().unwrap();
    let mut list2: Vec<String> = Vec::with_capacity(n2);
    for _ in 0..n2 {
        list2.push(it.next().unwrap().to_string());
    }

    // Compute:
    // max(0, max over all strings x that appear in the first list of
    //     (count in first list) - (count in second list))
    //
    // The spec's non-functional constraint suggests an O(n1*(n1+n2)) approach
    // with direct comparisons, so we implement that without hashing.
    let mut best: i32 = 0; // invariant: never negative

    for i in 0..list1.len() {
        let x = &list1[i];

        let mut c1: i32 = 0;
        for y in &list1 {
            if y == x {
                c1 += 1;
            }
        }

        let mut c2: i32 = 0;
        for y in &list2 {
            if y == x {
                c2 += 1;
            }
        }

        let net = c1 - c2;
        if net > best {
            best = net;
        }
        if best < 0 {
            // Maintain invariant even if net computation would allow negative.
            best = 0;
        }
    }

    if best < 0 {
        best = 0;
    }

    print!("{}", best);
}
