use std::isize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    }

    let mut sum = Vec::new();
    sum.push(a[0]);

    for i in 1..=n {
        sum.push(sum[i - 1] + a[i - 1]);
    }

    let mut sumi = Vec::new();
    let mut now = 0;
    for i in 0..m {
        now += a[i] * (i + 1) as isize;
    }
    sumi.push(now);

    for i in 1..n - m + 1 {
        sumi.push(sumi[i - 1] + m as isize * a[m + i - 1] - (sum[m + i - 1] - sum[i - 1]));
    }

    let mut ans = isize::MIN;

    for i in 0..n - m + 1 {
        ans = ans.max(sumi[i]);
    }

    println!("{}", ans)
}
