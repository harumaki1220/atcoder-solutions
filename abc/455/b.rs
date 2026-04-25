use proconio::{input, marker::Chars};

fn is_point_symmetric(r1: usize, r2: usize, c1: usize, c2: usize, s: &Vec<Vec<char>>) -> bool {
    for i in r1..=r2 {
        for j in c1..=c2 {
            if s[i][j] != s[r1 + r2 - i][c1 + c2 - j] {
                return false;
            }
        }
    }
    true
}
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut count = 0;

    for r1 in 0..h {
        for r2 in r1..h {
            for c1 in 0..w {
                for c2 in c1..w {
                    if is_point_symmetric(r1, r2, c1, c2, &s) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}
