use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        queries: [(usize, usize); q],
    }
    let mut row_first_wall = vec![0; h];
    let mut col_first_wall = vec![0; w];
    let mut walls = vec![vec![true; w]; h];

    for (r, c) in queries {
        let r_idx = r - 1;
        let c_idx = c - 1;

        if walls[r_idx][c_idx] {
            // 直接の壁を削除
            walls[r_idx][c_idx] = false;
            row_first_wall[r_idx] = r_idx;
            col_first_wall[c_idx] = c_idx;
        } else {
            // 最も近い列の壁を削除
            if row_first_wall[r_idx] != r_idx {
                for i in (0..r_idx).rev() {
                    if walls[i][c_idx] {
                        walls[i][c_idx] = false;
                        row_first_wall[r_idx] = i;
                        break;
                    }
                }
            }
            // 最も近い行の壁を削除
            if col_first_wall[c_idx] != c_idx {
                for i in (0..c_idx).rev() {
                    if walls[r_idx][i] {
                        walls[r_idx][i] = false;
                        col_first_wall[c_idx] = i;
                        break;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if walls[i][j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans)
}
