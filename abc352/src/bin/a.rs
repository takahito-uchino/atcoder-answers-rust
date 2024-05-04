use proconio::input;

fn main() {
    input! {
        _: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    println!(
        "{}",
        if (x <= z && z <= y) || (y <= z && z <= x) {
            "Yes"
        } else {
            "No"
        }
    )
}
