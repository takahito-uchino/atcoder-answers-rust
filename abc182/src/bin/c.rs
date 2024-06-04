use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut count = vec![0; 3];

    for c in n {
        let num = c.to_digit(10).unwrap();
        count[(num % 3) as usize] += 1;
    }

    let current = (count[1] + 2 * count[2]) % 3;
    let k = count[0] + count[1] + count[2];
    let mut result = 0;

    if current == 1 {
        if count[1] != 0 {
            if k == 1 {
                result = -1;
            } else {
                result = 1;
            }
        } else {
            if k == 2 {
                result = -1;
            } else {
                result = 2;
            }
        }
    } else if current == 2 {
        if count[2] != 0 {
            if k == 1 {
                result = -1;
            } else {
                result = 1;
            }
        } else {
            if k == 2 {
                result = -1;
            } else {
                result = 2;
            }
        }
    }

    println!("{}", result)
}
