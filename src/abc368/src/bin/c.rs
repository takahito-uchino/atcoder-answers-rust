use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut t = 0;

    for mut hi in h {
        let num = hi / 5;
        t += num * 3;
        hi -= num * 5;
        while hi > 0 {
            t += 1;
            if t % 3 == 0 {
                hi -= 3;
            } else {
                hi -= 1;
            }
        }
    }

    println!("{}", t)
}
