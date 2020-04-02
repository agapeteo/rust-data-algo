use std::path::Path;

#[allow(dead_code)]
pub fn traverse_dirs(dir_path: &Path, ignore_hidden: bool) {
    if !dir_path.exists() || !dir_path.is_dir() {
        panic!(format!("incorrect path {}", dir_path.to_str().unwrap()));
    }
    traverse_dir(dir_path, 0, ignore_hidden);
}

#[allow(dead_code)]
fn traverse_dir(path: &Path, tabs_count: usize, ignore_hidden: bool) {
    let f_name = path.file_name().unwrap().to_str().unwrap_or("?invalid_file_name?");
    if ignore_hidden && f_name.starts_with('.') {
        return;
    }
    let tabs_str = "\t".repeat(tabs_count);

    let f_name = tabs_str + f_name;

    println!("{}", f_name);

    if path.is_dir() {
        for f in path.read_dir().unwrap() {
            traverse_dir(&f.unwrap().path(), tabs_count + 1, ignore_hidden);
        }
    }
}

#[test]
fn test() {
    traverse_dirs(Path::new("/Users/some_user/some_folder"), true);
}