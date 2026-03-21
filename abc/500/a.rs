use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    for i in (1..=n).rev() {
        print!("{i}");
        if i != 1 {
            print!(",");
        }
    }
}
