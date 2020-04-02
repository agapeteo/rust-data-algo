fn power_set<'a, T: Clone>(items: &'a [&T]) -> Vec<Vec<&'a T>> {
    let mut power_set: Vec<Vec<&'a T>> = Vec::new();

    for item in items {
        for i in 0..power_set.len() {
            let cur_set = &power_set[i];
            let mut new_set = cur_set.clone();
            new_set.push(item);
            power_set.push(new_set);
        }
        power_set.push(vec![item]);
    }
    power_set.push(Vec::new());
    power_set
}

#[test]
fn test_power_set() {
    let mut actual = power_set(&[&1, &2, &3]);
    actual.sort();

    let expected: Vec<Vec<&i32>> = vec![
        vec![],
        vec![&1],
        vec![&1, &2],
        vec![&1, &2, &3],
        vec![&1, &3],
        vec![&2],
        vec![&2, &3],
        vec![&3]];

    assert_eq!(actual, expected);
}