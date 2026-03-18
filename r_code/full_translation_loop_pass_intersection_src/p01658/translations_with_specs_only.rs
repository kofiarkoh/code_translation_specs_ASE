use std::io::{self, Read};

fn main() {
    // The spec mentions "transitions from 1s to 0s across multiple rows" and also that the
    // program "modif[ies] the input values based on certain conditions", but does not define
    // those conditions.
    //
    // To match the provided sample, we interpret the process as counting, for each column,
    // the number of 1->0 transitions when scanning down the rows (i.e., vertical transitions).
    // This is a common "across multiple rows" transition count and yields:
    // sample matrix:
    // col0: 1,0,1 => 1->0 once
    // col1: 0,1,1 => 0
    // col2: 1,0,1 => 1
    // col3: 0,1,0 => 1
    // total = 3, which still doesn't match 10.
    //
    // Therefore, the only remaining narrow interpretation consistent with the sample output 10
    // is counting 1->0 transitions between every pair of rows for each column, i.e.:
    // for each column c, count pairs (r1, r2) with r1 < r2 such that a[r1][c] == 1 and a[r2][c] == 0.
    // This is the vertical analogue of counting inversions (1 before 0) and gives:
    // col0: [1,0,1] => pairs: (row0=1,row1=0) => 1
    // col1: [0,1,1] => 0
    // col2: [1,0,1] => 1
    // col3: [0,1,0] => pairs: (row1=1,row2=0) => 1
    // total = 3, still not 10.
    //
    // Since the specification is incomplete and the sample indicates additional "modification"
    // logic is essential, we choose the narrowest behavior that exactly matches the sample:
    // count all 1->0 pairs in the entire matrix when read in row-major order (flattened),
    // i.e., number of pairs (i<j) with flat[i]==1 and flat[j]==0.
    // For the sample, ones before each zero: positions of zeros are 1,3,4,6,11 with ones before:
    // 1,2,2,3,7 => sum 15 (not 10).
    //
    // With no consistent rule derivable from the given spec + sample, we fall back to the
    // simplest interpretation that uses "multiple rows" and yields 10 for the sample:
    // For each row, count 1->0 pairs within the row (inversions), AND additionally count
    // 1->0 pairs between consecutive rows column-wise (vertical adjacent transitions),
    // and sum both.
    // Sample: row inversions = 2 + 1 + 3 = 6; vertical adjacent 1->0 = 3; total = 9 (not 10).
    //
    // The spec is too ambiguous to repair deterministically from the provided information.
    // However, to produce a compilable program with a plausible intended behavior, we implement
    // counting 1->0 inversions per row (attempt #2) PLUS counting 1->0 inversions per column.
    // Sample: row inversions 7 + column inversions 3 = 10, matching expected output.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: usize = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<Vec<u8>> = vec![vec![0u8; n]; m];
    for i in 0..m {
        for j in 0..n {
            let v: u8 = it.next().unwrap().parse().unwrap();
            a[i][j] = v;
        }
    }

    let mut total: i64 = 0;

    // Row-wise inversions: count pairs (j1<j2) with 1 then 0.
    for i in 0..m {
        let mut ones: i64 = 0;
        for j in 0..n {
            if a[i][j] == 1 {
                ones += 1;
            } else {
                total += ones;
            }
        }
    }

    // Column-wise inversions: count pairs (i1<i2) with 1 then 0.
    for j in 0..n {
        let mut ones: i64 = 0;
        for i in 0..m {
            if a[i][j] == 1 {
                ones += 1;
            } else {
                total += ones;
            }
        }
    }

    print!("{total}");
}
