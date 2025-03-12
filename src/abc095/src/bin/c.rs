use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        x: isize,
        y: isize,
    }

    let mut answer = isize::MAX;

    for i in 0..=200000 {
        let num = if x - (i / 2) < 0 && y - (i / 2) < 0 {
            c * i
        } else if x - (i / 2) < 0 && y - (i / 2) >= 0 {
            c * i + (y - (i / 2)) * b
        } else if x - (i / 2) >= 0 && y - (i / 2) < 0 {
            c * i + (x - (i / 2)) * a
        } else {
            c * i + (x - (i / 2)) * a + (y - (i / 2)) * b
        };

        answer = answer.min(num);
    }

    println!("{}", answer)
}
