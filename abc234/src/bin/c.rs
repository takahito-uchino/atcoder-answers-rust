use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let binary_string = format!("{:b}", k);

    let mut answer = String::new();

    for c in binary_string.chars() {
        if c == '1' {
            answer.push('2');
        } else {
            answer.push('0');
        }
    }

    println!("{}", answer)
}
