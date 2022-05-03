use elo_test_manager::FileEntity;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TestEntity {
    name: String
}

impl FileEntity for TestEntity {
    fn get_id(&self) -> String {
        return self.name.clone();
    }

    fn new(name: &str) -> Self {
        Self {
            name: String::from(name)
        }
    }

    fn get_folder_name() -> &'static str {
        "test_entities"
    }
}