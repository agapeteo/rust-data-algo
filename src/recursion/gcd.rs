fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[test]
fn test() {
    assert_eq!(gcd(12, 6), 6);
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(9, 11), 1);
}

#[test]
fn remove_me() {
    let mut n_nonzero = 0;
    for i in 0..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };
        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }
    println!("non-zero bytes in memory: {}", n_nonzero);
}