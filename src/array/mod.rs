mod binary_search;
mod validate_single_parentheses;
mod validate_multiple_parentheses;
mod evicting_ring_buffer;
mod array_dedup;
mod product_of_numbers;

#[test]
fn array() {
    let array_of_ints: [usize; 3] = [0; 3]; // create immutable array of size 3 with default values 0

    assert_eq!(array_of_ints.len(), 3);
    assert_eq!(array_of_ints[..], [0, 0, 0]); // compare with slice


    let mut array_of_str = ["-", "-"]; // create mutable array with predefined values
    array_of_str[1] = "+";
    assert_eq!(array_of_str.len(), 2);
    assert_eq!(array_of_str[..], ["-", "+"]);

    for s in &array_of_str {
        println!("{}", s)
    }

    // iterate using iterator
    let mut arr_iter = array_of_str.iter();
    while let Some(s) = arr_iter.next() {
        println!("{}", s);
    }
}

#[test]
fn iterate_and_print() {
    for i in [1, 2, 3].iter() {
        println!("this is {}", i);
    }
}

#[test]
fn iterate_and_print_with_index() {
    for (idx, c) in ['a', 'b', 'c'].iter().enumerate() {
        println!("char {} at index {}", c, idx);
    }
}

#[test]
fn iterate_and_change_values() {
    let mut arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    for x in arr.iter_mut() {
        *x = x.repeat(3);
    }
    assert_eq!(["aaa".to_string(), "bbb".to_string(), "ccc".to_string()], arr);
}

#[test]
fn linear_search_for() {
    let search_for = 5;
    for (i, elem) in [1, 2, 3, 5, 7, 5].iter().enumerate() {
        if elem == &search_for {
            println!("found {} at index: {}", search_for, i)
        }
    }
}

#[test]
fn find_first_even_number() {
    let is_even = |&x| x % 2 == 0;

    let found = [1, 1, 3, 8, 7, 5].iter().find(|&x| is_even(x));

    assert_eq!(Some(&8), found);

    let found = [1, 1, 3, 5].iter()
        .find(|&x| is_even(x));

    assert_eq!(None, found);
}

#[test]
fn collect_all_even() {
    let is_even = |x| x % 2 == 0;

    let all_even: Vec<_> = [1, 2, 3, 4].iter()
        .filter(|x| is_even(*x))
        .collect();

    assert_eq!(2, all_even.len());
    assert_eq!(&2, all_even[0]);
    assert_eq!(&4, all_even[1]);
}

#[test]
fn max() {
    assert_eq!(Some(&99), [1, 99, -777].iter().max());
}


#[test]
fn max_by_key() {
    struct Person { name: &'static str, age: usize }
    let people = [
        Person { name: "Alex", age: 40 },
        Person { name: "John", age: 25 },
    ];

    let oldest = people.iter()
        .max_by_key(|p| p.age);

    assert_eq!("Alex", oldest.unwrap().name);
}

#[test]
fn average() {
    let arr = [1, 2];
    let average = arr.iter().sum::<i32>() as f32 / arr.len() as f32;

    assert_eq!(true, average.eq(&1.5));
}

#[test]
fn copy_copyable_array() {
    // if array contain Copy type, assigning to another variable will copy array.
    // i32 is copyable
    let mut a = [1, 2, 3];
    let b = a;

    assert_eq!(a, b);
    a[0] = 0;

    assert_eq!(a, [0, 2, 3]);
    assert_eq!(b, [1, 2, 3]);
}

#[test]
fn clone_array() {
    #[derive(PartialEq, Debug, Clone)]
    struct S(i32);

    let mut array_a = [S(1), S(2)];
    // let mut array_b = array_a; // -> moves, doesn't copy (to copy struct S should implement Copy)
    let array_b = array_a.clone();

    println!("array_a: {:?}", array_a);
    println!("array_b: {:?}", array_b);
    assert_eq!(array_a, array_b);

    array_a[0] = S(0);

    println!("array_a: {:?}", array_a);
    println!("array_b: {:?}", array_b);

    assert_eq!(array_a, [S(0), S(2)]);
    assert_eq!(array_b, [S(1), S(2)]);
}

#[test]
fn copy_vec() {
    let src = vec![1, 2, 3];

    let dest_a = src.clone();
    assert_eq!(src, dest_a);

    let mut dest_b: Vec<i32> = vec![];
    dest_b.clone_from(&src);
    assert_eq!(src, dest_b);

    let mut dest_c: Vec<i32> = vec![];
    dest_c.append(&mut dest_b);
    assert_eq!(src, dest_c);
}

#[test]
fn iter_collect_slow() {
    let src = vec![1, 2, 3];
    let dest: Vec<_> = src.iter().copied().collect();
    assert_eq!(src, dest);
}

#[test]
fn copy_vec_brute_force_slow() {
    let src = vec![1, 2, 3];
    let mut dest = Vec::with_capacity(src.len());

    for x in src.iter() {
        dest.push(*x);
    }
    assert_eq!(src, dest);
}

#[test]
fn reverse_array() {
    let mut arr = [1, 2, 3];
    arr.reverse(); // same for Vec
    assert_eq!([3, 2, 1], arr);
}

#[test]
fn reverse_array_brute_force_slow() {
    let mut arr = [1, 2, 3];
    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        // fast and efficient swap, just swap internal pointer, no extra variables or copies
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
    assert_eq!([3, 2, 1], arr);
}