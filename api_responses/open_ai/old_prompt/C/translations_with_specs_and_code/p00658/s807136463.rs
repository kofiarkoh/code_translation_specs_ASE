use std::io::{self, BufRead};

const GAMEOVER_CLEAR: i32 = 1;
const GAMEOVER_HOLE: i32 = 2;
const GAMEOVER_BLOCK: i32 = 3;
const GAMEOVER_WGOAL: i32 = 4;

struct Game {
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
    px: usize,
    py: usize,
    gx: usize,
    gy: usize,
    gameover: i32,
}

impl Game {
    fn new(height: usize, width: usize, map: Vec<Vec<char>>) -> Self {
        let mut game = Game {
            width,
            height,
            map,
            px: 0,
            py: 0,
            gx: 0,
            gy: 0,
            gameover: 0,
        };
        game.init();
        game
    }

    fn init(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y][x] == 'S' {
                    self.px = x;
                    self.py = y;
                    self.map[y][x] = '.';
                } else if self.map[y][x] == 'G' {
                    self.gx = x;
                    self.gy = y;
                }
            }
        }
        self.map.push(vec!['#'; self.width]);
        self.gameover = 0;
    }

    fn moveto(&mut self, to: usize) {
        let old_px = self.px;
        if to < 1 || to > self.width { return; }
        let to = to - 1;
        let add = if to < self.px { -1 } else { 1 };
        if self.map[self.py][to] != '.' || self.px == to { return; }

        let mut i = self.px;
        while i != to {
            if self.map[self.py + 1][i] == '.' { break; }
            i = (i as isize + add) as usize;
        }

        if i == to && self.map[self.py + 1][to] != '.' {
            self.px = to;
            self.countdown(old_px);
        }
    }

    fn push_internal(&mut self, x: usize, y: usize, isright: bool) {
        let block = self.map[y][x];
        if block == 'C' || block == '.' { return; }
        self.map[y][x] = '.';

        if isright {
            if x < self.width - 1 && self.map[y][x + 1] != '.' {
                self.map[y][x] = block;
                return;
            }
            let mut i = x + 1;
            while i < self.width {
                if !(block == 'I' || self.map[y + 1][i] == 'I') || self.map[y + 1][i] == '.' { break; }
                if i < self.width - 1 && self.map[y][i + 1] != '.' { break; }
                i += 1;
            }
            if i < self.width { self.map[y][i] = block; }
        } else {
            if x > 0 && self.map[y][x - 1] != '.' {
                self.map[y][x] = block;
                return;
            }
            let mut i = x as isize - 1;
            while i >= 0 {
                if !(block == 'I' || self.map[y + 1][i as usize] == 'I') || self.map[y + 1][i as usize] == '.' { break; }
                if i > 0 && self.map[y][(i - 1) as usize] != '.' { break; }
                i -= 1;
            }
            if i >= 0 { self.map[y][i as usize] = block; }
        }
    }

    fn push(&mut self, isright: bool) {
        if isright {
            if self.px >= self.width - 1 { return; }
            if self.map[self.py][self.px + 1] == '.' { return; }
            let mut i = self.px + 1;
            while i < self.width && self.map[self.py][i] != '.' {
                if self.map[self.py][i] == 'C' { return; }
                i += 1;
            }
            for i in (self.px + 1..i).rev() {
                self.push_internal(i, self.py, true);
            }
        } else {
            if self.px == 0 { return; }
            if self.map[self.py][self.px - 1] == '.' { return; }
            let mut i = self.px - 1;
            while i != usize::MAX && self.map[self.py][i] != '.' {
                if self.map[self.py][i] == 'C' { return; }
                i = i.checked_sub(1).unwrap_or(usize::MAX);
            }
            for i in (i + 1..=self.px).rev() {
                self.push_internal(i, self.py, false);
            }
        }
    }

    fn pull(&mut self, isright: bool) {
        let old_px = self.px;
        if self.px == 0 || self.px >= self.width - 1 { return; }
        if isright {
            if self.map[self.py][self.px + 1] == '.' && self.map[self.py][self.px - 1] != '.' && self.map[self.py][self.px - 1] != 'C' {
                self.map[self.py][self.px] = self.map[self.py][self.px - 1];
                self.map[self.py][self.px - 1] = '.';
                self.px += 1;
            } else { return; }
        } else {
            if self.map[self.py][self.px - 1] == '.' && self.map[self.py][self.px + 1] != '.' && self.map[self.py][self.px + 1] != 'C' {
                self.map[self.py][self.px] = self.map[self.py][self.px + 1];
                self.map[self.py][self.px + 1] = '.';
                self.px -= 1;
            } else { return; }
        }
        self.countdown(old_px);
    }

    fn climb(&mut self, isright: bool) {
        let old_px = self.px;
        if self.py == 0 { return; }
        if isright {
            if self.px < self.width - 1 && self.map[self.py - 1][self.px] == '.' && self.map[self.py - 1][self.px + 1] == '.' && self.map[self.py][self.px + 1] != '.' {
                self.px += 1;
                self.py -= 1;
            } else { return; }
        } else {
            if self.px > 0 && self.map[self.py - 1][self.px] == '.' && self.map[self.py - 1][self.px - 1] == '.' && self.map[self.py][self.px - 1] != '.' {
                self.px -= 1;
                self.py -= 1;
            } else { return; }
        }
        self.countdown(old_px);
    }

    fn getdown(&mut self, isright: bool) {
        let old_px = self.px;
        if isright {
            if self.px < self.width - 1 && self.map[self.py][self.px + 1] == '.' && self.map[self.py + 1][self.px + 1] == '.' {
                self.px += 1;
                self.py += 1;
            } else { return; }
        } else {
            if self.px > 0 && self.map[self.py][self.px - 1] == '.' && self.map[self.py + 1][self.px - 1] == '.' {
                self.px -= 1;
                self.py += 1;
            } else { return; }
        }
        self.countdown(old_px);
    }

    fn countdown(&mut self, old_px: usize) {
        if self.map[self.py + 1][old_px] == '3' {
            self.map[self.py + 1][old_px] = '2';
        } else if self.map[self.py + 1][old_px] == '2' {
            self.map[self.py + 1][old_px] = '1';
        } else if self.map[self.py + 1][old_px] == '1' {
            self.map[self.py + 1][old_px] = '.';
        }
    }

    fn gameover_hantei(&mut self) -> i32 {
        if self.gameover != 0 { return self.gameover; }
        if self.map[self.py + 1][self.px] == 'G' { return self.gameover = GAMEOVER_CLEAR; }
        if self.map[self.py + 1][self.px] == 'B' { return self.gameover = GAMEOVER_HOLE; }
        if self.map[self.py][self.px] != '.' { return self.gameover = GAMEOVER_BLOCK; }
        if self.map[self.gy][self.gx] != 'G' { return self.gameover = GAMEOVER_WGOAL; }
        0
    }

    fn auto_syori(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.map[y + 1][x] == 'B' && self.map[y][x] != '.' {
                        self.map[y][x] = '.';
                        moved = true;
                    }
                }
            }
            while self.map[self.py + 1][self.px] == '.' {
                self.py += 1;
                moved = true;
            }
            for y in (0..self.height - 1).rev() {
                for x in 0..self.width {
                    if self.map[y][x] != '.' {
                        let mut y2 = y;
                        while {
                            if (x == 0 || self.map[y2 + 1][x - 1] == '.') && self.map[y2 + 1][x] == '.' && (x == self.width - 1 || self.map[y2 + 1][x + 1] == '.') {
                                moved = true;
                                true
                            } else {
                                false
                            }
                        } {
                            y2 += 1;
                        }
                        if y2 != y {
                            self.map[y2][x] = self.map[y][x];
                            self.map[y][x] = '.';
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    loop {
        let first_line = lines.next().unwrap();
        let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let height = dimensions.next().unwrap();
        let width = dimensions.next().unwrap();
        if height == 0 && width == 0 { break; }

        let mut map = Vec::new();
        for _ in 0..height {
            let line = lines.next().unwrap();
            map.push(line.chars().collect());
        }

        let command_num: usize = lines.next().unwrap().parse().unwrap();
        let mut game = Game::new(height, width, map);

        for _ in 0..command_num {
            let command_line = lines.next().unwrap();
            let mut command_parts = command_line.split_whitespace();
            let command = command_parts.next().unwrap();
            let command2 = command_parts.next().unwrap();

            if game.gameover_hantei() == 0 {
                match command {
                    "MOVETO" => game.moveto(command2.parse::<usize>().unwrap()),
                    "PUSH" => game.push(command2.chars().next().unwrap() == 'R'),
                    "PULL" => game.pull(command2.chars().next().unwrap() == 'R'),
                    "GETDOWN" => game.getdown(command2.chars().next().unwrap() == 'R'),
                    "CLIMB" => game.climb(command2.chars().next().unwrap() == 'R'),
                    _ => return,
                }
                if game.gameover_hantei() == 0 {
                    game.auto_syori();
                }
            }
        }

        match game.gameover_hantei() {
            GAMEOVER_CLEAR => println!("Game Over : Cleared"),
            GAMEOVER_HOLE => println!("Game Over : Death by Hole"),
            GAMEOVER_BLOCK => println!("Game Over : Death by Block"),
            GAMEOVER_WGOAL => println!("Game Over : Death by Walking Goal"),
            _ => println!("Game Over : Gave Up"),
        }
    }
}
