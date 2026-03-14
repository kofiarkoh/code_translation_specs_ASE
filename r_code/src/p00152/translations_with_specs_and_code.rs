use std::io::{self, BufRead};

#[derive(Debug)]
struct Player {
    id: i32,
    score: i32,
}

fn sort_players(players: &mut Vec<Player>) {
    players.sort_by(|a, b| {
        if a.score != b.score {
            b.score.cmp(&a.score) // Descending order by score
        } else {
            a.id.cmp(&b.id) // Ascending order by player ID
        }
    });
}

fn calculate_score() -> i32 {
    let stdin = io::stdin();
    let mut total = 0;
    let mut sum = 0;
    let mut frame = 1;
    let mut time = 1;
    let mut flg_spare = false;
    let mut flg_strike = 0;

    while frame < 10 {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let x: i32 = input.trim().parse().unwrap();

        sum += x;

        if flg_spare {
            total += x;
            flg_spare = false;
        } else if flg_strike > 0 {
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }

        if time == 2 {
            total += sum;
            if sum == 10 {
                flg_spare = true;
            }
            sum = 0;
            time = 1;
            frame += 1;
        } else if time == 1 {
            time += 1;
            if x == 10 {
                total += 10;
                flg_strike += 2;
                sum = 0;
                time = 1;
                frame += 1;
            }
        }
    }

    for _ in 0..3 {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let x: i32 = input.trim().parse().unwrap();

        total += x;
        sum += x;

        if flg_spare {
            total += x;
            flg_spare = false;
        } else if flg_strike > 0 {
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }

        if sum < 10 {
            break; // Exit if the sum is less than 10
        }
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let mut players: Vec<Player> = Vec::new();

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        if n <= 0 {
            break; // Terminate if the number of players is zero or negative
        }

        for _ in 0..n {
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();
            let id: i32 = input.trim().parse().unwrap();
            let score = calculate_score();
            players.push(Player { id, score });
        }

        sort_players(&mut players);

        for player in players {
            println!("{} {}", player.id, player.score);
        }

        players.clear(); // Clear the players for the next round
    }
}
