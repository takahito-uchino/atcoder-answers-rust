use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut max_length = s[0].len();
    let max_s_len = s.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut transposed = vec![vec![]; max_s_len];

    for row in &s {
        for i in 0..max_s_len {
            if i < row.len() {
                transposed[i].push(row[i]);
            } else if i < max_length {
                transposed[i].push('*');
            } else {
                transposed[i].push('.');
            }
        }
        max_length = max_length.max(row.len());
    }

    for row in &mut transposed {
        row.reverse();
    }

    for i in 0..max_s_len {
        for j in 0..s.len() {
            if transposed[i][j] != '.' {
                print!("{}", transposed[i][j]);
            }
        }
        println!()
    }
}
