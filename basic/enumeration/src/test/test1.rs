enum WebEvent {
    PageLoad, //单元结构体
    PageUnload,
    KeyPress(char), //元组结构体
    Paste(String),
    Click { x: i64, y: i64 }, //普通结构体
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("key pressed:{}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

#[test]
fn test1() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("hello".to_owned()); //"hello"是slice
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
