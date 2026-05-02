use proconio::input;

fn main() {
    input! {
        a: [[i32; 6]; 3],
    }

    let mut count = 0;

    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                let mut results = [a[0][i], a[1][j], a[2][k]];
                results.sort();

                if results == [4, 5, 6] {
                    count += 1;
                }
            }
        }
    }

    println!("{:.10}", count as f64 / 216.0);
}
