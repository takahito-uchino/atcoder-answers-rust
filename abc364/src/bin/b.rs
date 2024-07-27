use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut si: usize,
        mut sj: usize,
        c: [Chars; h],
        x: Chars,
    }
    si -= 1;
    sj -= 1;

    for char in x {
        if char == 'L' && sj > 0 && c[si][sj - 1] == '.' {
            sj -= 1;
        } else if char == 'R' && sj < w - 1 && c[si][sj + 1] == '.' {
            sj += 1;
        } else if char == 'U' && si > 0 && c[si - 1][sj] == '.' {
            si -= 1;
        } else if char == 'D' && si < h - 1 && c[si + 1][sj] == '.' {
            si += 1;
        }
    }

    println!("{} {}", si + 1, sj + 1)
}
