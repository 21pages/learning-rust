#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

#[test]
fn test() {
    if cfg!(some_condition) {
        conditional_function();
    }
}
