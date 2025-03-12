use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut sum1 = 0;

    for i in 0..n {
        sum1 += a[i] * a[i];
    }

    sum1 *= n as isize;

    let sum2 = a.iter().sum::<isize>().pow(2);

    println!("{}", sum1 - sum2)
}
