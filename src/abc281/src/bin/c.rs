use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: usize,
        a: [usize; n],
    }

    let sum = a.iter().sum::<usize>();
    t %= sum;

    let mut num = 1;

    for i in a {
        if i < t {
            t -= i;
            num += 1;
        } else {
            println!("{} {}", num, t);
            return;
        }
    }
}
