use proconio::input;

fn main() {
    input! {
        h: i32,
    }

    let mut height = 0;
    let mut date = 0;

    loop {
        date += 1;
        height += 2i32.pow(date as u32);
        if height >= h {
            println!("{}", date + 1);
            return;
        }
    }
}
