use std::io::{self, Read};

fn main() {
    // The specification references a "predefined mapping of cities" and a "specific target city"
    // but does not define either. The narrowest faithful interpretation is that the program's
    // observable behavior cannot be determined without those definitions.
    //
    // To remain compilable and avoid inventing functionality, we implement the I/O loop and
    // termination condition exactly as specified, but we cannot compute "Yes"/"No" meaningfully.
    //
    // Therefore, we only read input lines and stop on a line starting with '#', producing no
    // output because the required mapping/target are unspecified.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    for line in input.lines() {
        let s = line.trim_end_matches(&['\r', '\n'][..]);
        if s.starts_with('#') {
            break;
        }
        // No output: mapping/target city not defined in the specification.
    }
}
