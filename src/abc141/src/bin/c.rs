use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        q: usize,
        a: [usize; q],
    }

    let base = k - q as isize;
    let mut scores = vec![base; n];

    for ai in a {
        scores[ai - 1] += 1;
    }

    for score in scores {
        println!("{}", if score > 0 { "Yes" } else { "No" })
    }
}
