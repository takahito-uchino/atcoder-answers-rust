use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let max = a.max(b).max(c);
    let sum = 3 * max - (a + b + c);

    if sum % 2 == 0 {
        println!("{}", sum / 2)
    } else {
        println!("{}", sum / 2 + 2)
    }
}
