use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Player {
    id: u32,
    score: u32,
}

impl Player {
    fn new(id: u32, score: u32) -> Self {
        Player { id, score }
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut tokens = input.split_whitespace();

    // Read the number of players
    let num_players: usize = match tokens.next() {
        Some(num) => num.parse().unwrap_or(0),
        None => 0,
    };

    // Terminate if the number of players is zero or negative
    if num_players == 0 {
        return;
    }

    let mut players: Vec<Player> = Vec::new();

    // Read player data
    for _ in 0..num_players {
        let id: u32 = tokens.next().unwrap().parse().unwrap();
        let scores: Vec<u32> = (0..10).map(|_| tokens.next().unwrap().parse().unwrap()).collect();
        let total_score = calculate_score(&scores);
        players.push(Player::new(id, total_score));
    }

    // Sort players by score (descending) and by id (ascending) for ties
    players.sort_by(|a, b| {
        match b.score.cmp(&a.score) {
            Ordering::Equal => a.id.cmp(&b.id),
            other => other,
        }
    });

    // Output the results
    for player in players {
        println!("{} {}", player.id, player.score);
    }
}

// Function to calculate the total score based on bowling rules
fn calculate_score(scores: &[u32]) -> u32 {
    // This function assumes that scores are valid and contains exactly 10 frames
    let mut total_score = 0;
    let mut frame_index = 0;

    for frame in 0..10 {
        if scores[frame_index] == 10 { // Strike
            total_score += 10 + scores[frame_index + 1] + scores[frame_index + 2];
            frame_index += 1;
        } else if scores[frame_index] + scores[frame_index + 1] == 10 { // Spare
            total_score += 10 + scores[frame_index + 2];
            frame_index += 2;
        } else { // Open frame
            total_score += scores[frame_index] + scores[frame_index + 1];
            frame_index += 2;
        }
    }

    total_score
}
