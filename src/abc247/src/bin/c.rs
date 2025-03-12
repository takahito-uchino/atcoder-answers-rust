use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let answer = connect(n);

    println!("{}", answer.iter().map(|&num| num.to_string()).join(" "))
}

fn connect(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    } else {
        let mut result = connect(n - 1);
        result.push(n);
        result.append(&mut connect(n - 1));
        return result;
    }
}
