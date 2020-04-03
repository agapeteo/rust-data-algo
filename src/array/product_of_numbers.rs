use std::slice::Iter;

/**
* return array of product of numbers except current value
*/

pub fn product_of_numbers_simple(slice: &[usize]) -> Vec<usize> {
    assert!(slice.len() > 2);

    let total_product: usize = slice.iter().product();

    let mut result: Vec<usize> = vec![usize::default(); slice.len()];

    for i in 0..slice.len() {
        result[i] = total_product / slice[i];
    }
    result
}

pub fn product_of_numbers(slice: &[usize]) -> Vec<usize> {
    let prod_asc: Vec<usize> = accumulative_product(&mut slice.iter(), slice.len(), false);
    let prod_desc: Vec<usize> = accumulative_product(&mut slice.iter(), slice.len(), true);

    let mut result: Vec<usize> = vec![usize::default(); slice.len()];

    for i in 0..slice.len() {
        if i == 0 {
            result[i] = prod_desc[i + 1];
        } else if i == slice.len() - 1 {
            result[i] = prod_asc[slice.len() - 2];
        } else {
            result[i] = prod_desc[i + 1] * prod_asc[i - 1];
        }
    }
    result
}

fn accumulative_product(iter: &mut Iter<usize>, len: usize, reverse: bool) -> Vec<usize> {
    let mut result: Vec<usize> = vec![usize::default(); len];

    for i in 0..len {
        let n = if reverse {
            *iter.rev().next().unwrap()
        } else {
            *iter.next().unwrap()
        };

        if i == 0 {
            result[i] = n;
        } else {
            result[i] = n * result[i - 1];
        }
    }
    if reverse {
        result.reverse();
    }
    result
}

#[test]
fn test_simple() {
    let arr = [1, 2, 3, 4, 5];

    assert_eq!(product_of_numbers_simple(&arr), [120, 60, 40, 30, 24]);
}

#[test]
fn test() {
    let arr = [1, 2, 3, 4, 5];

    assert_eq!(product_of_numbers(&arr), [120, 60, 40, 30, 24]);
}