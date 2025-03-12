use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_by(|a, b| b.0.cmp(&a.0));

    let mut answer = 0;

    for (a, b) in ab {
        answer += a * b.min(w);
        w -= b.min(w);
    }

    println!("{}", answer)
}
