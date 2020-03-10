use std::collections::{VecDeque, HashSet, HashMap};

fn validate_multiple_parentheses(open_close_tuples: &[(char, char)], str: &str) -> bool {
    let mut stack: VecDeque<char> = VecDeque::new();
    let (open_set, close_map) = build_open_close_structs(open_close_tuples);

    for char in str.chars() {
        if open_set.contains(&char) {
            stack.push_back(char);
        }
        if close_map.contains_key(&char) {
            let popped_open_char_opt = stack.pop_back();
            match popped_open_char_opt {
                None => {
                    return false;
                }
                Some(popped_open_char) => {
                    let found_open_char = close_map.get(&char);
                    if popped_open_char != **found_open_char.unwrap() {
                        return false;
                    }
                }
            }
        }
    }
    stack.is_empty()
}

fn build_open_close_structs(open_close_tuples: &[(char, char)]) -> (HashSet<&char>, HashMap<&char, &char>) {
    let mut open_set: HashSet<&char> = HashSet::new();
    let mut close_map: HashMap<&char, &char> = HashMap::new();

    for tup in open_close_tuples.iter() {
        open_set.insert(&tup.0);
        close_map.insert(&tup.1, &tup.0); // key - close char, value - open char
    }
    (open_set, close_map)
}


#[test]
fn test() {
    let tuples = [('(', ')'), ('[', ']'), ('{', '}')];
    assert_eq!(validate_multiple_parentheses(&tuples, "(to) be {[(or) (not)]} to be()"), true);
    assert_eq!(validate_multiple_parentheses(&tuples, "to ((be)) or (not) to be"), true);
    assert_eq!(validate_multiple_parentheses(&tuples, "to ([be]) or {not} to be"), true);
    assert_eq!(validate_multiple_parentheses(&tuples, "to ( [be] {or} not }"), false);
    assert_eq!(validate_multiple_parentheses(&tuples, ")("), false);
    assert_eq!(validate_multiple_parentheses(&tuples, "to (be) }{ or not"), false);
}