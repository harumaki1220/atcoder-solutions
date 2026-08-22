use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    for i in s {
        if i == 'A' {
            print!("A");
        } else {
            print!(".");
        }
    }
    println!("");
}
