use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ret1 = vec![None; n];
    let mut ret2 = vec![None; n];

    let mut streak1 = 1;
    ret1[0] = Some(1);

    for i in 0..n-1 {
        if s[i] == 'A' {
            streak1 += 1;
        } else {
            streak1 = 1;
        }
        ret1[i + 1] = Some(streak1);
    }

    let mut streak2 = 1;
    ret2[n - 1] = Some(1);

    for i in (0..n-1).rev() {
        if s[i] == 'A' {
            streak2 = 1;
        } else {
            streak2 += 1;
        }
        ret2[i] = Some(streak2);
    }

    let mut answer = 0;

    for i in 0..n {
        answer += ret1[i].unwrap().max(ret2[i].unwrap());
    }

    println!("{}", answer);
}
