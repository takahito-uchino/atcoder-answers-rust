use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut count = vec![vec![0; 10]; 10];

    for i in 0..n {
        for j in 0..10 {
            count[s[i][j] as usize - 48][j] += 1;
        }
    }

    let mut max = vec![0; 10];

    for i in 0..10 {
        for j in 0..10 {
            if count[i][j] > 0 {
                max[i] = max[i].max(10 * (count[i][j] - 1) + j);
            }
        }
    }

    println!("{}", max.iter().min().unwrap());
}
