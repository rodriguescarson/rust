use std::cmp::max;
use std::time::Instant;

fn lcs_rust(s1: &str, s2: &str) -> (String, f64) {
    let start_time = Instant::now();

    let mut table = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if &s1[i - 1..i] == &s2[j - 1..j] {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = max(table[i - 1][j], table[i][j - 1]);
            }
        }
    }

    let mut lcs = String::new();
    let mut i = s1.len();
    let mut j = s2.len();

    while i > 0 && j > 0 {
        if table[i][j] == table[i - 1][j] {
            i -= 1;
        } else if table[i][j] == table[i][j - 1] {
            j -= 1;
        } else {
            lcs.push_str(&s1[i - 1..i]);
            i -= 1;
            j -= 1;
        }
    }

    lcs = lcs.chars().rev().collect();

    let end_time = Instant::now();

    let execution_time = end_time.duration_since(start_time).as_secs_f64();

    (lcs, execution_time)
}

// Example usage:

fn main() {
    let s1 = "ABCGFSDE";
    let s2 = "FGdfDFSfsdgfsHAB";

    let (lcs, execution_time) = lcs_rust(s1, s2);

    println!("LCS: {}", lcs);
    println!("Execution time: {:.3} seconds", execution_time);
}
