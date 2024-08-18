use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
    }
    let len = n.to_string().len();
    let mut ans = 0;
    let mut lower = 1;

    for _ in 1..=len {
        let upper = lower * 10 - 1;
        let k = n.min(upper) - lower + 1;
        ans += f(k);
        ans %= MOD;
        lower *= 10;
    }
    println!("{}", ans)
}

fn f(x: usize) -> usize {
    let x_mod = x % MOD;
    x_mod * (x_mod + 1) / 2 % MOD
}
