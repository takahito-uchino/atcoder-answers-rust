use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", num::integer::lcm(a, b))
}
