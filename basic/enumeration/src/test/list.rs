use List::*;

enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        return Nil;
    }
    fn prepend(self, ele: u32) -> List {
        Cons(ele, Box::new(self))
    }
    fn len(&self) -> u32 {
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

#[test]
fn test() {
    let mut list = List::new();
    for i in 1..3 {
        list = list.prepend(i)
    }
    println!("len:{}", list.len()); //len:2
    println!("{}", list.stringify()); //2, 1, Nil
}
