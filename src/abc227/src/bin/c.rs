use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer: usize = 0;

    let mut a = 1;
    while a * a * a <= n {
        let mut b = a;
        while a * b * b <= n {
            answer += n / a / b - b + 1;
            b += 1;
        }
        a += 1;
    }

    println!("{}", answer)
}
