use std::collections::HashMap;
use std::{fs, thread};
use std::fmt::Debug;
use std::thread::JoinHandle;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Error;

trait IdEntity {
    fn get_id(&self) -> String;
    fn new(name: &str) -> Self;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl<T: IdEntity + Send + Clone + Serialize + Debug + DeserializeOwned> SeveralFilesObject<T> {
    fn new() -> SeveralFilesObject<T> {
        let mut s = SeveralFilesObject {
            id_to_file_entities: HashMap::new(),
            handles: Vec::new()
        };
        s.load();
        return s;
    }

    fn get_or_create(&mut self, id: &str) -> String {
        let entity = self.get(id);
        match entity {
            Some(entity) => entity.get_id(),
            None => self.create(id)
        }
    }

    fn contains(&self, id: &str) -> bool {
        return self.id_to_file_entities.contains_key(&id.to_lowercase());
    }

    fn create(&mut self, id: &str) -> String {
        let entity = T::new(id);
        let id = entity.get_id();
        self.id_to_file_entities.insert(id.to_lowercase(), entity);
        self.update(&id);
        return id;
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        return self.id_to_file_entities.get_mut(&id.to_lowercase());
    }

    fn get(&self, id: &str) -> Option<&T> {
        return self.id_to_file_entities.get(&id.to_lowercase());
    }

    fn update(&mut self, id: &str) {
        let entity = self.id_to_file_entities.get(&id.to_lowercase());
        match entity {
            Some(entity) => {
                let json = serde_json::to_string_pretty(&entity);
                match json {
                    Ok(json) => {
                        let path = String::from("entities/") + &entity.get_id().to_lowercase() + ".json";
                        println!("Mise à jour de l'entité d'id {}", entity.get_id());
                        let handle = thread::spawn(move || {
                            let res = fs::create_dir_all("entities/");
                            match res {
                                Ok(_) => {
                                    let res = fs::write(path, json);
                                    match res {
                                        Ok(_) => {}
                                        Err(err) => println!("{}", err)
                                    }
                                }
                                Err(err) => println!("{}", err)
                            }
                        });
                        self.handles.push(handle);
                    }
                    Err(err) => println!("{}", err)
                }
            },
            None => ()
        }
    }

    fn load(&mut self) {
        let path = "entities/";
        let folder = fs::read_dir(path);
        match folder {
            Ok(folder) => {
                for file in folder {
                    match file {
                        Ok(file) => {
                            if file.path().is_file() && file.path().display().to_string().ends_with(".json") {
                                let json = fs::read_to_string(file.path());
                                match json {
                                    Ok(json) => {
                                        let entity: Result<T, Error> = serde_json::from_str(&json);
                                        match entity {
                                            Ok(entity) => {
                                                match file.file_name().eq_ignore_ascii_case(entity.get_id() + ".json") {
                                                    true => {
                                                        self.id_to_file_entities.insert(entity.get_id().to_lowercase(), entity);
                                                    }
                                                    false => {
                                                        println!("Le nom du fichier '{}' ne correspond pas à l'id de l'entité '{}' !", file.path().display(), entity.get_id());
                                                    }
                                                }
                                            }
                                            Err(err) => println!("{}", err)
                                        }
                                    }
                                    Err(err) => println!("{}", err)
                                }
                            }
                        }
                        Err(err) => println!("{}", err)
                    }
                }
            }
            Err(_) => {}
        }
    }
}

fn main() {
    println!("Hello, world!");


    let mut s: SeveralFilesObject<Test> = SeveralFilesObject::new();
    s.get_or_create("Test2");
    /*let id = s.create("Test2");
    {
        let f2 = s.get_mut(&id);
        f2.set_level(5);
        s.update(&id);
    }
    s.create("Yo");
    let f2 = s.get(&id);
    println!("{} {}", f2.get_id(), f2.level);*/

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
