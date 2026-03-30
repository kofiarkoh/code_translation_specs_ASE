use std::collections::{HashSet, VecDeque};

const PUZZLE_SIZE: usize = 13;
const EMPTY_TILE: usize = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PuzzleState {
    tiles: [usize; PUZZLE_SIZE],
}

impl PuzzleState {
    fn is_solved(&self) -> bool {
        self.tiles == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, EMPTY_TILE]
    }

    fn get_empty_tile_index(&self) -> Option<usize> {
        self.tiles.iter().position(|&tile| tile == EMPTY_TILE)
    }

    fn generate_next_states(&self) -> Vec<PuzzleState> {
        let mut next_states = Vec::new();
        if let Some(empty_index) = self.get_empty_tile_index() {
            let moves = [
                (empty_index as isize - 1, empty_index as isize), // left
                (empty_index as isize + 1, empty_index as isize), // right
                (empty_index as isize - 4, empty_index as isize), // up
                (empty_index as isize + 4, empty_index as isize), // down
            ];

            for (new_index, old_index) in moves.iter() {
                if *new_index >= 0 && *new_index < PUZZLE_SIZE as isize {
                    let new_index = *new_index as usize;
                    if (new_index % 4 != 0 || *old_index % 4 != 3) && (new_index % 4 != 3 || *old_index % 4 != 0) {
                        let mut new_tiles = self.tiles.clone();
                        new_tiles.swap(new_index, *old_index);
                        next_states.push(PuzzleState { tiles: new_tiles });
                    }
                }
            }
        }
        next_states
    }
}

fn bfs(initial_state: PuzzleState) -> Result<usize, &'static str> {
    if !is_valid_input(&initial_state.tiles) {
        return Err("Invalid input");
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((initial_state.clone(), 0));
    visited.insert(initial_state.clone());

    while let Some((current_state, moves)) = queue.pop_front() {
        if current_state.is_solved() {
            return Ok(moves);
        }

        for next_state in current_state.generate_next_states() {
            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                queue.push_back((next_state, moves + 1));
            }
        }
    }

    Err("NA") // Puzzle cannot be solved
}

fn is_valid_input(tiles: &[usize; PUZZLE_SIZE]) -> bool {
    if tiles.len() != PUZZLE_SIZE {
        return false;
    }
    let mut empty_count = 0;
    for &tile in tiles {
        if tile < 0 {
            return false; // Negative number found
        }
        if tile == EMPTY_TILE {
            empty_count += 1;
        }
    }
    empty_count == 1 // Must have exactly one empty tile
}

fn main() {
    let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, EMPTY_TILE]; // Example input
    let initial_state = PuzzleState { tiles: input };

    match bfs(initial_state) {
        Ok(moves) => println!("{}", moves),
        Err(message) => println!("{}", message),
    }
}
