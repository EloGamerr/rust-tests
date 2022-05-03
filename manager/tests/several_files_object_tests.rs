use elo_test_manager::*;
use test_entity::TestEntity;
use crate::files_handler::clean;

mod files_handler;
mod test_entity;

#[test]
fn test_create() {
    let mut several_files_object: SeveralFilesObject<TestEntity> = SeveralFilesObject::new();
    let real_id = "test";
    let id = several_files_object.create(real_id);

    assert_eq!(real_id, id);
}

#[test]
fn test_load() {
    clean();

    let mut several_files_object: SeveralFilesObject<TestEntity> = SeveralFilesObject::new();
    let id = several_files_object.create("test");

    several_files_object.wait_handles();

    let mut several_files_object_new: SeveralFilesObject<TestEntity> = SeveralFilesObject::new();
    several_files_object_new.load();

    assert!(several_files_object_new.contains(&id));
}