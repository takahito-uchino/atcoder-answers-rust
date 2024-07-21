use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut answer = 0;

    for si in s {
        if si == "Takahashi" {
            answer += 1;
        }
    }

    println!("{}", answer)
}
