use std::io::{self, Read};

fn main() {
    // According to the Specification:
    // - We maintain three baseline top values initialized to 0.
    // - We read exactly ten integer tokens from stdin (C int range).
    // - We do not explicitly handle scanf failures; to stay faithful, parsing errors or missing tokens
    //   will cause the program to panic (undefined / unchecked behavior in the Spec).
    //
    // Implementation notes:
    // - Use i32 to represent C int (narrowest reasonable mapping).
    // - Read stdin into a string and split on whitespace to obtain tokens.
    //   (The Spec assumes at least ten integer tokens are available.)
    // - Use unwrap() on token extraction and parse to emulate the Spec's lack of error checking.
    // - Update the top-three with the ordering rule: if x > top1 -> shift down, else if x > top2 -> shift, else if x > top3 -> set.
    //   This preserves duplicates and ensures non-increasing order on output.

    let mut input = String::new();
    // Read all available stdin. Per the Spec, stdin is expected to contain at least ten integers.
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut tokens = input.split_whitespace();

    let mut top1: i32 = 0;
    let mut top2: i32 = 0;
    let mut top3: i32 = 0;

    for _ in 0..10 {
        // Unwrap token and parse without error handling to mirror the Spec's behavior
        // where scanf return value is not checked.
        let tok = tokens.next().unwrap();
        let x: i32 = tok.parse().unwrap();

        if x > top1 {
            top3 = top2;
            top2 = top1;
            top1 = x;
        } else if x > top2 {
            top3 = top2;
            top2 = x;
        } else if x > top3 {
            top3 = x;
        }
    }

    // Print the three values, each on its own line, in descending order.
    println!("{}", top1);
    println!("{}", top2);
    println!("{}", top3);
}
