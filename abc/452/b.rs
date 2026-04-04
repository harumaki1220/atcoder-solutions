use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
    }

    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
