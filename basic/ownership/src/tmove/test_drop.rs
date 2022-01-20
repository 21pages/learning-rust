struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
        std::mem::drop(self);
    }
}

#[test]
fn test() {
    let _x = ToDrop;
    println!("Made a ToDrop!");
}
