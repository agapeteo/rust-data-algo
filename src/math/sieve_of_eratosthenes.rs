use std::collections::HashSet;

fn primes(num: usize) -> Vec<usize> {
    let mut not_primes: HashSet<usize> = HashSet::new();
    let mut result: Vec<usize> = Vec::new();

    for i in 2..=num {
        if not_primes.contains(&i) {
            continue
        }
        result.push(i);

        let mut j = i;
        while j <= num {
            j += i;
            not_primes.insert(j);
        }
    }
    result
}

#[test]
fn test() {
    let primes = primes(100);
    assert_eq!(primes,
               [2, 3, 5, 7,
                   11, 13, 17, 19,
                   23, 29,
                   31, 37,
                   41, 43, 47,
                   53, 59,
                   61, 67,
                   71, 73, 79,
                   83, 89,
                   97]);
}