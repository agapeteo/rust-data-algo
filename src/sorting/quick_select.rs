pub fn top<T: Ord>(top_idx: usize, slice: &mut [T]) -> &T {
    top_in_range(top_idx, slice, 0, slice.len() - 1)
}

fn top_in_range<T: Ord>(top_idx: usize, slice: &mut [T], low_idx: usize, hi_idx: usize) -> &T {
    let mut i = low_idx;

    for j in i..hi_idx {
        if slice[j] <= slice[hi_idx] {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, hi_idx);

    if i != top_idx {
        return if top_idx > i {
            top_in_range(top_idx, slice, i + 1, hi_idx)
        } else {
            top_in_range(top_idx, slice, low_idx, i - 1)
        };
    }
    &slice[i]
}

#[cfg(test)]
mod test {
    use super::top;

    #[test]
    fn random_number() {
        let mut arr = [2, 1, 44, 5, 12];

        assert_eq!(top(0, &mut arr), &1);
        assert_eq!(top(1, &mut arr), &2);
        assert_eq!(top(2, &mut arr), &5);
        assert_eq!(top(3, &mut arr), &12);
        assert_eq!(top(4, &mut arr), &44);
    }

    #[test]
    fn ascending() {
        const LIMIT: usize = 100;
        let mut arr: Vec<usize> = (0..LIMIT).collect();

        for i in 0..LIMIT {
            assert_eq!(top(i, &mut arr), &i);
        }
    }

    #[test]
    fn descending() {
        const LIMIT: usize = 100;
        let mut arr: Vec<usize> = (0..LIMIT).rev().collect();

        for i in 0..LIMIT {
            assert_eq!(top(i, &mut arr), &i);
        }
    }
}