use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = usize::MAX;

    for i in 1..=1000000 {
        if n % i == 0 {
            ans = ans.min(i + n / i - 2);
        }
    }

    println!("{}", ans)
}
