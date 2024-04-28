use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut answer = vec![0; n];

    for i in 0..m {
        answer[a[i] - 1] += 1;
    }

    for i in answer {
        println!("{}", m - i)
    }
}
