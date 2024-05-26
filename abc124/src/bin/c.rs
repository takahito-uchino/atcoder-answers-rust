use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut answer = usize::MAX;
    let n = s.len();

    {
        let mut count = 0;
        for i in 0..n {
            if (s[i] as usize - '0' as usize) == i % 2 {
                count += 1;
            }
        }
        answer = answer.min(count);
    }

    {
        let mut count = 0;
        for i in 0..n {
            if (s[i] as usize - '0' as usize) != i % 2 {
                count += 1;
            }
        }
        answer = answer.min(count);
    }

    println!("{}", answer)
}
