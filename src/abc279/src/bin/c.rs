use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }

    let mut s_rev = vec![vec!['.'; h]; w];
    let mut t_rev = vec![vec!['.'; h]; w];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                s_rev[j][i] = '#';
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if t[i][j] == '#' {
                t_rev[j][i] = '#';
            }
        }
    }

    s_rev.sort_unstable();
    t_rev.sort_unstable();

    if s_rev == t_rev {
        println!("Yes");
    } else {
        println!("No");
    }
}
