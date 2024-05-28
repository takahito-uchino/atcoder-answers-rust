use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut count = 0;
    let mut total_distance = 0.0;

    for perm in (0..n).permutations(n) {
        count += 1;
        let mut distance = 0.0;
        for i in 0..n - 1 {
            let (x1, y1) = xy[perm[i]];
            let (x2, y2) = xy[perm[i + 1]];
            distance += ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        }
        total_distance += distance;
    }

    let average = total_distance / count as f64;

    println!("{}", average)
}
