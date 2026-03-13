// concordance.rs
//
// Translates the provided specification into Rust.
// Behavior preserved (per specification):
// - Reads whitespace-separated pairs (word, integer) from stdin until EOF.
// - Skips any input word equal to the literal "null" (treated as a deletion marker).
// - Keeps up to 128 input pairs (follows the fixed-capacity nature described).
// - Sorts entries by word (lexicographically).
// - Collapses adjacent duplicate words by marking duplicates (keeps first occurrence).
// - Builds per-word page-number arrays: first occurrence stored at index 0, duplicates at higher indices.
// - Performs an in-place ascending bubble-sort of the full fixed-size page array (128 entries).
// - When printing, for each distinct word prints the word on its own line, then prints integers
//   starting at index 1 of the page array. Each printed integer is emitted before checking
//   whether it is zero; if it is zero the zero is printed and the line ends (matching the
//   specification's described behavior where a zero sentinel may be printed).
//
// Notes about choices made to keep behavior deterministic and safe in Rust:
// - The original C had undefined behavior in sorting due to an uninitialized variable.
//   This translation implements the intended sort (deterministic lexicographic sort).
// - The spec allows up to 128 entries and 128 page numbers per word; we emulate that with fixed sizes.
// - If input parsing encounters a missing integer for the final word token or an integer parse error,
//   scanning stops and previously read pairs are processed (matching scanf behavior).
//
// The program exits normally (returns 0 implicitly).

use std::io::{self, Read};
use std::cmp::Ordering;

const MAX_ENTRIES: usize = 128;
const PAGE_PER_WORD: usize = 128;

fn main() {
    // Read entire stdin and split on whitespace tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, behave like no input.
        return;
    }

    let mut tokens = input.split_whitespace();

    // Temporary storage for words and numbers (up to MAX_ENTRIES)
    let mut words: Vec<String> = Vec::new();
    let mut nums: Vec<i32> = Vec::new();

    // Read pairs: word then integer. Stop if integer missing or parse fails.
    while let Some(word_tok) = tokens.next() {
        // Try to get the integer token; if missing, stop scanning (like scanf on EOF/mismatch).
        let int_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Try parse integer; if fails, stop scanning (mimic scanf mismatch behavior).
        let n: i32 = match int_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // If word is literal "null", treat it as removed/marked and do not store it.
        if word_tok == "null" {
            continue;
        }

        if words.len() >= MAX_ENTRIES {
            // The original program allocated for 128 entries and had undefined behavior
            // if exceeded. Here we simply stop accepting more entries to stay deterministic.
            break;
        }

        words.push(word_tok.to_string());
        nums.push(n);
    }

    let n = words.len();

    if n == 0 {
        // Nothing to do; exit normally.
        return;
    }

    // Pair up and sort by word lexicographically (stable).
    let mut pairs: Vec<(String, i32)> = words.into_iter().zip(nums.into_iter()).collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // After sorting, collapse adjacent duplicates: mark duplicates by replacing their word with "null"
    // and leaving the first occurrence intact.
    // We'll store the marked words in a vector of strings to mirror the original behavior.
    let mut marked_words: Vec<String> = pairs.iter().map(|(w, _)| w.clone()).collect();
    for i in 1..n {
        if marked_words[i] == marked_words[i - 1] {
            // Mark duplicate by setting to "null"
            marked_words[i] = "null".to_string();
            // (Spec mentions mark[] = 100; that internal detail is not externally visible.)
        }
    }

    // Build dict[] of non-"null" words and page[][] arrays.
    // page per word is a fixed-size array PAGE_PER_WORD initialized to zeros.
    let mut dict: Vec<String> = Vec::new();
    let mut pages: Vec<[i32; PAGE_PER_WORD]> = Vec::new();
    let mut next_pos: Vec<usize> = Vec::new(); // next free index within each page row

    // We'll iterate through the sorted pairs and marked_words in order.
    for i in 0..n {
        let word = &marked_words[i];
        let (_, num) = &pairs[i];

        if word != "null" {
            // New dictionary entry.
            dict.push(word.clone());
            let mut row = [0i32; PAGE_PER_WORD];
            row[0] = *num; // store initial occurrence at index 0
            pages.push(row);
            next_pos.push(1); // next free index is 1
        } else {
            // Duplicate: associate this num with the most recent dict entry.
            // Because duplicates were adjacent after sorting, the owner is the last dict entry.
            if let Some(last_idx) = dict.len().checked_sub(1) {
                let pos = next_pos[last_idx];
                if pos < PAGE_PER_WORD {
                    pages[last_idx][pos] = *num;
                    next_pos[last_idx] = pos + 1;
                } else {
                    // If the per-word page array is full (unlikely given constraints), ignore extra numbers.
                }
            } else {
                // This shouldn't happen: a duplicate without a prior non-null entry.
                // We'll ignore such a case to remain safe.
            }
        }
    }

    // For each page row, perform an in-place ascending bubble-sort over the entire PAGE_PER_WORD.
    // This mirrors the intended page-number sorting (and interacts with the zero sentinels).
    for row in pages.iter_mut() {
        // Simple bubble sort O(m^2) where m = PAGE_PER_WORD (128).
        // This is deterministic and mirrors the spec's described behavior.
        for a in 0..PAGE_PER_WORD {
            let mut swapped = false;
            for b in 0..(PAGE_PER_WORD - 1 - a) {
                if row[b] > row[b + 1] {
                    let tmp = row[b];
                    row[b] = row[b + 1];
                    row[b + 1] = tmp;
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }

    // Printing:
    // For each distinct word (in dict order), print the word then newline,
    // then print integers starting at index 1 of its page row. The first printed integer
    // is printed without a leading space; subsequent integers prefixed by a space.
    // After printing each integer, if that integer is zero, print a newline and stop the line.
    // This reproduces the described observable I/O behavior where the first stored integer (at index 0)
    // is not printed and a zero sentinel may be printed as the first visible integer.
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    use std::io::Write;

    for (i, word) in dict.iter().enumerate() {
        // Print word
        let _ = writeln!(handle, "{}", word);

        let row = &pages[i];

        // Print starting from index 1
        let mut first_printed = false;
        for j in 1..PAGE_PER_WORD {
            if !first_printed {
                // Print first shown integer without leading space.
                let _ = write!(handle, "{}", row[j]);
                first_printed = true;
            } else {
                // Subsequent integers prefixed by a space.
                let _ = write!(handle, " {}", row[j]);
            }

            // After printing the integer, if it is zero, end the line and stop.
            if row[j] == 0 {
                let _ = writeln!(handle);
                break;
            }

            // If we reached the last element without encountering zero, still terminate the line.
            if j + 1 == PAGE_PER_WORD {
                let _ = writeln!(handle);
            }
        }

        // Edge case: if PAGE_PER_WORD <= 1 (not the case here), ensure newline.
        if !first_printed {
            let _ = writeln!(handle);
        }
    }
}
