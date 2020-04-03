fn max_sum(slice: &[isize]) -> isize {
    let mut result = 0;
    let mut cur_sum = 0;

    for i in 0..slice.len() {
        if cur_sum + slice[i] > 0 {
            cur_sum += slice[i];
            if cur_sum > result {
                result = cur_sum;
            }
        } else {
            cur_sum = 0;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = [-1, 2, 3, -7, 3, 4];
        assert_eq!(max_sum(&arr), 7);
    }

    #[test]
    fn test_2() {
        let arr = [34, -50, 42, 14, -5, 86];
        assert_eq!(max_sum(&arr), 137);
    }

    #[test]
    fn all_negative() {
        let arr = [-3, -2, -1, -4];
        assert_eq!(max_sum(&arr), 0);
    }

    #[test]
    fn all_positive() {
        let arr = [1, 1, 0, 1, 1];
        assert_eq!(max_sum(&arr), 4);
    }
}

