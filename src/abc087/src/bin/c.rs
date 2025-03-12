use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; 2],
    }

    let mut answer = 0;

    for i in 0..n {
        let mut sum = 0;
        sum += a[0][0..i + 1].iter().sum::<usize>();
        sum += a[1][i..n].iter().sum::<usize>();
        answer = answer.max(sum);
    }

    println!("{}", answer)
}
