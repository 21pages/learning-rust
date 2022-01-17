#[test]
fn test_option() {
    let op = Some(2);

    match op {
        Some(i) => println!("{}", i),
        _ => println!("_"),
    }

    if let Some(i) = op {
        println!("{}", i);
    };

    let none: Option<i32> = None;
    match none {
        Some(i) => println!("{}", i),
        _ => println!("_"),
    }
    if let Some(i) = none {
        println!("{}", i);
    } else {
        println!("_");
    }
}
/*
2
2
_
_
*/

#[allow(unused)]
#[derive(Debug)]
enum Foo {
    Bar,
    Baz,
    Qux(i32),
}

#[test]
fn test_enum() {
    let a = Foo::Bar;
    let c = Foo::Qux(2);

    if let Foo::Bar = a {
        println!("{:?}", a);
    }
    if let Foo::Qux(i) = c {
        println!("{}", i);
    }
}

#[derive(PartialEq)]
#[allow(unused)]
enum Foo2 {
    Foo,
    Bar,
}

#[test]
fn test_not_partialeq() {
    let a = Foo2::Bar;
    if a == Foo2::Bar {
        //^^^^^^^^^ must implement `PartialEq<_>`
        println!("==");
    }

    if let Foo2::Bar = a {
        println!("if let");
    }
}
