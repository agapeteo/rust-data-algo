fn validate_single_parentheses(open: char, close: char, str: &str) -> bool {
    let mut count: usize = 0;

    for char in str.chars() {
        if char == open {
            count += 1
        }
        if char == close {
            if count == 0 {
                return false;
            }
            count -= 1;
        }
    }
    count == 0
}

#[test]
fn test() {
    assert_eq!(validate_single_parentheses('(', ')', "to ((be)) or (not) to be"), true);
    assert_eq!(validate_single_parentheses('(', ')', "to ((be)(or) not"), false);
    assert_eq!(validate_single_parentheses('(', ')', ")("), false);
    assert_eq!(validate_single_parentheses('(', ')', "to (be)) or ("), false);
    assert_eq!(validate_single_parentheses('(', ')', ""), true);
    assert_eq!(validate_single_parentheses('(', ')', "()"), true);
}