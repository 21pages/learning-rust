use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

//应该实现Disply, 会自动实现toString
impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

//用于parse函数
impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //1.map_or_else
        // s.parse::<i32>().map_or_else(
        //      |_|{
        //         Err(())
        //     },
        //     |v| Ok(Circle{radius:v})
        // )

        //2.map
        match s.parse::<i32>().map(|i| i) {
            Ok(v) => Ok(Circle { radius: v }),
            Err(_) => Err(()),
        }
    }
}

#[test]
fn test_to_string() {
    let a = Circle { radius: 2 };
    println!("{}", a.to_string()); //Circle of radius 2

    let b = "3".parse::<Circle>();
    println!("{:?}", b);
}
