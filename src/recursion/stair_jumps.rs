use std::collections::HashMap;

fn ways(stairs: i32, jump_ways: &[i32]) -> usize {
    if stairs < 0 {
        return 0;
    }
    if stairs == 0 {
        return 1;
    }
    let mut total_ways = 0;
    for cur_jump in jump_ways {
        total_ways += ways(stairs - *cur_jump, jump_ways);
    }
    total_ways
}

fn ways_dynamic(stairs: i32, jump_ways: &[i32]) -> usize {
    let mut cache: HashMap<i32, usize> = HashMap::new();
    ways_cached(stairs, jump_ways, &mut cache)
}

fn ways_cached(stairs: i32, jump_ways: &[i32], cache: &mut HashMap<i32, usize>) -> usize {
    if stairs < 0 {
        return 0;
    }
    if stairs == 0 {
        return 1;
    }
    let mut total_ways = 0;
    for cur_jump in jump_ways {
        let cur_stairs = stairs - *cur_jump;
        if let Some(count) = cache.get(&cur_stairs) {
            total_ways += *count;
        } else {
            cache.insert(cur_stairs, ways(cur_stairs, jump_ways));
            total_ways += cache[&cur_stairs];
        }
    }
    total_ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ways() {
        assert_eq!(ways(1, &[1]), 1);
        assert_eq!(ways(1, &[1, 2]), 1);
        assert_eq!(ways(2, &[1]), 1);
        assert_eq!(ways(2, &[1, 2]), 2);
        assert_eq!(ways(3, &[1, 2]), 3);
        assert_eq!(ways(3, &[1, 2, 3]), 4);
        assert_eq!(ways(3, &[1, 2, 3]), 4);
        assert_eq!(ways(5, &[1, 2, 5]), 9);
        assert_eq!(ways(5, &[1, 2, 3]), 13);
    }

    #[test]
    fn test_ways_dynamic() {
        assert_eq!(ways_dynamic(1, &[1]), 1);
        assert_eq!(ways_dynamic(1, &[1, 2]), 1);
        assert_eq!(ways_dynamic(2, &[1]), 1);
        assert_eq!(ways_dynamic(2, &[1, 2]), 2);
        assert_eq!(ways_dynamic(3, &[1, 2]), 3);
        assert_eq!(ways_dynamic(3, &[1, 2, 3]), 4);
        assert_eq!(ways_dynamic(3, &[1, 2, 3]), 4);
        assert_eq!(ways_dynamic(5, &[1, 2, 5]), 9);
        assert_eq!(ways_dynamic(5, &[1, 2, 3]), 13);
    }
}
