use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

trait IdEntity {
    fn get_id(&self) -> String;
    fn new(name: &str) -> Self;
}

#[derive(Clone)]
struct Test {
    name: String,
    level: i16
}

impl Test {
    fn set_level(&mut self, level: i16) {
        self.level = level;
    }
}

unsafe impl Send for Test {}

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

struct SeveralFilesObject<T: 'static> {
    id_to_file_entities: HashMap<String, T>,
    handles: Vec<JoinHandle<()>>
}

impl<T: IdEntity + Send + Clone> SeveralFilesObject<T> {
    fn new() -> SeveralFilesObject<T> {
        SeveralFilesObject {
            id_to_file_entities: HashMap::new(),
            handles: Vec::new()
        }
    }

    fn create(&mut self, id: &str) -> String {
        let entity = T::new(id);
        return self.add(entity);
    }

    fn get_mut(&mut self, id: &str) -> &mut T {
        return self.id_to_file_entities.get_mut(&id.to_lowercase()).unwrap();
    }

    fn get(&self, id: &str) -> &T {
        return self.id_to_file_entities.get(&id.to_lowercase()).unwrap();
    }

    fn add(&mut self, entity: T) -> String {
        let id = entity.get_id();
        self.id_to_file_entities.insert(id.to_lowercase(), entity);
        self.update(&id);
        return id;
    }

    fn update(&mut self, id: &str) {
        let entity = self.id_to_file_entities.get(&id.to_lowercase());
        match entity {
            Some(entity) => {
                // On doit clone l'entité pour pouvoir la move dans le nouveau thread sans que la map ne
                // perde sa propriété
                let entity = entity.clone();
                let handle = thread::spawn(move || {
                    println!("Mise à jour de l'entité d'id {}", entity.get_id());
                });
                self.handles.push(handle);
            },
            None => ()
        }
    }
}

fn main() {
    println!("Hello, world!");


    let mut s: SeveralFilesObject<Test> = SeveralFilesObject::new();
    let id = s.create("Test2");
    {
        let f2 = s.get_mut(&id);
        f2.set_level(5);
        let id = f2.get_id();
        s.update(&id);
    }
    let f2 = s.get(&id);
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

    for handle in s.handles {
        handle.join().unwrap();
    }
}
