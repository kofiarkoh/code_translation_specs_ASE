use std::io::{self, Read};
use std::process;

const GAMEOVER_CLEAR: i32 = 1;
const GAMEOVER_HOLE: i32 = 2;
const GAMEOVER_BLOCK: i32 = 3;
const GAMEOVER_WGOAL: i32 = 4;

struct Game {
    width: usize,
    height: usize,
    // map has an extra bottom boundary row at index height, filled with '#'
    map: Vec<Vec<u8>>,
    px: usize,
    py: usize,
    gx: usize,
    gy: usize,
    gameover: i32,
}

impl Game {
    fn new(height: usize, width: usize, rows: Vec<Vec<u8>>) -> Self {
        // allocate height+1 rows, width columns
        let mut map = vec![vec![b'.'; width]; height + 1];
        for y in 0..height {
            map[y].copy_from_slice(&rows[y]);
        }
        // bottom boundary
        for x in 0..width {
            map[height][x] = b'#';
        }

        let mut g = Game {
            width,
            height,
            map,
            px: 0,
            py: 0,
            gx: 0,
            gy: 0,
            gameover: 0,
        };
        g.game_init();
        g
    }

    fn game_init(&mut self) {
        // Scan order: y then x; last 'S'/'G' wins.
        for y in 0..self.height {
            for x in 0..self.width {
                match self.map[y][x] {
                    b'S' => {
                        self.px = x;
                        self.py = y;
                        self.map[y][x] = b'.';
                    }
                    b'G' => {
                        self.gx = x;
                        self.gy = y;
                    }
                    _ => {}
                }
            }
        }
        // bottom boundary already set in new()
        self.gameover = 0;
    }

    fn dec_countdown_cell(&mut self, y: usize, x: usize) {
        match self.map[y][x] {
            b'3' => self.map[y][x] = b'2',
            b'2' => self.map[y][x] = b'1',
            b'1' => self.map[y][x] = b'.',
            _ => {}
        }
    }

    fn moveto(&mut self, to_1based: i32) {
        let old_px = self.px;
        if to_1based < 1 || to_1based > self.width as i32 {
            return;
        }
        let to = (to_1based - 1) as usize;
        if self.map[self.py][to] != b'.' {
            return;
        }
        if self.px == to {
            return;
        }

        let add: i32 = if to < self.px { -1 } else { 1 };
        let mut i = self.px as i32;
        while i != to as i32 {
            let iu = i as usize;
            if self.map[self.py + 1][iu] == b'.' {
                break;
            }
            i += add;
        }
        if i == to as i32 && self.map[self.py + 1][to] != b'.' {
            self.px = to;
            // countdown below previous position
            self.dec_countdown_cell(self.py + 1, old_px);
        }
    }

    fn push_internal(&mut self, x: usize, y: usize, isright: bool) {
        let block = self.map[y][x];
        if block == b'C' || block == b'.' {
            return;
        }
        self.map[y][x] = b'.';

        if isright {
            if x < self.width - 1 && self.map[y][x + 1] != b'.' {
                self.map[y][x] = block;
                return;
            }
            let mut i = x + 1;
            while i < self.width {
                // stop if not (block is I or below is I) OR below is '.'
                if !((block == b'I' || self.map[y + 1][i] == b'I') && self.map[y + 1][i] != b'.')
                {
                    break;
                }
                if i < self.width - 1 && self.map[y][i + 1] != b'.' {
                    break;
                }
                i += 1;
            }
            if i < self.width {
                self.map[y][i] = block;
            }
            // else: out of bounds => removed
        } else {
            if x > 0 && self.map[y][x - 1] != b'.' {
                self.map[y][x] = block;
                return;
            }
            let mut i: i32 = x as i32 - 1;
            while i >= 0 {
                let iu = i as usize;
                if !((block == b'I' || self.map[y + 1][iu] == b'I')
                    && self.map[y + 1][iu] != b'.')
                {
                    break;
                }
                if iu > 0 && self.map[y][iu - 1] != b'.' {
                    break;
                }
                i -= 1;
            }
            if i >= 0 {
                self.map[y][i as usize] = block;
            }
            // else: out of bounds => removed
        }
    }

    fn push(&mut self, isright: bool) {
        if isright {
            if self.px >= self.width - 1 {
                return;
            }
            if self.map[self.py][self.px + 1] == b'.' {
                return;
            }
            let mut i = self.px + 1;
            while i < self.width && self.map[self.py][i] != b'.' {
                if self.map[self.py][i] == b'C' {
                    return;
                }
                i += 1;
            }
            // Need a '.' after the run; if i==width, no '.' found => do nothing.
            if i == self.width {
                return;
            }
            // process from farthest to nearest: i-1 down to px+1
            let mut j = i - 1;
            while j > self.px {
                self.push_internal(j, self.py, true);
                j -= 1;
            }
        } else {
            if self.px == 0 {
                return;
            }
            if self.map[self.py][self.px - 1] == b'.' {
                return;
            }
            let mut i: i32 = self.px as i32 - 1;
            while i >= 0 && self.map[self.py][i as usize] != b'.' {
                if self.map[self.py][i as usize] == b'C' {
                    return;
                }
                i -= 1;
            }
            if i < 0 {
                return; // no '.' found
            }
            // process from farthest to nearest: i+1 up to px-1
            let mut j = i + 1;
            while (j as usize) < self.px {
                self.push_internal(j as usize, self.py, false);
                j += 1;
            }
        }
    }

    fn pull(&mut self, isright: bool) {
        let old_px = self.px;
        if self.px == 0 || self.px >= self.width - 1 {
            return;
        }
        if isright {
            if self.map[self.py][self.px + 1] == b'.'
                && self.map[self.py][self.px - 1] != b'.'
                && self.map[self.py][self.px - 1] != b'C'
            {
                self.map[self.py][self.px] = self.map[self.py][self.px - 1];
                self.map[self.py][self.px - 1] = b'.';
                self.px += 1;
            } else {
                return;
            }
        } else {
            if self.map[self.py][self.px - 1] == b'.'
                && self.map[self.py][self.px + 1] != b'.'
                && self.map[self.py][self.px + 1] != b'C'
            {
                self.map[self.py][self.px] = self.map[self.py][self.px + 1];
                self.map[self.py][self.px + 1] = b'.';
                self.px -= 1;
            } else {
                return;
            }
        }
        self.dec_countdown_cell(self.py + 1, old_px);
    }

    fn climb(&mut self, isright: bool) {
        let old_px = self.px;
        if self.py == 0 {
            return;
        }
        if isright {
            if self.px < self.width - 1
                && self.map[self.py - 1][self.px] == b'.'
                && self.map[self.py - 1][self.px + 1] == b'.'
                && self.map[self.py][self.px + 1] != b'.'
            {
                self.px += 1;
                self.py -= 1;
            } else {
                return;
            }
        } else {
            if self.px > 0
                && self.map[self.py - 1][self.px] == b'.'
                && self.map[self.py - 1][self.px - 1] == b'.'
                && self.map[self.py][self.px - 1] != b'.'
            {
                self.px -= 1;
                self.py -= 1;
            } else {
                return;
            }
        }
        // countdown below previous position: now at py+2 (since py decreased by 1)
        self.dec_countdown_cell(self.py + 2, old_px);
    }

    fn getdown(&mut self, isright: bool) {
        let old_px = self.px;
        if isright {
            if self.px < self.width - 1
                && self.map[self.py][self.px + 1] == b'.'
                && self.map[self.py + 1][self.px + 1] == b'.'
            {
                self.px += 1;
                self.py += 1;
            } else {
                return;
            }
        } else {
            if self.px > 0
                && self.map[self.py][self.px - 1] == b'.'
                && self.map[self.py + 1][self.px - 1] == b'.'
            {
                self.px -= 1;
                self.py += 1;
            } else {
                return;
            }
        }
        // countdown at previous cell (old row, old col): now at map[py][old_px] because py increased by 1
        self.dec_countdown_cell(self.py, old_px);
    }

    fn gameover_hantei(&mut self) -> i32 {
        if self.gameover != 0 {
            return self.gameover;
        }
        if self.map[self.py + 1][self.px] == b'G' {
            self.gameover = GAMEOVER_CLEAR;
            return self.gameover;
        }
        if self.map[self.py + 1][self.px] == b'B' {
            self.gameover = GAMEOVER_HOLE;
            return self.gameover;
        }
        if self.map[self.py][self.px] != b'.' {
            self.gameover = GAMEOVER_BLOCK;
            return self.gameover;
        }
        if self.map[self.gy][self.gx] != b'G' {
            self.gameover = GAMEOVER_WGOAL;
            return self.gameover;
        }
        0
    }

    fn auto_syori(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;

            // Hole removal
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.map[y + 1][x] == b'B' && self.map[y][x] != b'.' {
                        self.map[y][x] = b'.';
                        moved = true;
                    }
                }
            }

            // Player falling
            while self.map[self.py + 1][self.px] == b'.' {
                self.py += 1;
                moved = true;
            }

            // Block falling: from height-2 down to 0
            if self.height >= 2 {
                for y in (0..=self.height - 2).rev() {
                    for x in 0..self.width {
                        if self.map[y][x] != b'.' {
                            let mut y2 = y;
                            loop {
                                let down_left_ok = x == 0 || self.map[y2 + 1][x - 1] == b'.';
                                let down_ok = self.map[y2 + 1][x] == b'.';
                                let down_right_ok =
                                    x == self.width - 1 || self.map[y2 + 1][x + 1] == b'.';
                                if down_left_ok && down_ok && down_right_ok {
                                    moved = true;
                                    y2 += 1;
                                } else {
                                    break;
                                }
                            }
                            if y2 != y {
                                let b = self.map[y][x];
                                self.map[y2][x] = b;
                                self.map[y][x] = b'.';
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let h = match it.next() {
            Some(v) => v.parse::<usize>().unwrap(),
            None => break,
        };
        let w = it.next().unwrap().parse::<usize>().unwrap();
        if h == 0 && w == 0 {
            break;
        }

        // Spec: height/width <= 50, each row length == width.
        let mut rows: Vec<Vec<u8>> = Vec::with_capacity(h);
        for _ in 0..h {
            let s = it.next().unwrap();
            // Assume well-formed as per spec.
            rows.push(s.as_bytes().to_vec());
        }

        let command_num = it.next().unwrap().parse::<usize>().unwrap();

        let mut game = Game::new(h, w, rows);

        for _ in 0..command_num {
            let cmd = it.next().unwrap();
            let arg = it.next().unwrap();

            if game.gameover_hantei() == 0 {
                match cmd {
                    "MOVETO" => {
                        let n = arg.parse::<i32>().unwrap_or(0);
                        game.moveto(n);
                    }
                    "PUSH" => {
                        let isright = arg.as_bytes().first().copied() == Some(b'R');
                        game.push(isright);
                    }
                    "PULL" => {
                        let isright = arg.as_bytes().first().copied() == Some(b'R');
                        game.pull(isright);
                    }
                    "GETDOWN" => {
                        let isright = arg.as_bytes().first().copied() == Some(b'R');
                        game.getdown(isright);
                    }
                    "CLIMB" => {
                        let isright = arg.as_bytes().first().copied() == Some(b'R');
                        game.climb(isright);
                    }
                    _ => {
                        // Spec: terminate immediately with nonzero exit status and no further outputs.
                        process::exit(1);
                    }
                }
                if game.gameover_hantei() == 0 {
                    game.auto_syori();
                }
            }
            // If game over, we still consume input (already done) but do nothing.
        }

        let line = match game.gameover_hantei() {
            GAMEOVER_CLEAR => "Game Over : Cleared",
            GAMEOVER_HOLE => "Game Over : Death by Hole",
            GAMEOVER_BLOCK => "Game Over : Death by Block",
            GAMEOVER_WGOAL => "Game Over : Death by Walking Goal",
            _ => "Game Over : Gave Up",
        };
        outputs.push(line.to_string());
    }

    print!("{}", outputs.join("\n"));
    if !outputs.is_empty() {
        print!("\n");
    }
}
