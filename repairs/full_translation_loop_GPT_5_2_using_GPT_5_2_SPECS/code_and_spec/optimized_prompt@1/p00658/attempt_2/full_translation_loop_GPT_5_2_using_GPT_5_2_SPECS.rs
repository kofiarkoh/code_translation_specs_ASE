use std::io::{self, Read};
use std::process;

const GAMEOVER_CLEAR: i32 = 1;
const GAMEOVER_HOLE: i32 = 2;
const GAMEOVER_BLOCK: i32 = 3;
const GAMEOVER_WGOAL: i32 = 4;

struct Game {
    width: usize,
    height: usize,
    // map has an extra solid boundary row at y=height
    map: Vec<Vec<u8>>,
    px: usize,
    py: usize,
    gx: usize,
    gy: usize,
    gameover: i32,
}

impl Game {
    fn new(height: usize, width: usize, rows: Vec<Vec<u8>>) -> Self {
        let mut map = vec![vec![b'.'; width]; height + 1];
        for y in 0..height {
            map[y].copy_from_slice(&rows[y]);
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
        for y in 0..self.height {
            for x in 0..self.width {
                match self.map[y][x] {
                    b'S' => {
                        self.px = x;
                        self.py = y;
                        self.map[y][x] = b'.';
                    }
                    b'G' => {
                        // last encountered in row-major order is tracked
                        self.gx = x;
                        self.gy = y;
                    }
                    _ => {}
                }
            }
        }
        for x in 0..self.width {
            self.map[self.height][x] = b'#';
        }
        self.gameover = 0;
    }

    fn countdown_at(&mut self, y: usize, x: usize) {
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
        if self.map[self.py][to] != b'.' || self.px == to {
            return;
        }
        let add: i32 = if to < self.px { -1 } else { 1 };
        let mut i = self.px as i32;
        while i != to as i32 {
            if self.map[self.py + 1][i as usize] == b'.' {
                break;
            }
            i += add;
        }
        if i == to as i32 && self.map[self.py + 1][to] != b'.' {
            self.px = to;
            self.countdown_at(self.py + 1, old_px);
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
                // C condition:
                // if(!(block=='I' || map[y+1][i]=='I') || map[y+1][i]=='.')break;
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
        } else {
            if x > 0 && self.map[y][x - 1] != b'.' {
                self.map[y][x] = block;
                return;
            }
            let mut i: i32 = x as i32 - 1;
            while i >= 0 {
                let iu = i as usize;
                if !((block == b'I' || self.map[y + 1][iu] == b'I') && self.map[y + 1][iu] != b'.')
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
        }
    }

    fn push(&mut self, isright: bool) {
        if isright {
            if self.px >= self.width - 1 || self.map[self.py][self.px + 1] == b'.' {
                return;
            }
            let mut i = self.px + 1;
            while i < self.width && self.map[self.py][i] != b'.' {
                if self.map[self.py][i] == b'C' {
                    return;
                }
                i += 1;
            }
            // C code does: for(i--;i>px;i--) push_internal(i,...)
            // If i==width (no '.'), i-- => width-1 and still pushes.
            if i == 0 {
                return;
            }
            let mut j = i - 1;
            while j > self.px {
                self.push_internal(j, self.py, true);
                j -= 1;
            }
        } else {
            if self.px == 0 || self.map[self.py][self.px - 1] == b'.' {
                return;
            }
            let mut i: i32 = self.px as i32 - 1;
            while i >= 0 && self.map[self.py][i as usize] != b'.' {
                if self.map[self.py][i as usize] == b'C' {
                    return;
                }
                i -= 1;
            }
            // C code does: for(i++;i<px;i++) push_internal(i,...)
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
        self.countdown_at(self.py + 1, old_px);
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
        self.countdown_at(self.py + 2, old_px);
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
        self.countdown_at(self.py, old_px);
    }

    fn gameover_hantei(&mut self) -> i32 {
        if self.gameover != 0 {
            return self.gameover;
        }
        if self.map[self.py + 1][self.px] == b'G' {
            self.gameover = GAMEOVER_CLEAR;
        } else if self.map[self.py + 1][self.px] == b'B' {
            self.gameover = GAMEOVER_HOLE;
        } else if self.map[self.py][self.px] != b'.' {
            self.gameover = GAMEOVER_BLOCK;
        } else if self.map[self.gy][self.gx] != b'G' {
            self.gameover = GAMEOVER_WGOAL;
        }
        self.gameover
    }

    fn auto_syori(&mut self) {
        // The original C code can loop for a long time in worst cases.
        // We preserve semantics but add a conservative iteration cap to avoid non-termination
        // due to accidental logic divergence. The cap is far above any realistic stabilization
        // steps for a 50x50 grid.
        let mut moved = true;
        let mut iter = 0usize;
        let iter_cap = 1_000_000usize;

        while moved {
            iter += 1;
            if iter > iter_cap {
                // If this triggers, something is inconsistent; match "timeout avoidance"
                // by terminating (no defined behavior in spec for this case).
                process::exit(1);
            }

            moved = false;

            // remove any non-empty cell directly above a hole 'B'
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.map[y + 1][x] == b'B' && self.map[y][x] != b'.' {
                        self.map[y][x] = b'.';
                        moved = true;
                    }
                }
            }

            // player falls
            while self.map[self.py + 1][self.px] == b'.' {
                self.py += 1;
                moved = true;
            }

            // blocks fall: IMPORTANT: in C, y2 is not bounded by height; it can fall into the
            // boundary row (y==height) because map[height][x] is '#', but the *cell itself*
            // can be moved into map[height][x] if the three below cells are '.'.
            // To preserve this, we allow y2 to reach height (but not beyond).
            for y in (0..self.height).rev() {
                // C starts from height-2; y==height-1 cannot move because below is boundary '#'
                if y + 1 > self.height {
                    continue;
                }
                for x in 0..self.width {
                    if self.map[y][x] == b'.' {
                        continue;
                    }
                    let mut y2 = y;
                    loop {
                        if y2 >= self.height {
                            break;
                        }
                        let left_ok = x == 0 || self.map[y2 + 1][x - 1] == b'.';
                        let mid_ok = self.map[y2 + 1][x] == b'.';
                        let right_ok = x + 1 >= self.width || self.map[y2 + 1][x + 1] == b'.';
                        if left_ok && mid_ok && right_ok {
                            moved = true;
                            y2 += 1;
                        } else {
                            break;
                        }
                    }
                    if y2 != y {
                        self.map[y2][x] = self.map[y][x];
                        self.map[y][x] = b'.';
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

    let mut out = String::new();

    loop {
        let h = match it.next() {
            Some(v) => v.parse::<usize>().unwrap(),
            None => break,
        };
        let w = it.next().unwrap().parse::<usize>().unwrap();
        if h == 0 && w == 0 {
            break;
        }

        let mut rows: Vec<Vec<u8>> = Vec::with_capacity(h);
        for _ in 0..h {
            let s = it.next().unwrap();
            let bytes = s.as_bytes();
            let mut row = vec![b'.'; w];
            for x in 0..w {
                row[x] = bytes[x];
            }
            rows.push(row);
        }

        let command_num = it.next().unwrap().parse::<usize>().unwrap();
        let mut game = Game::new(h, w, rows);

        for _ in 0..command_num {
            let cmd = it.next().unwrap();
            let arg = it.next().unwrap();

            if game.gameover_hantei() == 0 {
                match cmd {
                    "MOVETO" => {
                        let to = arg.parse::<i32>().unwrap_or(0);
                        game.moveto(to);
                    }
                    "PUSH" => {
                        let isright = arg.as_bytes()[0] == b'R';
                        game.push(isright);
                    }
                    "PULL" => {
                        let isright = arg.as_bytes()[0] == b'R';
                        game.pull(isright);
                    }
                    "GETDOWN" => {
                        let isright = arg.as_bytes()[0] == b'R';
                        game.getdown(isright);
                    }
                    "CLIMB" => {
                        let isright = arg.as_bytes()[0] == b'R';
                        game.climb(isright);
                    }
                    _ => {
                        // Spec: unrecognized command => terminate immediately non-zero.
                        process::exit(1);
                    }
                }
                if game.gameover_hantei() == 0 {
                    game.auto_syori();
                }
            }
        }

        match game.gameover_hantei() {
            GAMEOVER_CLEAR => out.push_str("Game Over : Cleared\n"),
            GAMEOVER_HOLE => out.push_str("Game Over : Death by Hole\n"),
            GAMEOVER_BLOCK => out.push_str("Game Over : Death by Block\n"),
            GAMEOVER_WGOAL => out.push_str("Game Over : Death by Walking Goal\n"),
            _ => out.push_str("Game Over : Gave Up\n"),
        }
    }

    print!("{out}");
}
