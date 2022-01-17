#[test]
fn test_match1() {
    let i = 1;
    //match需覆盖所有可能性
    match i {
        1 => println!("1"),
        2 | 3 | 4 | 5 => println!("2~5"),
        6..=10 => println!("6~10"),
        _ => println!(">10"),
    }
}

//解构元组
#[test]
fn test_tuple() {
    let pair = (0, 1);
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }
}
//解构枚举
#[test]
fn test_enum() {
    // 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
    #[allow(dead_code)]
    enum Color {
        // 这三个取值仅由它们的名字（而非类型）来指定。
        Red,
        Blue,
        Green,
        // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}

// 解引用使用 *
// 解构使用 &、ref、和 ref mut
#[test]
fn test_pointer() {
    let reference = &1;
    match reference {
        &val => println!("val is {}", val),
    }
    match *reference {
        val => println!("val is {}", val),
    }
    let _not_a_reference = 2;
    let ref _a_reference = 3;

    let val = 4;
    match val {
        ref r => println!("reference of value {:?}", r),
    }

    let mut mut_val = 5;
    match mut_val {
        ref mut r => {
            *r = 6;
            println!("{:?}", r);
        }
    }
    println!("mut_val:{}", mut_val);
}
