use proconio::input;

fn main() {
    input! {
        _: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
    }

    for i in p..=q {
        for j in r..=s {
            if i + b == j + a {
                print!("#");
            } else if i + j == a + b {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}
