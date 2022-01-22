use std::ops;

#[derive(Debug)]
struct Foo(i32);

impl ops::Add for Foo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Foo(self.0 + rhs.0)
    }
}

#[test]
fn test() {
    let f1 = Foo(1);
    let f2 = Foo(2);
    println!("{:?}", f1 + f2);
}
