use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let num = 1 << n;

    input! {
        a: [usize; num],
    }

    let half = 1 << (n - 1);
    let max_index = a
        .iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.cmp(&b))
        .unwrap()
        .0;

    let start = if max_index < half { half } else { 0 };
    let result = (start..start + half)
        .max_by(|&i, &j| a[i].cmp(&a[j]))
        .unwrap()
        + 1;

    println!("{}", result)
}
