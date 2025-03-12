use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();

    for i in 1..=n {
        input! {
            a_i: [usize; i],
        }
        a.push(a_i);
    }

    let mut now = 1;

    for i in 1..=n {
        if now >= i {
            now = a[now - 1][i - 1];
        } else {
            now = a[i - 1][now - 1];
        }
    }

    println!("{}", now)
}
