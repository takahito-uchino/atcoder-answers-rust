use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let ac_is_minus = if c % 2 == 1 && a < 0 { true } else { false };
    let bc_is_minus = if c % 2 == 1 && b < 0 { true } else { false };

    if ac_is_minus != bc_is_minus {
        if ac_is_minus {
            println!("<")
        } else {
            println!(">")
        }
    } else {
        if a.abs() == b.abs() {
            println!("=")
        } else if (a.abs() < b.abs()) ^ ac_is_minus {
            println!("<")
        } else {
            println!(">")
        }
    }
}
