use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 3 {
            continue; // Skip invalid input lines
        }

        let year: i32 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };
        let month: u32 = match parts[1].parse() {
            Ok(num) if num >= 1 && num <= 12 => num,
            _ => continue, // Skip invalid input
        };
        let day: u32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };

        // Determine the Japanese era and adjusted date
        let (era_name, adjusted_year, adjusted_month, adjusted_day) = match (year, month, day) {
            (1868..=1912, _, _) => ("Meiji", year - 1867, month, day),
            (1912..=1926, _, _) => ("Taisho", year - 1911, month, day),
            (1926..=1989, _, _) => ("Showa", year - 1925, month, day),
            (1989..=2019, _, _) => ("Heisei", year - 1988, month, day),
            (2019..=i32::MAX, _, _) => ("Reiwa", year - 2018, month, day),
            _ => (null, year, month, day), // If not in any era, output the original date
        };

        // Output the result
        if era_name.is_null() {
            println!("{} {} {}", adjusted_year, adjusted_month, adjusted_day);
        } else {
            println!("{} {} {} {}", era_name, adjusted_year, adjusted_month, adjusted_day);
        }
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input until EOF, processing each line as a separate date input.
2. **Error Handling**: Invalid inputs are skipped without explicit error messages, as per the specification's lack of defined behavior for such cases.
3. **Era Calculation**: The program uses pattern matching to determine the Japanese era based on the year, adjusting the year accordingly.
4. **Output**: The output format is preserved as specified, either showing the era name or the adjusted date values.
5. **Safety**: The code avoids `unsafe` blocks, relying on safe Rust features for parsing and handling input.
