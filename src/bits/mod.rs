fn two_in_binary_format() -> usize {
    0b10
}

fn to_binary_str(n: u8) -> String {
    format!("{:08b}", n)
}

fn check_bit_set(n: usize, idx: usize) -> bool {
    ((1 << idx) & n) != 0
}

fn set_bit(n: usize, idx: usize) -> usize {
    (1 << idx) | n
}

fn clear_bit(n: usize, idx: usize) -> usize {
    let mask = !(1 << idx);
    n & mask
}

fn toggle_bit(n: usize, idx: usize) -> usize {
    (1 << idx) ^ n
}

fn is_even(n: usize) -> bool {
    (n & 1) != 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_in_binary_format() {
        assert_eq!(two_in_binary_format(), 2);
    }

    #[test]
    fn test_to_binary_str() {
        assert_eq!(to_binary_str(2), "00000010")
    }

    #[test]
    fn test_check_bit_set() {
        assert_eq!(check_bit_set(1, 0), true);
        assert_eq!(check_bit_set(1, 1), false);

        assert_eq!(check_bit_set(2, 0), false);
        assert_eq!(check_bit_set(2, 1), true);
        assert_eq!(check_bit_set(2, 2), false);

        assert_eq!(check_bit_set(3, 0), true);
        assert_eq!(check_bit_set(3, 1), true);
        assert_eq!(check_bit_set(3, 2), false);

        assert_eq!(check_bit_set(4, 0), false);
        assert_eq!(check_bit_set(4, 1), false);
        assert_eq!(check_bit_set(4, 2), true);
    }

    #[test]
    fn test_set_bit() {
        assert_eq!(set_bit(1, 0), 1);
        assert_eq!(set_bit(1, 1), 3);

        assert_eq!(set_bit(2, 0), 3);
        assert_eq!(set_bit(2, 1), 2);
        assert_eq!(set_bit(2, 2), 6);

        assert_eq!(set_bit(3, 0), 3);
        assert_eq!(set_bit(3, 1), 3);
        assert_eq!(set_bit(3, 2), 7);
    }

    #[test]
    fn test_toggle_bit() {
        assert_eq!(toggle_bit(1, 0), 0);
        assert_eq!(toggle_bit(1, 1), 3);

        assert_eq!(toggle_bit(2, 0), 3);
        assert_eq!(toggle_bit(2, 1), 0);
        assert_eq!(toggle_bit(2, 2), 6);

        assert_eq!(toggle_bit(3, 0), 2);
        assert_eq!(toggle_bit(3, 1), 1);
        assert_eq!(toggle_bit(3, 2), 7);
    }


    #[test]
    fn test_is_even() {
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(1), false);
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(11), false);
    }

    #[test]
    fn test_clear_bit() {
        assert_eq!(clear_bit(1, 0), 0);
        assert_eq!(clear_bit(2, 1), 0);
        assert_eq!(clear_bit(7, 1), 5);
    }
}