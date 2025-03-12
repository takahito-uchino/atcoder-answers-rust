use proconio::input;

fn main() {
    input! {
        n: usize,
         mut v: [usize; n],
    }

    v.sort_unstable();

    let mut value = (v[0] as f64 + v[1] as f64) / 2.;

    for i in 2..n {
        value = (value + v[i] as f64) / 2.;
    }

    println!("{}", value)
}
