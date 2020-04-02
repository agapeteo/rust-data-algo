pub fn sort(slice: &mut [usize]) {
    if slice.len() < 2 {
        return;
    }
    let capacity = slice.iter().max().unwrap() + 1;
    let mut tmp_vec: Vec<usize> = vec![0; capacity];

    for cur_num in slice.iter() {
        match tmp_vec.get(*cur_num) {
            None => { tmp_vec[*cur_num] = 1; }
            Some(_) => {
                tmp_vec[*cur_num] += 1;
            }
        }
    }

    let mut cur_idx = 0;
    for i in 0..tmp_vec.len() {
        let cur_count = tmp_vec[i];
        for _ in 0..cur_count {
            slice[cur_idx] = i;
            cur_idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn random_numbers_1() {
        let mut arr = [0, 2, 2, 1];

        sort(&mut arr);

        assert_eq!(arr, [0, 1, 2, 2]);
    }

    #[test]
    fn random_numbers_2() {
        let mut arr = [1, 999, 1, 7, 999, 2, 2, 1, 5];

        sort(&mut arr);

        assert_eq!(arr, [1, 1, 1, 2, 2, 5, 7, 999, 999]);
    }

    #[test]
    fn ascending() {
        let mut arr: Vec<usize> = (0..100).collect();
        let expected = arr.clone();

        sort(&mut arr);

        assert_eq!(arr, expected);
    }

    #[test]
    fn descending() {
        let mut arr: Vec<usize> = (0..100).rev().collect();
        let expected: Vec<usize> = (0..100).collect();

        sort(&mut arr);

        assert_eq!(arr, expected);
    }
}
