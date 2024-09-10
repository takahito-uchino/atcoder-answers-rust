use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    }

    let mut ans = 0;

    for i1 in 0..9 {
        for j1 in 0..9 {
            for i2 in 0..9 {
                for j2 in 0..9 {
                    if i2 > i1 && j2 >= j1 && s[i1][j1] == '#' && s[i2][j2] == '#' {
                        let di = i2 as isize - i1 as isize;
                        let dj = j2 as isize - j1 as isize;
                        let i3 = i2 as isize + dj;
                        let j3 = j2 as isize - di;
                        let i4 = i3 - di;
                        let j4 = j3 - dj;

                        if i3 >= 0
                            && i3 < 9
                            && j3 >= 0
                            && j3 < 9
                            && i4 >= 0
                            && i4 < 9
                            && j4 >= 0
                            && j4 < 9
                        {
                            if s[i3 as usize][j3 as usize] == '#'
                                && s[i4 as usize][j4 as usize] == '#'
                            {
                                ans += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
