pub fn permutations(str: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(factorial(str.len()));

    permut(&[], &to_chars(&str), &mut result);
    result
}

fn permut(prefix: &[char], suffix: &[char], result: &mut Vec<String>) {
    if suffix.is_empty() {
        result.push(to_string(prefix));
        return;
    }
    for i in 0..suffix.len() {
        let mut new_prefix = prefix.to_vec();
        new_prefix.push(suffix[i]);

        let (left, right) = suffix.split_at(i);
        let mut new_suffix = left.to_vec();

        let mut right_part: Vec<char> = Vec::new();
        right_part.extend(&right[1..]); // without current char
        new_suffix.extend(right_part);

        permut(&new_prefix, &new_suffix, result);
    }
}

fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}

fn to_chars(str: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::with_capacity(str.len());
    for char in str.chars() {
        result.push(char);
    }
    result
}

fn to_string(chars: &[char]) -> String {
    let mut result = String::with_capacity(chars.len());
    for char in chars {
        result.push(*char);
    }
    result
}

#[test]
fn test() {
    let mut actual = permutations("ABC");
    actual.sort();

    assert_eq!(actual, ["ABC", "ACB", "BAC", "BCA", "CAB", "CBA"]);

    assert_eq!(permutations("1234").len(), factorial(4));
    assert_eq!(permutations("12345").len(), factorial(5));
}