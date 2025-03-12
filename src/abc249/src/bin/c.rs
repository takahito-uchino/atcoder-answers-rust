use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }

    let mut ans = 0;

    for i in 0..(1 << n) {
        let mut sum = vec![0; 26];
        for j in 0..n {
            if (i >> j) & 1 != 0 {
                for x in 0..s[j].len() {
                    sum[s[j][x] as usize - 'a' as usize] += 1;
                }
            }
        }

        let mut now = 0;
        for j in 0..26 {
            if sum[j] == k {
                now += 1;
            }
        }
        ans = ans.max(now);
    }
    println!("{}", ans)
}
