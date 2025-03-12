use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        lr: [(usize, usize); m],
    }

    let mut max_l = 0;
    let mut min_r = usize::MAX;

    for (l, r) in lr {
        max_l = max_l.max(l);
        min_r = min_r.min(r);
    }

    if max_l <= min_r {
        println!("{}", min_r - max_l + 1)
    } else {
        println!("0")
    }
}
