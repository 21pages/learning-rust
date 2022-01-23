use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|s| s.parse::<i32>().map(|v| v * 2))
}

fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|s| s.parse::<i32>().map(|v| v * 2));
    opt.map_or(Ok(None), |i| i.map(Some))
}

#[test]
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first2(numbers));

    println!("The first doubled is {:?}", double_first2(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first2(strings));
    // Error 2: the element doesn't parse to a number
}
