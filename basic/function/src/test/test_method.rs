#[derive(Debug)]
#[allow(unused)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    //关连函数
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    //方法
    fn show(&self) {
        println!("{:?}", &self);
    }
}

#[test]
fn test_point() {
    let p = Point::new(1.0, 2.0);
    p.show();
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;

        self.p2.x += x;
        self.p2.y += y;
    }
}

#[test]
fn test_rectangle() {
    let mut r = Rectangle {
        p1: Point::new(0.0, 0.0),
        p2: Point::new(2.0, 2.0),
    };
    println!("area:{}", r.area());
    r.translate(1.0, 1.0);
    println!("{:#?}", r);
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    //self所有权转移了
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroy pair Pair({}, {})", first, second);
    }
}

#[test]
fn test_pair() {
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}
