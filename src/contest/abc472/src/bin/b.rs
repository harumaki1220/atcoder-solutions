use std::{cmp::min, i32};

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i32; n]
    }

    let total_sum: i32 = l.iter().sum();
    let mut left_sum = 0;
    let mut min_diff = i32::MAX;

    for i in l {
        left_sum += i;
        let right_sum = total_sum - left_sum;
        let diff = (left_sum - right_sum).abs();
        min_diff = min(min_diff, diff);
    }
    println!("{}", min_diff);
}
