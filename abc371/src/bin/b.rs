use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        babies: [(usize, String); m],
    }

    let mut houses = vec![0; n];

    for (a, b) in babies {
        if b == "F" {
            println!("No");
        } else {
            if houses[a - 1] == 0 {
                println!("Yes");
                houses[a - 1] += 1;
            } else {
                println!("No");
            }
        }
    }
}
