use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let r#mod = 1000000007;

    let mut sum = 0;

    for i in &a {
        sum += i;
        sum %= r#mod;
    }

    let mut answer = 0;

    for i in 0..n {
        if a[i] > sum {
            sum = (sum + r#mod - a[i]) % r#mod;
        } else {
            sum -= a[i];
        }

        answer = (answer + a[i] * sum) % r#mod;
    }

    println!("{}", answer)
}
