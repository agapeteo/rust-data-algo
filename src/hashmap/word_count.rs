use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<&str, i32> {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

#[test]
fn test() {
    let text = "let my people go go again my people go";
    let result = word_count(text);

    let mut expected = HashMap::new();
    expected.insert("go", 3);
    expected.insert("my", 2);
    expected.insert("people", 2);
    expected.insert("let", 1);
    expected.insert("again", 1);

    assert_eq!(result, expected);
}