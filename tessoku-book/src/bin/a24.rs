use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![0; n];
    let mut len = 0;
    let mut l = Vec::new();

    for i in 0..n {
        let pos = l.binary_search(&a[i]).unwrap_or_else(|x| x);
        dp[i] = pos;

        if dp[i] >= len {
            l.push(a[i]);
            len += 1;
        } else {
            l[pos] = a[i];
        }
    }

    println!("{}", len);
}
