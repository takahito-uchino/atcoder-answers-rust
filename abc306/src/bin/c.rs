use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 3 * n],
    }

    let mut count = vec![0; n];
    let mut answer = Vec::new();

    for i in 0..3 * n {
        count[a[i] - 1] += 1;
        if count[a[i] - 1] == 2 {
            answer.push(a[i]);
        }
    }

    println!("{}", answer.iter().map(|num| num.to_string()).join(" "))
}
