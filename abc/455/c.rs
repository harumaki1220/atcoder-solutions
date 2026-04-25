use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let total_sum: i64 = a.iter().sum();

    let mut value_sums = HashMap::new();
    for &x in &a {
        *value_sums.entry(x).or_insert(0) += x;
    }

    let mut sum_per_value: Vec<i64> = value_sums.into_values().collect();
    sum_per_value.sort();
    sum_per_value.reverse();

    let mut reduction = 0;
    for i in 0..k.min(sum_per_value.len()) {
        reduction += sum_per_value[i];
    }

    println!("{}", total_sum - reduction);
}
