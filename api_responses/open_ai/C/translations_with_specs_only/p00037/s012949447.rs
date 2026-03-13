use std::io::{self, Read};

fn main() {
    // Direction indices and bitmask mapping:
    // dir_idx: 0=U, 1=R, 2=D, 3=L
    // bit values: U=1 (1<<0), R=2 (1<<1), D=4 (1<<2), L=8 (1<<3)
    const U: usize = 0;
    const R: usize = 1;
    const D: usize = 2;
    const L: usize = 3;

    // Read all stdin and split on whitespace similar to scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // The specification assumes exactly 9 tokens are provided; we follow that assumption.
    // If fewer tokens are present, we simply do nothing (narrowest reasonable behavior).
    if tokens.len() < 9 {
        return;
    }

    // adjacency masks for 5x5 intersections
    let mut f = [[0u8; 5]; 5];

    // Process horizontal tokens at indices 0,2,4,6,8 (five tokens), each provides 4 chars:
    // token for row r: characters j=0..3 correspond to edge between (r,j) and (r,j+1)
    for r in 0..5 {
        let tok = tokens[2 * r];
        let bytes = tok.as_bytes();
        // The spec assumes at least 4 chars; indexing will panic if not, which mirrors undefined behavior.
        for j in 0..4 {
            if bytes[j] == b'1' {
                // set right on (r,j) and left on (r,j+1)
                f[r][j] |= 1 << R;
                f[r][j + 1] |= 1 << L;
            }
        }
    }

    // Process vertical tokens at indices 1,3,5,7 (four tokens), each provides 5 chars:
    // token for gap g (between rows g and g+1): characters c=0..4 correspond to column
    for g in 0..4 {
        let tok = tokens[2 * g + 1];
        let bytes = tok.as_bytes();
        // The spec assumes at least 5 chars.
        for c in 0..5 {
            if bytes[c] == b'1' {
                // set down on (g,c) and up on (g+1,c)
                f[g][c] |= 1 << D;
                f[g + 1][c] |= 1 << U;
            }
        }
    }

    // Traversal state
    let mut y: i32 = 0; // row
    let mut x: i32 = 1; // column
    let mut cur_dir: usize = R; // initial orientation Right

    let mut output = String::new();
    output.push('R'); // first character always 'R' indicating initial orientation

    loop {
        // Find next outgoing open edge by clockwise-scanning starting from current orientation.
        // Clockwise order of dir indices: U(0),R(1),D(2),L(3). Starting at cur_dir and adding 0..3.
        let mut chosen: Option<usize> = None;
        // Current cell indices as usize (they should be within 0..4 per preconditions)
        let cy = y as usize;
        let cx = x as usize;
        let cell_mask = f[cy][cx];

        for k in 0..4 {
            let d = (cur_dir + k) % 4;
            let bit = 1u8 << d;
            if (cell_mask & bit) != 0 {
                chosen = Some(d);
                break;
            }
        }

        // If no outgoing edge found, behavior is unspecified by the original spec.
        // To avoid infinite loop, we stop traversal here.
        let dir = match chosen {
            Some(d) => d,
            None => break,
        };

        // Update orientation to chosen direction and move one intersection
        cur_dir = dir;
        match cur_dir {
            U => y -= 1,
            R => x += 1,
            D => y += 1,
            L => x -= 1,
            _ => {}
        }

        // Append corresponding letter
        let ch = match cur_dir {
            U => 'U',
            R => 'R',
            D => 'D',
            L => 'L',
            _ => '?',
        };
        output.push(ch);

        // Termination condition:
        // Continue until the current intersection is (0,0) and NOT the special-case:
        // (f[0][0] == 6 and current direction == Left) -> in that specific situation do NOT terminate.
        if y == 0 && x == 0 {
            let special_case = (f[0][0] as u8 == 6) && (cur_dir == L);
            if !special_case {
                break;
            }
            // else continue traversal
        }

        // Loop continues otherwise
    }

    // Print the output line terminated with newline
    println!("{}", output);
}
