const THRESHOLD :i32 = 10; //不可变
static LANGUAGE :&'static str = "hello"; //一直存在, 加mut可变, 字符串字面量是static不可变
fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

#[test]
fn test() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}
