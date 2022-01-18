/*
声明时使用 || 替代 () 将输入参数括起来。
函数体定界符（{}）对于单个表达式是可选的，其他情况必须加上。
有能力捕获外部环境的变量。
*/

#[test]
fn test1() {
    fn function(i: i32) -> i32 {
        i + 1
    }
    let closure1 = |i: i32| i + 1;
    let closure2 = |i: i32| i + 1;

    let i = 1;
    println!("function:{}", function(i));
    println!("closure1:{}", closure1(i));
    println!("closure2:{}", closure2(i));

    let one = || 1;
    println!("one:{}", one())
}

/*
通过引用：&T
通过可变引用：&mut T
通过值：T
*/

#[test]
fn test_closure2() {
    use std::mem;
    let color = "green";

    //&T
    let print = || println!("`color`:{}", color);
    println!("只定义, 未调用");
    print();
    print();

    //&mut T
    let mut num = 1;
    let mut inc = || {
        num += 1;
        println!("num:{}", num)
    };
    inc();
    inc();

    //T
    //不可复制类型 no-copy type
    let movable = Box::<i32>::new(1);
    let consume = || {
        println!("movalbe:{}", movable);
        //mem::drop(movable);
    };
    consume();
    consume();
    println!("movable:{}", movable);

    // |前加move强制获取所有权
    let v = vec![1, 2, 3];
    let owner = move |x| v.contains(x);
    println!("contains:{}", owner(&1));
    println!("contains:{}", owner(&4));
    //println!("v:{:?}", v)
}
