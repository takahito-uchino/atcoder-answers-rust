use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let m = 1000000007;

    let mut p = a % m;
    let mut answer = 1;

    for i in 0..60 {
        let wari = 1 << i;
        if (b / wari) % 2 == 1 {
            answer = (answer * p) % m;
        }
        p = (p * p) % m;
    }

    println!("{}", answer);
}
