/*
Fn：表示捕获方式为通过引用（&T）的闭包,不可变的引用
FnMut：表示捕获方式为通过可变引用（&mut T）的闭包,可变的引用
FnOnce：表示捕获方式为通过值（T）的闭包,所有权
*/
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_immut_borrow<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

#[test]
fn test() {
    use std::mem;
    let a = "greeting";
    let mut b = "hello".to_owned();
    let f = || {
        println!("a:{}", a); //需要Fn
        b.push_str("world"); //需要FnMut
        mem::drop(b); //需要FnOnce
    }; //函数接受该闭包需要FnOnce

    apply(f);
    let g = |x| x * 2;
    apply_immut_borrow(g);
}
