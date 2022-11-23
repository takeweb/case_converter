/// Convert snake case to camel case
/// # Example
/// ```
/// use case_converter::snake_to_camel;
///
/// let result = snake_to_camel("snake_to_camel");
/// println!("{}", result); // → snakeToCamel
/// ```
pub fn snake_to_camel(arg_str: &str) -> String {
    // 「_(アンダーバー)」で文字列部を分割
    let list: Vec<&str> = arg_str
                            .split('_')
                            .map(str::trim)
                            .collect();
    // 分割した文字列を処理
    list.into_iter()
        .enumerate()
        .map(|(i, val)| make_camel(i, val))
        .collect()
}

// 文字列単位の処理(snake_to_camel用)
fn make_camel(i: usize, s: &str) -> String {
    if i == 0 {
        String::from(s)
    } else {
        let c: Vec<char> = s.chars().collect();
        let tmp: Vec<char> = c.iter()
                                .enumerate()
                                .map(|(i, val)| to_uppercase(i, val))
                                .collect();
        String::from_iter(tmp)
    }
}

// 文字単位の処理(snake_to_camel用)
fn to_uppercase(i: usize, c: &char) -> char {
    if i == 0 {
        c.to_ascii_uppercase()
    } else {
        *c
    }
}

/// Convert camel case to snake case
/// # Example
/// ```
/// use case_converter::camel_to_snake;
///
/// let result = camel_to_snake("camelToSnake");
///  println!("{}", result); // → camel_to_snake
/// ```
// キャメルケース文字列をスネークケース文字列へ変換
pub fn camel_to_snake(arg_str: &str) -> String {
    let chars: Vec<char> = arg_str.chars().collect();
    chars.into_iter()
        .map(|c| to_lowercase(c))
        .collect()
}

// 文字単位の処理(camel_to_snake用)
fn to_lowercase(c: char) -> String {
    if c.is_uppercase() {
        let mut chars: Vec<char> = Vec::new();
        chars.push('_');
        chars.push(c.to_ascii_lowercase());
        String::from_iter(chars)
    } else {
        c.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_to_camel() {
        assert_eq!(snake_to_camel("snake"), "snake");
        assert_eq!(snake_to_camel("snake_to"), "snakeTo");
        assert_eq!(snake_to_camel("snake_to_camel"), "snakeToCamel");
    }

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(camel_to_snake("camel"), "camel");
        assert_eq!(camel_to_snake("camelTo"), "camel_to");
        assert_eq!(camel_to_snake("camelToSnake"), "camel_to_snake");
    }
}
