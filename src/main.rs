use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

trait IdEntity {
    fn get_id(&self) -> String;
    fn new(name: &str) -> Self;
}

struct Test {
    name: String,
}

impl Test {

}

impl IdEntity for Test {
    fn get_id(&self) -> String {
        self.name.clone()
    }

    fn new(name: &str) -> Test {
        Test {
            name: String::from(name)
        }
    }
}

struct FileEntity<T> {
    entity: T,
}

impl<T: IdEntity> FileEntity<T> {
    fn new(entity: T) -> FileEntity<T> {
        FileEntity {
            entity
        }
    }

    fn get_entity(&self) -> &T {
        &self.entity
    }

    fn get_id(&self) -> String {
        self.entity.get_id()
    }

    fn save(&mut self) {
        //self.several_files_object.add_or_update(self);
    }
}

struct SeveralFilesObject<T> {
    id_to_file_entities: HashMap<String, FileEntity<T>>
}

impl<T: IdEntity> SeveralFilesObject<T> {
    fn new() -> SeveralFilesObject<T> {
        SeveralFilesObject {
            id_to_file_entities: HashMap::new()
        }
    }

    fn create(&mut self, id: &str) -> &FileEntity<T> {
        let entity = T::new(id);
        let file_entity = FileEntity::new(entity);
        return self.add_or_update(file_entity);
    }

    fn add_or_update(&mut self, file_entity: FileEntity<T>) -> &FileEntity<T> {
        let id = file_entity.get_id().to_lowercase();
        self.id_to_file_entities.insert(id.clone(), file_entity);
        return self.id_to_file_entities.get(&id).unwrap();
    }
}

fn main() {
    println!("Hello, world!");

    let mut s: SeveralFilesObject<Test> = SeveralFilesObject::new();
    let f = s.create("Test2");

    println!("{}", f.get_id());

    for (key, value) in s.id_to_file_entities.iter() {
        println!("{} {}", key, value.get_id());
    }

   /* let mut s = SeveralFilesObject::new();
    let mut f;
    {
        f = FileEntity::new(Test::new("yo"), &mut s);
    }

    let mut f2 = FileEntity::new(Test::new("yo2"), &s);
    println!("{}", f.get_id());
    println!("{}", f2.get_id());
    println!("{}", f.get_id());
    println!("Save");
    f.save();
    f2.save();*/
}
