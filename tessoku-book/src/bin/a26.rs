use proconio::input;

fn main() {
    input! {
        q: usize,
        x: [usize; q],
    }

    let mut deleted = vec![false; 300001];

    for i in 2..=((300000 as f64).sqrt() as usize) {
        if deleted[i] {
            continue;
        }
        for j in (i * 2..=300000).step_by(i) {
            deleted[j] = true;
        }
    }

    for xi in x {
        println!("{}", if !deleted[xi] { "Yes" } else { "No" })
    }
}
