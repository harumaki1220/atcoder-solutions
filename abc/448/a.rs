use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: i32,
        a: [i32; n],
    }

    for i in a.iter() {
        if i < &x {
            x = *i;
            println!("1");
        } else {
            println!("0");
        }
    }
}
