use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut a = vec![vec![false; n]; n];
    let mut b = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        a[u - 1][v - 1] = true;
        a[v - 1][u - 1] = true;
    }

    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        b[u - 1][v - 1] = true;
        b[v - 1][u - 1] = true;
    }

    let mut answer = false;
    for p in (0..n).permutations(n) {
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] != b[p[i]][p[j]] {
                    ok = false;
                }
            }
        }
        if ok {
            answer = true;
        }
    }
    println!("{}", if answer { "Yes" } else { "No" })
}
