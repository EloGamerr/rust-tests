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
    level: i16
}

impl Test {
    fn set_level(&mut self, level: i16) {
        self.level = level;
        self.save();
    }

    fn save(&self) {
        //self.file_entity.save(self);
    }
}

impl IdEntity for Test {
    fn get_id(&self) -> String {
        self.name.clone()
    }

    fn new(name: &str) -> Test {
        Test {
            name: String::from(name),
            level: 0
        }
    }
}

struct FileEntity<T: 'static> {
    several: &'static SeveralFilesObject<T>
}

impl<T: IdEntity> FileEntity<T> {
    fn new(several: &'static SeveralFilesObject<T>) -> FileEntity<T> {
        FileEntity {
            several
        }
    }

    fn save(&self, entity: &T) {
        self.several.update(entity);
    }
}

struct SeveralFilesObject<T: 'static> {
    id_to_file_entities: HashMap<String, T>
}

impl<T: IdEntity> SeveralFilesObject<T> {
    fn new() -> SeveralFilesObject<T> {
        SeveralFilesObject {
            id_to_file_entities: HashMap::new()
        }
    }

    fn create(&mut self, id: &str) -> String {
        let entity = T::new(id);
        return self.add(entity);
    }

    fn get_mut(&mut self, id: String) -> &mut T {
        return self.id_to_file_entities.get_mut(&id.to_lowercase()).unwrap();
    }

    fn add(&mut self, entity: T) -> String {
        let id = entity.get_id();
        self.id_to_file_entities.insert(id.to_lowercase(), entity);
        let entity = self.id_to_file_entities.get(&id.to_lowercase()).unwrap();
        self.update(entity);
        return id;
    }

    fn update(&self, entity: &T) {
        let id = entity.get_id().to_lowercase();
        println!("Mise à jour de l'entité d'id {}", id);
        // Mise à jour du fichier
    }
}

fn main() {
    println!("Hello, world!");

    let mut s: SeveralFilesObject<Test> = SeveralFilesObject::new();
    let id = s.create("Test2");
    let f2 = s.get_mut(id);
    f2.level = 5;
    println!("{} {}", f2.get_id(), f2.level);

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
