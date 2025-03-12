use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        t: [usize; n],
    }

    let mut time = 0;

    for ti in t {
        if ti <= time {
            time += a;
            println!("{}", time);
        } else {
            time = ti + a;
            println!("{}", time);
        }
    }
}
