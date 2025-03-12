use proconio::input;

fn main() {
    input! {
        n: usize,
        fs: [(usize, usize); n],
    }

    let mut bk = vec![Vec::new(); n + 1];

    for (f, s) in fs {
        bk[f].push(s);
    }

    let mut result = 0;
    let mut best = Vec::new();

    for i in 1..=n {
        bk[i].sort_unstable();
        bk[i].reverse();
        if bk[i].len() >= 2 {
            result = result.max(bk[i][0] + bk[i][1] / 2);
        }
        if bk[i].len() >= 1 {
            best.push(bk[i][0]);
        }
    }
    best.sort_unstable();
    best.reverse();
    if best.len() >= 2 {
        result = result.max(best[0] + best[1]);
    }

    println!("{}", result)
}
