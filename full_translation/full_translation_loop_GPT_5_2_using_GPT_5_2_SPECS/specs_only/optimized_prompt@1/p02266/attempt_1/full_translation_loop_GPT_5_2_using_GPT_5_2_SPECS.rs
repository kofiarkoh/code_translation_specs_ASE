use std::io::{self, Read};

fn main() {
    // Read entire stdin as bytes; treat each byte as one terrain character.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Stack of positions of unmatched descending slopes ('\').
    let mut down_stack: Vec<usize> = Vec::new();

    // Stack of (left_position, area) for formed regions, used to merge nested regions.
    let mut regions: Vec<(usize, usize)> = Vec::new();

    let mut total_area: usize = 0;

    for (pos, ch) in input.chars().enumerate() {
        match ch {
            '\\' => {
                down_stack.push(pos);
            }
            '/' => {
                if let Some(left) = down_stack.pop() {
                    let mut area = pos - left;
                    total_area += area;

                    // Merge any regions that are nested within this new region.
                    // A region is nested if its left boundary is to the right of `left`.
                    while let Some(&(prev_left, prev_area)) = regions.last() {
                        if prev_left > left {
                            area += prev_area;
                            regions.pop();
                        } else {
                            break;
                        }
                    }
                    regions.push((left, area));
                }
            }
            _ => {
                // Flat or unrecognized: advances position implicitly via enumerate; no action.
            }
        }
    }

    // Output:
    // total_area
    // count areas...
    // with a single space before each area, and a trailing newline.
    let mut out = String::new();
    out.push_str(&format!("{}", total_area));
    out.push('\n');

    out.push_str(&format!("{}", regions.len()));
    for &(_, area) in &regions {
        out.push(' ');
        out.push_str(&area.to_string());
    }
    out.push('\n');

    print!("{out}");
}
