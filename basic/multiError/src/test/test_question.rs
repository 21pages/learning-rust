use std::error;
use std::fmt;

/*
之前我们说 ? 就是 “要么 unwrap 要么 return Err(error)”，这大部分是对的，但 事实上 ? 是 “要么 unwrap 要么 return Err(From::from(err))”。From::from 是 不同类型间的转换工具，也就是说，如果在错误能够转换成返回类型的地方使用 ?，它就 会自动转换成返回类型。
*/

// 为 `Box<error::Error>` 取别名。
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // 泛型错误，没有记录内部原因。
        None
    }
}

// 这里的结构和之前一样，但是这次没有把所有的 `Results` 和 `Options` 串起来，
// 而是使用 `?` 立即得到内部值。
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn test() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
