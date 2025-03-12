use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut deleted = vec![false; n + 1];

    for i in 2..=((n as f64).sqrt() as usize) {
        if deleted[i] {
            continue;
        }
        for j in (i * 2..=n).step_by(i) {
            deleted[j] = true;
        }
    }

    for i in 2..=n {
        if !deleted[i] {
            println!("{}", i);
        }
    }
}
