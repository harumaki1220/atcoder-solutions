use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a + b == 9 || a - b == 9 || a * b == 9 || a == 9 * b {
        println!("Nine");
    } else {
        println!("Nein")
    }
}
