use std::fs;

#[allow(dead_code)]
pub fn clean() {
    let _ = fs::remove_dir_all("entities/test_entities");
}