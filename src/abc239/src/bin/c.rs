use proconio::input;

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }

    for x in x1 - 2..x1 + 3 {
        for y in y1 - 2..y1 + 3 {
            if dist(x, y, x1, y1) == 5 && dist(x, y, x2, y2) == 5 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}

fn dist(a: isize, b: isize, c: isize, d: isize) -> isize {
    (a - c).pow(2) + (b - d).pow(2)
}
