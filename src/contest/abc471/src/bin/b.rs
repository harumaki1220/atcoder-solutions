use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut counts = HashMap::new();

    for word in s {
        let lower = word.to_ascii_lowercase();
        *counts.entry(lower).or_insert(0) += 1;
    }

    let ans = counts.values().max().unwrap();
    println!("{}", ans);
}
