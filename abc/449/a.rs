use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        d: f64,
    }

    let radius = d / 2.0;
    let area = radius * radius * PI;

    println!("{}", area);
}
