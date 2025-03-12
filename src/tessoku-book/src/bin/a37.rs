use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m],
    }

    let a_sum = a.iter().sum::<usize>();
    let c_sum = c.iter().sum::<usize>();

    let answer = a_sum * m + b * n * m + c_sum * n;

    println!("{}", answer);
}
