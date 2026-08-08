use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n]
    }

    let mut vec = vec![0; n + 1];

    for i in c {
        vec[i] += 1;
    }

    vec.sort();
    vec.reverse();
    println!("{}", n - vec[0]);
}
