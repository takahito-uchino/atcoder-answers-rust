use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n],
    }

    let mut scores = Vec::new();

    for score in p {
        scores.push(score.iter().sum::<usize>());
    }

    let mut rank = scores.clone();
    rank.sort_unstable();
    let border = rank[n - k];

    for i in 0..n {
        if scores[i] + 300 >= border {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
