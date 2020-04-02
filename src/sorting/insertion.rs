pub fn sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() < 2 {
        return;
    }
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn sort_numbers() {
        let mut arr = [2, 0, 5, 6, -1, 99];
        sort(&mut arr);
        assert_eq!(arr, [-1, 0, 2, 5, 6, 99]);
    }

    #[test]
    fn sort_two_numbers() {
        let mut arr = [1, 0];
        sort(&mut arr);
        assert_eq!(arr, [0, 1]);
    }

    #[test]
    fn sort_one_number() {
        let mut arr = [55];
        sort(&mut arr);
        assert_eq!(arr, [55]);
    }

    #[test]
    fn sort_str() {
        let mut arr = ["b", "d", "a", "!", "z"];
        sort(&mut arr);
        assert_eq!(arr, ["!", "a", "b", "d", "z"]);
    }
}