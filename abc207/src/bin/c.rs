use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut l = Vec::with_capacity(n);
    let mut r = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            t: usize,
            mut li: f64,
            mut ri: f64,
        }

        if t == 2 {
            ri -= 0.5;
        } else if t == 3 {
            li += 0.5;
        } else if t == 4 {
            li += 0.5;
            ri -= 0.5;
        }

        l.push(li);
        r.push(ri);
    }

    let mut answer = 0;

    for i in 0..n {
        for j in i + 1..n {
            if l[i].max(l[j]) <= r[i].min(r[j]) {
                answer += 1;
            }
        }
    }

    println!("{}", answer);
}

