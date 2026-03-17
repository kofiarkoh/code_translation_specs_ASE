// Define constants for the maximum grid size
const MAX_HEIGHT: usize = 51;
const MAX_WIDTH: usize = 51;

// Define the commands as constants for better readability
const MOVETO: &str = "MOVETO";
const PUSH: &str = "PUSH";
const PULL: &str = "PULL";
const GETDOWN: &str = "GETDOWN";
const CLIMB: &str = "CLIMB";

// Define the possible characters in the grid
const PLAYER: char = 'S';
const GOAL: char = 'G';
const BLOCK: char = '#';
const EMPTY: char = '.';

// Struct to represent the game state
struct Game {
    height: usize,
    width: usize,
    map: Vec<Vec<char>>,
    player_pos: (usize, usize),
    gameover: bool,
}

impl Game {
    // Initialize a new game with the given parameters
    fn new(height: usize, width: usize, map: Vec<Vec<char>>) -> Self {
        let player_pos = Self::find_player(&map);
        Game {
            height,
            width,
            map,
            player_pos,
            gameover: false,
        }
    }

    // Find the player's initial position in the grid
    fn find_player(map: &Vec<Vec<char>>) -> (usize, usize) {
        for (i, row) in map.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == PLAYER {
                    return (i, j);
                }
            }
        }
        panic!("Player not found on the map");
    }

    // Execute a command
    fn execute_command(&mut self, command: &str, command2: Option<(usize, usize)>) {
        match command {
            MOVETO => {
                if let Some((new_x, new_y)) = command2 {
                    self.move_to(new_x, new_y);
                }
            }
            PUSH => {
                if let Some((new_x, new_y)) = command2 {
                    self.push_block(new_x, new_y);
                }
            }
            PULL => {
                if let Some((new_x, new_y)) = command2 {
                    self.pull_block(new_x, new_y);
                }
            }
            GETDOWN => {
                self.get_down();
            }
            CLIMB => {
                self.climb();
            }
            _ => {}
        }
    }

    // Move the player to a new position
    fn move_to(&mut self, new_x: usize, new_y: usize) {
        if self.is_within_bounds(new_x, new_y) && self.map[new_x][new_y] != BLOCK {
            self.player_pos = (new_x, new_y);
            self.check_game_over();
        }
    }

    // Push a block if possible
    fn push_block(&mut self, new_x: usize, new_y: usize) {
        // Logic for pushing a block would go here
        // For now, we will just ignore the command
    }

    // Pull a block if possible
    fn pull_block(&mut self, new_x: usize, new_y: usize) {
        // Logic for pulling a block would go here
        // For now, we will just ignore the command
    }

    // Get down from a position
    fn get_down(&mut self) {
        // Logic for getting down would go here
        // For now, we will just ignore the command
    }

    // Climb to a position
    fn climb(&mut self) {
        // Logic for climbing would go here
        // For now, we will just ignore the command
    }

    // Check if the player has reached the goal or if the game is over
    fn check_game_over(&mut self) {
        let (px, py) = self.player_pos;
        if self.map[px][py] == GOAL {
            self.gameover = true;
            println!("Game Over: You've reached the goal!");
        }
    }

    // Check if the given coordinates are within the grid bounds
    fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.height && y < self.width
    }
}

// Main function to run the game
fn main() {
    // Example initialization of the game
    let height = 5;
    let width = 5;
    let map = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', 'S', '#', '.', 'G'],
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '#', '#', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];

    let mut game = Game::new(height, width, map);
    
    // Example command execution
    game.execute_command(MOVETO, Some((1, 2))); // Attempt to move to a blocked position
    game.execute_command(MOVETO, Some((1, 4))); // Move to the goal
}
