use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v);
        }
        write!(f, "]")
    }
}

#[test]
fn test_list() {
    let list = List(vec![1, 2, 3]);
    println!("{}", list) //[0:1, 1:2, 2:3]
}
