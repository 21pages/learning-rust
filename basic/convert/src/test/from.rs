#[test]
fn test_string() {
    let a = String::from("hello");
    println!("{}", a); //hello
}

#[derive(Debug)]
#[allow(unused)]
struct Number {
    v: u32,
}

impl From<u32> for Number {
    fn from(v: u32) -> Self {
        Number { v: v }
    }
}

#[test]
fn test_number() {
    //from
    let n = Number::from(2);
    println!("{:?}", n); //Number { v: 2 }

    //实现from也就实现了into
    let num: Number = 5.into();
    println!("{:?}", num); //Number { v: 5 }
}
