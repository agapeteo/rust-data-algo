#[test]
fn slices() {
    let mut vec = vec!["a", "b", "c", "d"];

    let slice_all = &vec[..];
    assert_eq!(slice_all, ["a", "b", "c", "d"]);

    let slice_first_two = &vec[..2];
    assert_eq!(slice_first_two, ["a", "b"]);

    let slice_without_first = &vec[1..];
    assert_eq!(slice_without_first, ["b", "c", "d"]);

    let (first_half, second_half) = vec.split_at_mut(2);
    first_half[0] = "_";
    second_half[0] = "?";

    for s in &vec {
        println!("{}", s);
    }
    assert_eq!(vec, ["_", "b", "?", "d"]);
}