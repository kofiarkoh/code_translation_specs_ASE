// translated from the provided C code, preserving behavior (including quirks) as closely as possible.
// Notes about choices made to match the original semantics:
// - The C program read whitespace-separated pairs "%s %d" until EOF. Here we read all whitespace tokens
//   and consume them in pairs (word, int). If the final word has no integer following, it is ignored.
// - Sorting: the original code attempted to keep the list lexicographically ordered while inserting.
//   The final effect is that the entire list is lexicographically sorted. We replicate that by sorting
//   the pair list by the word (stable sort).
// - The C program used "null" to mark duplicate entries after sorting; we replicate that exact string.
// - The C code used an uninitialized 'page' array in places, but the intended behavior treats unused
//   cells as zero (they test for == 0 as end sentinel). We therefore initialize page cells to 0.
// - When printing, the original code prints the dictionary word on its own line, then prints page
//   numbers starting from index 1 up to (i-s)-1, breaking and printing a newline when a zero is seen.
//   If the loop completes without encountering a zero, no newline is printed (this replicates the C bug).
//   We reproduce that printing behavior exactly (including when i-s == 1, the inner loop doesn't run).
use std::io::{self, Read};

fn main() {
    // Read all input and split into whitespace tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Consume tokens in pairs (word, int). If integer parsing fails or there's no pair, stop.
    let mut words: Vec<String> = Vec::new();
    let mut nums: Vec<i32> = Vec::new();
    let mut idx = 0usize;
    while idx + 1 < tokens.len() {
        let w = tokens[idx].to_string();
        if let Ok(n) = tokens[idx + 1].parse::<i32>() {
            words.push(w);
            nums.push(n);
            idx += 2;
        } else {
            break;
        }
    }

    // Pair and sort lexicographically by word (stable)
    let mut pairs: Vec<(String, i32)> = words.into_iter().zip(nums.into_iter()).collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0)); // lexicographic ascending

    // Re-extract words and numbers after sorting
    let i = pairs.len(); // original count
    let mut wordv: Vec<String> = pairs.iter().map(|(w, _)| w.clone()).collect();
    let mut numv: Vec<i32> = pairs.iter().map(|(_, n)| *n).collect();

    // Mark contiguous duplicates by setting the later ones to the exact string "null"
    for l in 0..i {
        let mut m = l + 1;
        while m < i {
            if wordv[l] != "null" {
                if wordv[l] == wordv[m] {
                    wordv[m] = "null".to_string();
                } else {
                    break;
                }
            }
            m += 1;
        }
    }

    // Build dict (unique words) and mark array (100 for those set to "null")
    let mut dict: Vec<String> = Vec::new();
    let mut mark = vec![0i32; i];
    for l in 0..i {
        if wordv[l] != "null" {
            dict.push(wordv[l].clone());
        } else {
            mark[l] = 100;
        }
    }

    // page matrix: initialized to zeros (match intended sentinel behavior in C)
    // dimensions i x i (matches original using i as bounds in many places)
    let mut page = vec![vec![0i32; i]; i];

    // Populate page using the same index arithmetic as the C code:
    // when mark[k]==0 => new dict entry at row (k - s), column 0 gets num[k]
    // when mark[k]!=0 => a duplicate: increment r and s and place num at page[k-s][r]
    let mut r: i32 = 0;
    let mut s: usize = 0;
    for k in 0..i {
        if mark[k] == 0 {
            r = 0;
            page[k - s][0] = numv[k];
        } else {
            r += 1;
            s += 1;
            page[k - s][r as usize] = numv[k];
        }
    }

    // Number of distinct dictionary entries (i - s)
    let n_dict = i.saturating_sub(s);

    // For each populated row, sort the non-zero prefix ascending.
    // The original C code bubbled until sorted but only up to encountering a zero sentinel.
    for k in 0..n_dict {
        // find first zero index (or i if none)
        let mut pos = i;
        for t in 0..i {
            if page[k][t] == 0 {
                pos = t;
                break;
            }
        }
        // sort only the non-zero prefix [0..pos)
        if pos > 0 {
            page[k][..pos].sort();
        }
    }

    // Reproduce printing behavior exactly:
    // for k in 0..i-s:
    //   printf("%s\n", dict[k]);
    //   for t=1; t<i-s; t++:
    //     if t==1: printf("%d", page[k][t]);
    //     else if page[k][t] != 0: printf(" %d", page[k][t]);
    //     else { printf("\n"); break; }
    // Note: if the inner loop completes without encountering a zero, no newline is printed.
    let mut out = String::new();
    for k in 0..n_dict {
        out.push_str(&format!("{}\n", dict[k]));
        // t runs 1..(i-s-1) inclusive in C with condition t < i-s
        let mut t = 1usize;
        while t < n_dict {
            if t == 1 {
                if page[k][t] != 0 {
                    out.push_str(&format!("{}", page[k][t]));
                } else {
                    out.push_str("\n");
                    break;
                }
            } else {
                if page[k][t] != 0 {
                    out.push_str(&format!(" {}", page[k][t]));
                } else {
                    out.push_str("\n");
                    break;
                }
            }
            t += 1;
        }
        // If the inner loop did not run at all (e.g., n_dict == 1), it does nothing,
        // which matches the original C behavior (no extra newline).
    }

    // Emit final output exactly as constructed
    print!("{}", out);
}
