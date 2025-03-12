use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut books = Vec::new();

    for _ in 0..q {
        input! {
            n: usize,
        }

        if n == 1 {
            input! {
                title: String,
            }
            books.push(title);
        } else if n == 2 {
            println!("{}", books[books.len() - 1]);
        } else if n == 3 {
            books.pop();
        }

    }
}
