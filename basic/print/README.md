# 单元测试
* #[test]标注
* cargo test funcname -- --nocapture 输出详细信息
* 用rust-analyzer更方便

# 宏

| 宏                 | 输出       |
| ------------------ | ---------- |
| format!            | 字符串     |
| print!, println!   | io::stdout |
| eprint!, eprintln! | io::stderr |

# trait

| 宏           | 使用       | derive         |
| ------------ | ---------- | -------------- |
| fmt::Display | {}         |                |
| fmt::Debug   | {:?},{:#?} | #derive[Debug] |

所有 `std` 库类型都天生可以使用 `{:?}` 来打印,其余可以加\#[derive(Debug)]打印

Display实现:

```rust
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "my value is {}", self.0)
    }
}
```



# 格式化

| 格式                            | 解释                                       |
| ------------------------------- | ------------------------------------------ |
| {}                              | Display                                    |
| {:?},{:#?}                      | Debug                                      |
| {0}{1}{2}                       | 位置                                       |
| {abc}                           | 名字,abc="hello"                           |
| {:05},{:<5},{1:>5},{:^5},{:>05} | 宽度最小5,左/右/居中对齐,填充0, 默认左对齐 |
| {:+}                            | 带正负号                                   |
| {:X}或{:x},{#b},{#o}            | 0x十六进制,0b二进制,0o八进制               |
| {:.5}                           | 5位小数                                    |
| {:.2$}                          | 第二个参数做精度大小                       |
| {:.*}                           | 占两个位置, 前一个位置是精度, 后一个是值   |
| {:p}                            | Pointer                                    |
| e,E                             | LowerExp,UpperExp                          |



