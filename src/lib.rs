pub fn calc(x: i32, y: i32, op: &str) -> i32 {
    match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => panic!("Unknown operator")
    }
}

pub fn factorial(x: i64) -> i64 {
    if x <= 1 { 1 } else { x * factorial(x - 1) }
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn it_can_add() {
        assert_eq!(calc(5, 6, "+"), 11);
    }

    #[test]
    fn it_can_subtract() {
        assert_eq!(calc(10, 6, "-"), 4);
    }

    #[test]
    fn it_can_multiply() {
        assert_eq!(calc(3, 4, "*"), 12);
    }

    #[test]
    fn it_can_divide() {
        assert_eq!(calc(20, 4, "/"), 5);
    }

    #[test]
    #[should_panic]
    fn it_panics_with_unknown_operator() {
        calc(4, 5, "%");
    }
}
