use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = 0;

    for i in 1..=n {
        if !i.to_string().contains('7') {
            if !format!("{:o}", i).contains('7') {
                answer += 1;
            }
        }
    }

    println!("{}", answer)
}
