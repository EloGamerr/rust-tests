use elo_test_manager::*;
use test_entity::TestEntity;
use files_handler::*;
use std::{fs};

mod files_handler;
mod test_entity;

/// Just check that we have permissions to write and also that the directory name
/// is still "entities/test_entities"
#[test]
fn test_write() {
    clean();

    let mut several_files_object: SeveralFilesObject<TestEntity> = SeveralFilesObject::new();
    several_files_object.create("test");

    // We must wait that each file is created
    several_files_object.wait_handles();

    let file = fs::File::open("entities/test_entities/test.json");

    assert!(file.is_ok());
}

/// Just check that we have permissions to clean and also that the directory name
/// is still "entities/test_entities"
#[test]
fn test_clean() {
    let mut several_files_object: SeveralFilesObject<TestEntity> = SeveralFilesObject::new();
    several_files_object.create("test");

    // We must wait that each file is created
    several_files_object.wait_handles();

    clean();

    let file = fs::File::open("entities/test_entities/test.json");

    assert!(file.is_err());
}