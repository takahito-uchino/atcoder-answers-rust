use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut answer = 0;

    for mut i in a {
        while i % 2 == 0 {
            i /= 2;
            answer += 1;
        }
    }

    println!("{}", answer)
}
