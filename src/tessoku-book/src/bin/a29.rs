use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut p = a;
    let mut answer = 1;

    for i in 0..30 {
        let wari = 1 << i;
        if (b / wari) % 2 == 1 {
            answer = (answer * p) % 1000000007;
        }
        p = (p * p) % 1000000007;
    }

    println!("{}", answer);
}
