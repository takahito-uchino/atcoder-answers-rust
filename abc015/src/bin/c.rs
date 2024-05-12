use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; k]; n],
    }

    if dfs(0, 0, &t, n, k) {
        println!("Found")
    } else {
        println!("Nothing")
    }
}

fn dfs(x: usize, num: usize, t: &Vec<Vec<usize>>, n: usize, k: usize) -> bool {
    if num == n {
        return x == 0;
    }

    (0..k).any(|i| dfs(x ^ t[num][i], num + 1, t, n, k))
}
