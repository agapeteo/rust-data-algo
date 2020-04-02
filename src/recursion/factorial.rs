fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}

#[test]
fn test() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(5), 120);
}