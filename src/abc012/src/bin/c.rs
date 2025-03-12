use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let answer = 2025 - n;

    if 1 <= answer && answer <= 9 {
        println!("1 x {}", answer)
    }
    if answer % 2 == 0 && answer / 2 >= 1 && answer / 2 <= 9 {
        println!("2 x {}", answer / 2)
    }
    if answer % 3 == 0 && answer / 3 >= 1 && answer / 3 <= 9 {
        println!("3 x {}", answer / 3)
    }
    if answer % 4 == 0 && answer / 4 >= 1 && answer / 4 <= 9 {
        println!("4 x {}", answer / 4)
    }
    if answer % 5 == 0 && answer / 5 >= 1 && answer / 5 <= 9 {
        println!("5 x {}", answer / 5)
    }
    if answer % 6 == 0 && answer / 6 >= 1 && answer / 6 <= 9 {
        println!("6 x {}", answer / 6)
    }
    if answer % 7 == 0 && answer / 7 >= 1 && answer / 7 <= 9 {
        println!("7 x {}", answer / 7)
    }
    if answer % 8 == 0 && answer / 8 >= 1 && answer / 8 <= 9 {
        println!("8 x {}", answer / 8)
    }
    if answer % 9 == 0 && answer / 9 >= 1 && answer / 9 <= 9 {
        println!("9 x {}", answer / 9)
    }
}
