use rand::*;

fn random_from_list<T>(slice: &[T]) -> Option<&T> {
    let rnd_idx: usize = thread_rng().gen_range(0, slice.len());
    slice.get(rnd_idx)
}

#[test]
fn test() {
    let languages = ["Java", "Kotlin", "Go", "Python", "JavaScript", "Rust"];
    let actual = random_from_list(&languages).unwrap();

    println!("{}", actual);
}