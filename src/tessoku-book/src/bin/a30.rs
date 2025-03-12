use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
    }

    let m = 1000000007;

    let mut a = 1;

    for i in 1..=n {
        a = (a * i) % m;
    }

    let mut b = 1;

    for i in 1..=r {
        b = (b * i) % m;
    }

    for i in 1..=n - r {
        b = (b * i) % m;
    }

    println!("{}", division(a, b, m));
}

fn power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut answer = 1;

    for i in 0..30 {
        let wari = 1 << i;
        if (b / wari) % 2 == 1 {
            answer = (answer * p) % m;
        }
        p = (p * p) % m;
    }

    answer
}

fn division(a: usize, b: usize, m: usize) -> usize {
    (a * power(b, m - 2, m)) % m
}
