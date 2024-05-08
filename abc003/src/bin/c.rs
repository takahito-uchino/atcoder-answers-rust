use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut r: [usize; n],
    }

    r.sort_unstable();

    let mut score = 0.;

    for i in (0..k).rev() {
        score = (score + r[r.len() - i - 1] as f64) / 2.;
    }

    println!("{}", score)
}
