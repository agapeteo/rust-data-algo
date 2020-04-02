fn dedup<T: Ord>(slice: &mut [T]) -> usize {
    slice.sort();

    let mut i = 0;

    for j in i + 1..slice.len() {
        if slice[i] != slice[j] {
            i += 1;
            slice.swap(i, j);
        }
    }
    i + 1
}

#[cfg(test)]
mod tests {
    use super::dedup;

    #[test]
    fn all_unique() {
        let mut arr = [5, 4, 2, 1, 0];

        let last_idx = dedup(&mut arr);

        assert_eq!(last_idx, arr.len());
    }

    #[test]
    fn all_same() {
        let mut arr = ["a", "a", "a", "a"];

        let last_idx = dedup(&mut arr);

        assert_eq!(last_idx, 1);
    }

    #[test]
    fn some_dups_2() {
        let mut arr = ["a", "b", "a", "a"];

        let last_idx = dedup(&mut arr);

        assert_eq!(last_idx, 2);
    }

    #[test]
    fn some_dups_5() {
        let mut arr = ["a", "b", "a", "b", "c", "d", "d", "e"];

        let last_idx = dedup(&mut arr);

        assert_eq!(last_idx, 5);
    }
}