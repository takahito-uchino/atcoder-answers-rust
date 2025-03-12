use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
    }

    s.sort_unstable();

    for vec in s.iter().permutations(n) {
        let mut ok = true;
        for i in 0..n - 1 {
            let mut cnt = 0;
            for j in 0..m {
                if vec[i][j] != s[i + 1][j] {
                    cnt += 1;
                }
            }
            if cnt != 1 {
                ok = false;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
