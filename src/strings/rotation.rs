pub enum Direction {
    Left,
    Right,
}

pub fn rotate(str: &str, direction: Direction, count: usize) -> String {
    let mut str_vec: Vec<char> = str.chars().collect();
    match direction {
        Direction::Left => { str_vec.rotate_left(count) }
        Direction::Right => { str_vec.rotate_right(count) }
    }
    str_vec.iter().collect()
}

#[test]
fn test() {
    let str = "ABCDE";

    assert_eq!(rotate(str, Direction::Left, 1), "BCDEA");
    assert_eq!(rotate(str, Direction::Left, 2), "CDEAB");
    assert_eq!(rotate(str, Direction::Left, 5), "ABCDE");

    assert_eq!(rotate(str, Direction::Right, 1), "EABCD");
    assert_eq!(rotate(str, Direction::Right, 2), "DEABC");
    assert_eq!(rotate(str, Direction::Right, 5), "ABCDE");
}