#[test]
fn using_built_in() {
    let numbers = [0, 2, 4, 6, 8];

    assert_eq!(Ok(0), numbers.binary_search(&0));
    assert_eq!(Ok(3), numbers.binary_search(&6));

    let expected_missing = Err(2); // Err(2) means that 3 is missing, but would be inserted at idx 3
    assert_eq!(expected_missing, numbers.binary_search(&3));
}

fn binary_search<T>(slice: &[T], target: &T) -> Result<usize, usize>
    where T: Ord {
    let mut low_idx = 0;
    let mut high_idx = slice.len() - 1;

    while low_idx < high_idx {
        let mid_idx = (low_idx + high_idx) / 2;
        let mid_value = &slice[mid_idx];

        if *mid_value == *target {
            return Ok(mid_idx);
        }

        if *mid_value > *target {
            high_idx = mid_idx - 1;
        } else {
            low_idx = mid_idx + 1;
        }
    }
    Err(low_idx + 1)
}

#[test]
fn check_simple_binary_search() {
    let numbers = [0, 2, 4, 6, 8];

    assert_eq!(Ok(0), binary_search(&numbers, &0));
    assert_eq!(Ok(3), binary_search(&numbers, &6));

    let expected_missing = Err(2); // Err(2) means that 3 is missing, but would be inserted at idx 3
    assert_eq!(binary_search(&numbers, &3), expected_missing);
}

#[test]
fn bla() {
    let a: f32 = 42.42;
    println!("{:032b}", a as u32);
    println!("{}", a as u32);

    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };
    println!("{:032b}", frankentype);
    println!("{}", frankentype);
}
