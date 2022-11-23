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

/// Convert kebab case to camel case
/// # Example
/// ```
/// use case_converter::kebab_to_camel;
///
/// let result = kebab_to_camel("kebab-to-camel");
/// println!("{}", result); // → kebabToCamel
/// ```
pub fn kebab_to_camel(arg_str: &str) -> String {
    // 「-(ハイフン)」で文字列部を分割
    let list: Vec<&str> = arg_str
                            .split('-')
                            .map(str::trim)
                            .collect();
    // 分割した文字列を処理
    list.into_iter()
        .enumerate()
        .map(|(i, val)| make_camel(i, val))
        .collect()
}

// 文字単位のキャメルケース変換処理
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

// 文字単位の小文字変換処理
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
pub fn camel_to_snake(arg_str: &str) -> String {
    let chars: Vec<char> = arg_str.chars().collect();
    chars.into_iter()
        .map(|c| to_lowercase(c, '_'))
        .collect()
}

/// Convert camel case to kebab case
/// # Example
/// ```
/// use case_converter::camel_to_kebab;
///
/// let result = camel_to_kebab("camelToKebab");
///  println!("{}", result); // → camel-to-kebab
/// ```
pub fn camel_to_kebab(arg_str: &str) -> String {
    let chars: Vec<char> = arg_str.chars().collect();
    chars.into_iter()
        .map(|c| to_lowercase(c, '-'))
        .collect()
}

// 文字単位の小文字変換処理
fn to_lowercase(c: char, s: char) -> String {
    if c.is_uppercase() {
        let mut chars: Vec<char> = Vec::new();
        chars.push(s);
        chars.push(c.to_ascii_lowercase());
        String::from_iter(chars)
    } else {
        c.to_string()
    }
}

/// Convert kebab case to snake case
/// # Example
/// ```
/// use case_converter::kebab_to_snake;
///
/// let result = kebab_to_snake("kebab-to-snake");
///  println!("{}", result); // → kebab_to_snake
/// ```
pub fn kebab_to_snake(arg_str: &str) -> String {
    arg_str.replace("-", "_")
}

/// Convert snake case to kebab case
/// # Example
/// ```
/// use case_converter::snake_to_kebab;
///
/// let result = snake_to_kebab("snake_to_kebab");
///  println!("{}", result); // → snake-to-kebab
/// ```
pub fn snake_to_kebab(arg_str: &str) -> String {
    arg_str.replace("_", "-")
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
    fn test_kebab_to_camel() {
        assert_eq!(kebab_to_camel("kebab"), "kebab");
        assert_eq!(kebab_to_camel("kebab-to"), "kebabTo");
        assert_eq!(kebab_to_camel("kebab-to-camel"), "kebabToCamel");
    }

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(camel_to_snake("camel"), "camel");
        assert_eq!(camel_to_snake("camelTo"), "camel_to");
        assert_eq!(camel_to_snake("camelToSnake"), "camel_to_snake");
    }

    #[test]
    fn test_camel_to_kebab() {
        assert_eq!(camel_to_kebab("camel"), "camel");
        assert_eq!(camel_to_kebab("camelTo"), "camel-to");
        assert_eq!(camel_to_kebab("camelToKebab"), "camel-to-kebab");
    }

    #[test]
    fn test_kebab_to_snake() {
        assert_eq!(kebab_to_snake("kebab"), "kebab");
        assert_eq!(kebab_to_snake("kebab-to"), "kebab_to");
        assert_eq!(kebab_to_snake("kebab-to-snake"), "kebab_to_snake");
    }

    #[test]
    fn test_snake_to_kebab() {
        assert_eq!(snake_to_kebab("kebab"), "kebab");
        assert_eq!(snake_to_kebab("kebab_to"), "kebab-to");
        assert_eq!(snake_to_kebab("snake_to_kebab"), "snake-to-kebab");
    }
}
