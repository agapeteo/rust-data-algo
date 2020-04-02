use rand::prelude::*;

pub fn sort<T: Ord>(slice: &mut [T]) {
    if slice.len() < 2 {
        return;
    }
    slice.shuffle(&mut rand::thread_rng());

    sort_range(slice, 0, slice.len() - 1);
}

fn sort_range<T: Ord>(slice: &mut [T], lo_idx: usize, hi_idx: usize) {
    if lo_idx >= hi_idx {
        return;
    }
    let partition_idx = partition(slice, lo_idx, hi_idx);
    if partition_idx > 0 {
        sort_range(slice, lo_idx, partition_idx - 1);
    }
    sort_range(slice, partition_idx + 1, hi_idx);
}

pub fn partition<T: Ord>(slice: &mut [T], lo_idx: usize, hi_idx: usize) -> usize {
    let mut i = lo_idx;
    for j in i..hi_idx {
        if slice[j] <= slice[hi_idx] {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(hi_idx, i);
    i
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