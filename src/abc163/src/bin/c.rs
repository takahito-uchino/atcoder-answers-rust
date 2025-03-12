use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
    }

    let mut answer = vec![0; n];

    for i in a {
        answer[i - 1] += 1;
    }

    for i in 0..n {
        println!("{}", answer[i])
    }
}
