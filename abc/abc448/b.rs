use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [i64; m], // レストランにある在庫
        pepper: [(usize, i64); n], // (a, b)
    }

    // 各コショウの種類ごとに料理が必要としている合計量を貯める箱
    let mut needs = vec![0i64; m];

    for (a, b) in pepper {
        // 種類 a のコショウを b グラム使いたい
        needs[a - 1] += b;
    }

    let mut ans = 0i64;
    for j in 0..m {
        // 使いたい量と在庫の小さい方の分だけ実際にかけられる
        if needs[j] < c[j] {
            ans += needs[j];
        } else {
            ans += c[j];
        }
    }

    println!("{}", ans);
}