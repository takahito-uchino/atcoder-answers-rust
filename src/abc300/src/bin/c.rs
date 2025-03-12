use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h],
    }

    let mut ans = vec![0; usize::min(h, w)];

    for y in 0..h {
        for x in 0..w {
            if c[y][x] == '#' {
                let count = dfs(&mut c, y, x, h, w) / 4;
                if count > 0 && count - 1 < ans.len() {
                    ans[count - 1] += 1;
                }
            }
        }
    }

    println!("{}", ans.iter().map(|num| num.to_string()).join(" "))
}

fn dfs(c: &mut Vec<Vec<char>>, y: usize, x: usize, h: usize, w: usize) -> usize {
    let mut cnt = 1;
    c[y][x] = '.';
    for dy in -1..=1 {
        for dx in -1..=1 {
            let y2 = (y as isize + dy) as usize;
            let x2 = (x as isize + dx) as usize;
            if (dy != 0 || dx != 0) && y2 < h && x2 < w && c[y2][x2] == '#' {
                cnt += dfs(c, y2, x2, h, w);
            }
        }
    }
    cnt
}
