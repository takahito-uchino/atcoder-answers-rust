use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let s_vec = rle(s);
    let t_vec = rle(t);

    if s_vec.len() != t_vec.len() {
        println!("No");
        return;
    }

    let mut ans = true;

    for i in 0..s_vec.len() {
        if s_vec[i].0 != t_vec[i].0 {
            ans = false;
            break;
        }
        if !(s_vec[i].1 == t_vec[i].1 || s_vec[i].1 < t_vec[i].1 && s_vec[i].1 >= 2) {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" })
}

fn rle(s: String) -> Vec<(char, usize)> {
    let mut encoded = Vec::new();
    let mut chars = s.chars().peekable();
    let mut count = 1;

    while let Some(current) = chars.next() {
        if let Some(&next) = chars.peek() {
            if current == next {
                count += 1;
            } else {
                encoded.push((current, count));
                count = 1;
            }
        } else {
            encoded.push((current, count));
        }
    }

    encoded
}
