use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut m: u32
    }

    let mut ans = Vec::new();

    for i in (0..=10).rev() {
        let a = 3u32.pow(i);
        for _ in 0..2 {
            if m >= a {
                ans.push(i);
                m -= a;
            }
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|num| num.to_string()).join(" "));
}
