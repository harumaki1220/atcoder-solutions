use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut cost = vec![vec![0i64; n]; n];

    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                c_val: i64,
            }
            cost[i][j] = c_val;
        }
    }

    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if cost[a][c] > cost[a][b] + cost[b][c] {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
