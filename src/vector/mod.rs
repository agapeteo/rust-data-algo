#[test]
fn vector() {
    let empty_vector: Vec<String> = vec![]; // create empty vector using vec! macro
    let non_empty_macro_vector = vec!["a", "b", "c"]; // create vector with predefined values, type is inferred

    let mut some_vec: Vec<i8> = Vec::new(); // note, empty vector doesn't allocate any memory in heap, only fat pointer in stack allocated
    some_vec.push(0);
    some_vec.push(1);
    some_vec.push(1);

    assert_eq!(some_vec.len(), 3);
    some_vec[2] = 2;
    some_vec.push(3);
    assert_eq!(some_vec, [0, 1, 2, 3]);
    assert_eq!(some_vec.len(), 4);

    while let Some(latest) = some_vec.pop() {
        println!("{}", latest)
    }
    assert_eq!(some_vec.is_empty(), true);
}