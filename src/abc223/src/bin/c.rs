use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }

    let mut total_time = 0.;

    for (a, b) in &ab {
        total_time += a / b;
    }

    total_time /= 2.0;

    let mut answer = 0.;

    for i in 0..n {
        let min_val = ab[i].0.min(total_time * ab[i].1);
        answer += min_val;
        total_time -= (ab[i].0 / ab[i].1).min(total_time);
    }

    println!("{:.15}", answer);
}
