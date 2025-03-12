use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
    }

    let a_size = a.len();
    let mut x = 0;
    let mut y = 0;

    for i in 0..a_size {
        if a[i] == 'a' {
            x += 1;
        } else {
            break;
        }
    }

    for i in (0..a_size).rev() {
        if a[i] == 'a' {
            y += 1;
        } else {
            break;
        }
    }

    if x == a_size {
        println!("Yes");
        return;
    }

    if x > y {
        println!("No");
        return;
    }

    for i in x..(a_size - y) {
        if a[i] != a[x + a_size - y - i - 1] {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
