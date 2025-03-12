use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut answer = n;

    for bit in 0..1 << n {
        let mut exist = vec![false; m];
        let mut count = 0;
        for i in 0..n {
            if bit >> i & 1 == 1 {
                count += 1;
                for j in 0..m {
                    if s[i][j] == 'o' {
                        exist[j] = true;
                    }
                }
            }
        }
        let all_exist = exist.iter().all(|&x| x);
        if all_exist {
            answer = answer.min(count);
        }
    }

    println!("{}", answer);
}
