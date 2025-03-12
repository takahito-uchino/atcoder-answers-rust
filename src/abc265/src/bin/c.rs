use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    }

    let mut i = 0;
    let mut j = 0;
    let mut vis = vec![vec![false; w]; h];

    loop {
        if vis[i][j] == true {
            println!("-1");
            return;
        }
        vis[i][j] = true;
        if g[i][j] == 'U' && i != 0 {
            i -= 1;
        } else if g[i][j] == 'D' && i != h - 1 {
            i += 1;
        } else if g[i][j] == 'L' && j != 0 {
            j -= 1;
        } else if g[i][j] == 'R' && j != w - 1 {
            j += 1;
        } else {
            break;
        }
    }

    println!("{} {}", i + 1, j + 1);
}
