use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        items: [(usize, usize); m],
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for (a, b) in items {
        adj[a].push(b);
    }

    let mut visited = vec![false; n + 1];
    let mut queue = VecDeque::new();

    visited[1] = true;
    queue.push_back(1);

    let mut count = 0;
    while let Some(curr) = queue.pop_front() {
        count += 1;

        for &next_item in &adj[curr] {
            if !visited[next_item] {
                visited[next_item] = true;
                queue.push_back(next_item);
            }
        }
    }

    println!("{}", count);
}
