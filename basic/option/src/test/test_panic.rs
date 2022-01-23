fn give_princess(gift: &str) {
    // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止！
    if gift == "snake" {
        panic!("snake!!!")
    }

    println!("I love {}s!!!!!", gift);
}

#[test]
fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
