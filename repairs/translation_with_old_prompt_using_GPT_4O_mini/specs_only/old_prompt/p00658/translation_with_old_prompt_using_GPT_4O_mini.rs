use std::io::{self, Write};

const MAX_HEIGHT: usize = 51;
const MAX_WIDTH: usize = 51;

#[derive(Clone, Copy)]
enum Command {
    MoveTo,
    Push,
    Pull,
    GetDown,
    Climb,
}

struct Game {
    height: usize,
    width: usize,
    map: Vec<Vec<char>>,
    player_pos: (usize, usize),
    game_over: bool,
}

impl Game {
    fn new(height: usize, width: usize, map: Vec<Vec<char>>, player_pos: (usize, usize)) -> Self {
        Game {
            height,
            width,
            map,
            player_pos,
            game_over: false,
        }
    }

    fn execute_command(&mut self, command: Command, command2: Option<(usize, usize)>) {
        match command {
            Command::MoveTo => {
                if let Some((new_x, new_y)) = command2 {
                    self.move_to(new_x, new_y);
                }
            }
            Command::Push => {
                if let Some((dx, dy)) = command2 {
                    self.push(dx, dy);
                }
            }
            Command::Pull => {
                if let Some((dx, dy)) = command2 {
                    self.pull(dx, dy);
                }
            }
            Command::GetDown => {
                self.get_down();
            }
            Command::Climb => {
                self.climb();
            }
        }
    }

    fn move_to(&mut self, new_x: usize, new_y: usize) {
        if new_x < self.height && new_y < self.width {
            if self.map[new_x][new_y] != '#' { // Assuming '#' is a block
                self.player_pos = (new_x, new_y);
                if self.map[new_x][new_y] == 'G' { // Assuming 'G' is a goal
                    self.game_over = true;
                    println!("Game Over: Reached the goal!");
                }
            }
        }
    }

    fn push(&mut self, direction: (isize, isize)) {
        let (px, py) = self.player_pos;
        let target_x = (px as isize + direction.0) as usize;
        let target_y = (py as isize + direction.1) as usize;

        if target_x < self.height && target_y < self.width {
            if self.map[target_x][target_y] == '#' { // Assuming '#' is a block
                let block_x = (target_x as isize + direction.0) as usize;
                let block_y = (target_y as isize + direction.1) as usize;
                if block_x < self.height && block_y < self.width && self.map[block_x][block_y] == ' ' {
                    self.map[block_x][block_y] = '#'; // Move block
                    self.map[target_x][target_y] = ' '; // Clear original block position
                }
            }
        }
    }

    fn pull(&mut self, direction: (isize, isize)) {
        let (px, py) = self.player_pos;
        let target_x = (px as isize - direction.0) as usize;
        let target_y = (py as isize - direction.1) as usize;

        if target_x < self.height && target_y < self.width {
            if self.map[target_x][target_y] == '#' { // Assuming '#' is a block
                let block_x = (px as isize - direction.0) as usize;
                let block_y = (py as isize - direction.1) as usize;
                if block_x < self.height && block_y < self.width && self.map[block_x][block_y] == ' ' {
                    self.map[block_x][block_y] = '#'; // Move block
                    self.map[target_x][target_y] = ' '; // Clear original block position
                }
            }
        }
    }

    fn get_down(&mut self) {
        let (px, py) = self.player_pos;
        if px + 1 < self.height && self.map[px + 1][py] != '#' {
            self.player_pos = (px + 1, py);
        }
    }

    fn climb(&mut self) {
        let (px, py) = self.player_pos;
        if px > 0 && self.map[px - 1][py] != '#' {
            self.player_pos = (px - 1, py);
        }
    }

    fn print_map(&self) {
        for row in &self.map {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn main() {
    let height = 5;
    let width = 5;
    let map = vec![
        vec!['#', '#', '#', '#', '#'],
        vec!['#', 'S', '.', '.', '#'],
        vec!['#', '.', '.', '.', '#'],
        vec!['#', '.', '.', 'G', '#'],
        vec!['#', '#', '#', '#', '#'],
    ];
    let player_pos = (1, 1); // Starting position of the player

    let mut game = Game::new(height, width, map, player_pos);
    game.print_map();

    // Example command execution
    game.execute_command(Command::MoveTo, Some((3, 1))); // Move to (3, 1)
    game.execute_command(Command::Push, (0, -1)); // Push left
    game.execute_command(Command::MoveTo, Some((4, 1))); // Move to (4, 1)
    game.execute_command(Command::GetDown, None); // Get down
    game.execute_command(Command::Climb, None); // Climb
    game.print_map();

    if game.game_over {
        println!("Game Over: Gave Up");
    }
}
