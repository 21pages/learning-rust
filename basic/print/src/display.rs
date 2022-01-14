use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "my value is {}", self.0)
    }
}

#[test]
fn test_display() {
    let s: Structure = Structure(2);
    println!("{}", s); //my value is 2
    println!("{:?}", s); //Structure(2)
    println!("{:#?}", &s); //Structure(
                           //     2,
                           // )
}
