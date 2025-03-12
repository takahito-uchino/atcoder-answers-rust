use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut tmp_a = a;
    let mut tmp_b = b;
    while tmp_a >= 1 && tmp_b >= 1 {
        if tmp_a >= tmp_b {
            tmp_a %= tmp_b;
        } else {
            tmp_b %= tmp_a;
        }
    }

    println!("{}", if tmp_a != 0 {a * b / tmp_a} else { a * b / tmp_b });
}
