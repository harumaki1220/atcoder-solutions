use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [i64; n]
    }

    let mut calories: Vec<i64> = vec![0; n];
    let mut sum: i64 = 0;

    for i in 0..n {
        if i >= m {
            sum -= calories[i - m];
        }

        if sum + a[i] <= k {
            println!("Yes");
            calories[i] = a[i];
            sum += a[i];
        } else {
            println!("No");
        }
    }
}
