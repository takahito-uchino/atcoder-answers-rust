use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    if n == 1 || n == 2 {
        println!("Yes");
        return;
    }

    for i in 0..n - 2 {
        if s[i] == "sweet" && s[i + 1] == "sweet" {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
