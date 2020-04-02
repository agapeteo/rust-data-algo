struct Range {
    low_idx: usize,
    high_idx: usize,
}

impl Range {
    pub fn new(low_idx: usize, high_idx: usize) -> Self {
        Range { low_idx, high_idx }
    }
    pub fn len(&self) -> usize {
        self.high_idx - self.low_idx
    }
}

pub fn sort<T: Ord + Clone>(slice: &mut [T]) {
    if slice.len() < 2 {
        return
    }
    let mut helper_vec = Vec::new();
    helper_vec.extend_from_slice(slice);

    sort_range(slice, &Range::new(0, slice.len() - 1), &mut helper_vec);
}

fn sort_range<T: Ord + Clone>(slice: &mut [T], range: &Range, helper_slice: &mut [T]) {
    if range.low_idx >= range.high_idx {
        return;
    }

    let mid_idx = range.low_idx + (range.high_idx - range.low_idx) / 2;

    let left_range = &Range::new(range.low_idx, mid_idx);
    sort_range(slice, left_range, helper_slice);

    let right_range = &Range::new(mid_idx + 1, range.high_idx);
    sort_range(slice, right_range, helper_slice);

    merge(slice, left_range, right_range, helper_slice);
}

fn merge<T: Ord + Clone>(slice: &mut [T], left_range: &Range, right_range: &Range, helper_slice: &mut [T]) {
    helper_slice[left_range.low_idx..=right_range.high_idx]
        .clone_from_slice(&slice[left_range.low_idx..=right_range.high_idx]);

    let mut cur_idx = left_range.low_idx;
    let mut cur_left_idx = left_range.low_idx;
    let mut cur_right_idx = right_range.low_idx;

    while cur_left_idx <= left_range.high_idx && cur_right_idx <= right_range.high_idx {
        if helper_slice[cur_left_idx] < helper_slice[cur_right_idx] {
            slice[cur_idx] = helper_slice[cur_left_idx].clone();
            cur_left_idx += 1;
        } else {
            slice[cur_idx] = helper_slice[cur_right_idx].clone();
            cur_right_idx += 1;
        }
        cur_idx += 1;
    }

    if cur_right_idx <= right_range.high_idx {
        while cur_right_idx <= right_range.high_idx {
            slice[cur_idx] = helper_slice[cur_right_idx].clone();
            cur_right_idx += 1;
            cur_idx += 1;
        }
    } else {
        while cur_left_idx <= left_range.high_idx {
            slice[cur_idx] = helper_slice[cur_left_idx].clone();
            cur_left_idx += 1;
            cur_idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use rand::prelude::*;

    fn sorted<T: Ord>(slice: &[T]) -> bool {
        for i in 0..(slice.len() -1) {
            if slice[i] > slice[i+1] {
                return false;
            }
        }
        true
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        sort(&mut arr);
        assert!(sorted(&arr));
    }

    #[test]
    fn already_sorted_reversed() {
        let mut arr = [5, 4, 3, 2, 1];
        sort(&mut arr);
        assert!(sorted(&arr));
    }

    #[test]
    fn with_dups() {
        let mut arr = [1, 4, 1, 3, 19, 2, 1, 19, 40];
        sort(&mut arr);
        assert!(sorted(&arr));
    }

    #[test]
    fn shuffled_0_100() {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();

        let mut arr: Vec<i32> = (1..100).collect();
        arr.shuffle(&mut rng);

        sort(&mut arr);

        assert!(sorted(&arr))
    }

    #[test]
    fn random_arr() {
        const TIMES: usize = 100;
        for _ in 0..TIMES {
            let mut arr: Vec<i32> = vec![];
            let mut rng = thread_rng();
            let rnd_vec_size: usize = rng.gen_range(1, 1_000);
            for _ in 0..rnd_vec_size {
                let rnd_num: i32 = rng.gen();
                arr.push(rnd_num);
            }
            sort(&mut arr);
            assert!(sorted(&arr));
        }
    }
}