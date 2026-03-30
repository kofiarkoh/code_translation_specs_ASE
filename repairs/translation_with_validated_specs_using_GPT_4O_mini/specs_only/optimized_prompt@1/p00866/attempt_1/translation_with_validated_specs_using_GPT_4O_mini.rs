use std::fmt;

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}

fn align_clock_hands(n: usize, positions: Vec<(u32, u32, u32)>) {
    for (hour, minute, second) in positions {
        // Validate input ranges
        if hour > 59 || minute > 59 || second > 59 {
            continue; // Skip invalid input
        }

        let mut alignments = Vec::new();

        // Check for alignments at each minute offset
        for offset in 0..60 {
            let adjusted_hour = (hour + offset) % 60;
            let adjusted_minute = (minute + offset) % 60;
            let adjusted_second = (second + offset) % 60;

            // Check if all hands align
            if adjusted_hour == adjusted_minute && adjusted_minute == adjusted_second {
                let total_seconds = offset * 60 + adjusted_hour * 3600 + adjusted_minute * 60 + adjusted_second;
                alignments.push(total_seconds);
            }
        }

        // Determine the earliest and latest alignments
        if alignments.is_empty() {
            println!("No valid alignments for input: ({}, {}, {})", hour, minute, second);
        } else {
            let min_time = alignments.iter().min().unwrap();
            let max_time = alignments.iter().max().unwrap();

            let earliest = convert_seconds_to_time(*min_time);
            let latest = convert_seconds_to_time(*max_time);

            println!("Earliest alignment: {}, Latest alignment: {}", earliest, latest);
        }
    }
}

fn convert_seconds_to_time(total_seconds: u32) -> Time {
    let hours = (total_seconds / 3600) % 24; // Wrap around after 24 hours
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    Time { hours, minutes, seconds }
}

fn main() {
    // Example usage
    let n = 2; // Number of sets
    let positions = vec![(0, 0, 0), (15, 30, 45)]; // Example clock hand positions
    align_clock_hands(n, positions);
}
