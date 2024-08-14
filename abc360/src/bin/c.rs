use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        w: [usize; n],
    }

    let mut max_weight = vec![0; n];

    for i in 0..n {
        a[i] -= 1;
        max_weight[a[i]] = max_weight[a[i]].max(w[i]);
    }

    let sum_w: usize = w.iter().sum();
    let sum_max: usize = max_weight.iter().sum();

    println!("{}", sum_w - sum_max);
}
