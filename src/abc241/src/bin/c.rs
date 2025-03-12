use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" })
}

fn judge(sr: usize, sc: usize, dr: isize, dc: isize, n: usize, s: &Vec<Vec<i32>>) -> bool {
    let (mut row, mut col) = (sr as isize, sc as isize);
    let mut bl = 0;
    for _ in 0..6 {
        if row < 0 || row >= n as isize || col < 0 || col >= n as isize {
            return false;
        }
        bl += s[row as usize][col as usize];
        row += dr;
        col += dc;
    }
    bl >= 4
}

fn solve() -> bool {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut s_matrix = vec![vec![0; n]; n];

    for (i, line) in s.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            s_matrix[i][j] = if c == '#' { 1 } else { 0 };
        }
    }

    let pat = [(1, 0), (0, 1), (1, -1), (1, 1)];

    for row in 0..n {
        for col in 0..n {
            for &(dr, dc) in &pat {
                if judge(row, col, dr, dc, n, &s_matrix) {
                    return true;
                }
            }
        }
    }
    false
}
