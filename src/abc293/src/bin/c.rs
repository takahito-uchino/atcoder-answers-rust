use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut ans = 0;

    for v in (0..h + w - 2).combinations(h - 1) {
        let mut now_h = 0;
        let mut now_w = 0;
        let mut passed = HashSet::new();
        passed.insert(a[now_h][now_w]);
        for i in 0..h + w - 2 {
            if v.contains(&i) {
                now_h += 1;
            } else {
                now_w += 1;
            }
            passed.insert(a[now_h][now_w]);
        }
        if passed.len() == h + w - 1 {
            ans += 1;
        }
    }

    println!("{}", ans)
}
