use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(String, isize); n],
    }

    let mut t = Vec::new();
    let mut a = Vec::new();

    for (ti, ai) in ta {
        t.push(ti);
        a.push(ai);
    }

    let mut answer = 0;

    for i in 0..n {
        if t[i] == "+" {
            answer += a[i];
        } else if t[i] == "-" {
            answer -= a[i];
        } else {
            answer *= a[i];
        }

        answer %= 10000;
        if answer < 0 {
            println!("{}", 10000 + answer);
        } else {
            println!("{}", answer);
        }
    }
}
