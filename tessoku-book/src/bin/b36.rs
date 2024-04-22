use proconio::input;

fn main() {
    input! {
        _: usize,
        k: usize,
        s: String,
    }

    let mut on_count = 0;

    for c in s.chars() {
        if c == '1' {
            on_count +=1;
        }
    }

    if on_count % 2 == k % 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
