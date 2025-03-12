use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut cnt = vec![0; n];

    for i in 0..n {
        for j in 0..3 {
            cnt[(p[i] + n - 1 - i + j) % n] += 1;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        ans = ans.max(cnt[i]);
    }

    println!("{}", ans)
}
