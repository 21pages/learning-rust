// pub trait Iterator {
//     // 被迭代的类型。
//     type Item;

//     // `find` 接受 `&mut self` 参数，表明函数的调用者可以被借用和修改，
//     // 但不会被消耗。
//     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
//         // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
//         // `&Self::Item` 指明了被捕获变量的类型（译注：是对迭代器元素的引用类型）
//         P: FnMut(&Self::Item) -> bool {}
// }

#[test]
fn test() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("find 2 in vec1:{:?}", iter.find(|&&x| x == 2));
    println!("find 2 in vec2:{:?}", into_iter.find(|&x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];
    println!("find 2 in arr1:{:?}", arr1.iter().find(|&&x| x == 2));
    println!("find 2 in arr1:{:?}", arr2.into_iter().find(|&x| x == 2));
}
