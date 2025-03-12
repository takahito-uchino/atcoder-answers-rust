use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    }

    let mut x = vec![vec![0; m]; n];
    let mut y = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            x[i][j] = (b[i][j] + 6) / 7;
            y[i][j] = (b[i][j] - 1) % 7 + 1;
        }
    }

    let mut answer = "Yes";

    for i in 0..n {
        for j in 0..m {
            if (i != 0 && x[i][j] != x[i - 1][j] + 1)
                || (j != 0 && y[i][j] != y[i][j - 1] + 1)
                || (j != 0 && x[i][j] != x[i][j - 1])
                || (i != 0 && y[i][j] != y[i - 1][j])
            {
                answer = "No";
            }
        }
    }

    println!("{}", answer)
}
