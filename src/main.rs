use std::io;

fn main() {
    println!("入力してね");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("失敗したから爆発するね");
    print!("あなたの入力: {}", x);
}