use proconio::input;

fn main() {
    input! {
        t: usize,
        x: i32,
        a: [i32; t + 1],
    }

    let mut last_saved_value = a[0];
    println!("0 {}", last_saved_value);

    for i in 1..=t {
        let cur_value = a[i];

        if (cur_value - last_saved_value).abs() >= x {
            println!("{} {}", i, cur_value);
            last_saved_value = cur_value;
        }
    }
}
