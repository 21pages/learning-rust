fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

#[test]
fn test() {
    let mut x = 7;

    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
    // x = 8;
    // apply(print);
}
