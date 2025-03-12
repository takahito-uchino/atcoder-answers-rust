use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut a = Vec::new();
    let mut b = Vec::new();

    for _ in 0..n {
        input! {
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
    }

    let mut answer = 0;

    for ai in 1..=100 {
        for bi in 1..=100 {
            let mut count = 0;
            for i in 0..n {
                if ai <= a[i] && a[i] <= ai + k && bi <= b[i] && b[i] <= bi + k {
                    count += 1;
                } 
            }
            answer = answer.max(count);
        }
    }

    println!("{}", answer);
}

