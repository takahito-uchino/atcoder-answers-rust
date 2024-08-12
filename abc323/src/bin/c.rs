use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        s: [Chars; n],
    }

    let mut now_scores = vec![0; n];
    for i in 0..n {
        now_scores[i] = i + 1;
    }

    for i in 0..n {
        for j in 0..m {
            if s[i][j] == 'o' {
                now_scores[i] += a[j];
            }
        }
    }

    let &max = now_scores.iter().max().unwrap();

    for i in 0..n {
        let mut nokori = Vec::new();
        for j in 0..m {
            if s[i][j] == 'x' {
                nokori.push(a[j]);
            }
        }
        nokori.sort_unstable();
        nokori.reverse();
        let mut answer = 0;
        while now_scores[i] < max {
            now_scores[i] += nokori[answer];
            answer += 1;
        }
        println!("{}", answer);
    }
}
