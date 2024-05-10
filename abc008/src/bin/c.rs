use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }

    let mut answer = 0.;

    for i in 0..n {
        let mut sum = 0;

        for j in 0..n {
            if i != j && c[i] % c[j] == 0 {
                sum += 1;
            }
        }

        answer += if sum % 2 == 0 {
            (sum as f64 + 2.) / (2. * sum as f64 + 2.)
        } else {
            0.5
        };
    }

    println!("{}", answer)
}
