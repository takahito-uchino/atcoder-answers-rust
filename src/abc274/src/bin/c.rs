use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut answer = vec![0; 2 * n + 1];

    for (i, v) in a.iter().enumerate() {
        answer[2 * i + 1] = answer[v - 1] + 1;
        answer[2 * i + 2] = answer[v - 1] + 1;
    }

    for ans in answer {
        println!("{}", ans)
    }
}
