use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut power_10 = Vec::new();
    for i in 0..17 {
        power_10.push(10usize.pow(i));
    }

    let mut r = vec![vec![None; 10]; 17];

    for i in 0..16 {
        let digit = (n / power_10[i]) % 10;

        for j in 0..10 {
            if j < digit {
                r[i][j] = Some((n / power_10[i + 1] + 1) * power_10[i]);
            } else if j == digit {
                r[i][j] = Some((n / power_10[i + 1]) * power_10[i] + (n % power_10[i]) + 1);
            } else {
                r[i][j] = Some((n / power_10[i + 1]) * power_10[i]);
            }
        }
    }

    let mut answer = 0;

    for i in 0..16 {
        for j in 0..10 {
            answer += j * r[i][j].unwrap();
        }
    }

    println!("{}", answer);
}
