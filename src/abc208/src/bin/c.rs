use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| a[i].cmp(&a[j]));

    let mut answer = vec![k / n; n];
    for i in 0..(k % n) {
        answer[order[i]] += 1;
    }

    for ans in answer {
        println!("{}", ans);
    }
}

