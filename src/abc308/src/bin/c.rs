use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut ids: Vec<usize> = (0..n).collect();

    ids.sort_by(|&i, &j| {
        let left = ab[i].0 * (ab[j].0 + ab[j].1);
        let right = ab[j].0 * (ab[i].0 + ab[i].1);
        right.cmp(&left)
    });

    for (i, id) in ids.iter().enumerate() {
        if i != n - 1 {
            print!("{} ", id + 1);
        } else {
            println!("{}", id + 1);
        }
    }
}
