pub(crate) fn parsing(val: &str) -> &str {
    let parsing_val = val.parse::<i8>();
    match parsing_val {
        Ok(parsing_val) => return "i8",
        Err(error) => {},
    }
    let parsing_val2 = val.parse::<i32>();
    match parsing_val2 {
        Ok(parsing_val2) => return "i32",
        Err(error) => {},
    }
    let parsing_val3 = val.parse::<i64>();
    match parsing_val3 {
        Ok(parsing_val3) => return "i64",
        Err(error) => {},
    }
    let parsing_val4 = val.parse::<f64>();
    match parsing_val4 {
        Ok(parsing_val4) => return "f64",
        Err(error) => {},
    }
    let len = val.len();
    if val.chars().nth(0) != None {
        if val.chars().nth(0).unwrap() == '[' &&
            val.chars().nth(len - 1).unwrap() == ']' {
            return "list"
        } else { return "String" }
    } return "String"
    // return "String" -> int값이 아니면 error 메시지 출력으로 변경 할 것
}