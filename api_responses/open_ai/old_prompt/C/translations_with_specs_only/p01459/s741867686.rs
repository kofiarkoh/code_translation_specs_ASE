use std::collections::VecDeque;

const MAX_SIZE: usize = 103;

fn bfs_pathfinding(H: usize, W: usize, A: usize, B: usize, map: [[char; MAX_SIZE]; MAX_SIZE]) -> i32 {
    // Directions for movement: right, down, left, up
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    // Find the starting ('S') and goal ('G') positions
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for r in 0..H {
        for c in 0..W {
            if map[r][c] == 'S' {
                start = (r, c);
            } else if map[r][c] == 'G' {
                goal = (r, c);
            }
        }
    }

    // Check precondition: 'S' must not be at the bottom row or adjacent to a wall below
    if start.0 == H - 1 || (start.0 < H - 1 && map[start.0 + 1][start.1] == '#') {
        return -1;
    }

    // Visited states: mk[r][c][a][b] where a and b are moves in respective directions
    let mut mk = [[[[[false; 11]; 11]; MAX_SIZE]; MAX_SIZE]; MAX_SIZE];
    
    // BFS queue
    let mut que = VecDeque::new();
    que.push_back((start.0, start.1, 0, 0)); // (row, col, moves in A, moves in B)
    mk[start.0][start.1][0][0] = true;

    while let Some((r, c, a, b)) = que.pop_front() {
        // Check if we reached the goal
        if (r, c) == goal {
            return (a + b) as i32;
        }

        // Explore all directions
        for (dr, dc) in &directions {
            let mut na = a;
            let mut nb = b;
            let mut nr = r as isize;
            let mut nc = c as isize;

            // Move in the current direction until hitting a wall or exceeding limits
            while na < A && nb < B {
                nr += *dr as isize;
                nc += *dc as isize;

                // Check bounds and obstacles
                if nr < 0 || nr >= H as isize || nc < 0 || nc >= W as isize || map[nr as usize][nc as usize] == '#' {
                    break;
                }

                // Update moves based on direction
                if *dr == 0 {
                    na += 1; // horizontal move
                } else {
                    nb += 1; // vertical move
                }

                // Check if this state has been visited
                if !mk[nr as usize][nc as usize][na][nb] {
                    mk[nr as usize][nc as usize][na][nb] = true;
                    que.push_back((nr as usize, nc as usize, na, nb));
                }
            }
        }
    }

    // If no path was found, return -1
    -1
}

fn main() {
    // Example usage
    let H = 5;
    let W = 5;
    let A = 2;
    let B = 2;
    let map = [
        ['#', '#', '#', '#', '#'],
        ['#', 'S', '.', 'G', '#'],
        ['#', '#', '#', '#', '#'],
        ['#', '#', '#', '#', '#'],
        ['#', '#', '#', '#', '#'],
    ];

    let result = bfs_pathfinding(H, W, A, B, map);
    println!("{}", result); // Output the result
}
