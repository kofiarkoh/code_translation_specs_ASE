use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string (valid sequence of characters is assumed by the spec).
    // The spec states max length 114514; we don't enforce it beyond reading what's provided.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Preconditions say length is at least 1 character.
    // Interpret "sequence of characters" as the raw input sequence including any newlines/spaces.
    let mut chars = input.chars();
    let first = chars.next().unwrap();

    // Find last character without allocating a separate collection.
    let mut last = first;
    for c in chars {
        last = c;
    }

    if first == last {
        print!("First");
    } else {
        print!("Second");
    }
}
