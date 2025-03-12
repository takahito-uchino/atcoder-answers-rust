use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }

    if (s1 == "<" && s2 == "<" && s3 == "<") || (s1 == ">" && s2 == ">" && s3 == ">") {
        println!("B")
    } else if (s1 == "<" && s2 == "<" && s3 == ">") || (s1 == ">" && s2 == ">" && s3 == "<") {
        println!("C")
    } else {
        println!("A")
    }
}
