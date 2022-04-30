pub mod files {
    pub mod several {
        use std::collections::HashMap;
        use std::{fs, thread};
        use std::collections::hash_map::Iter;
        use std::thread::JoinHandle;
        use serde::de::DeserializeOwned;
        use serde::Serialize;
        use serde_json::Error;
        use std::any::{Any, TypeId};

        pub trait IdEntity {
            fn get_id(&self) -> String;
            fn new(name: &str) -> Self;
            fn get_folder_name() -> &'static str;
        }

        pub struct SeveralFilesObject<T: 'static> {
            id_to_file_entities: HashMap<String, T>,
            handles: Vec<JoinHandle<()>>
        }

        impl<T: IdEntity + Send + Clone + Serialize + DeserializeOwned> SeveralFilesObject<T> {
            pub fn new() -> SeveralFilesObject<T> {
                let mut s = SeveralFilesObject {
                    id_to_file_entities: HashMap::new(),
                    handles: Vec::new()
                };
                s.load();
                return s;
            }

            pub fn get_or_create(&mut self, id: &str) -> String {
                let entity = self.get(id);
                match entity {
                    Some(entity) => entity.get_id(),
                    None => self.create(id)
                }
            }

            pub fn contains(&self, id: &str) -> bool {
                return self.id_to_file_entities.contains_key(&id.to_lowercase());
            }

            pub fn create(&mut self, id: &str) -> String {
                let entity = T::new(id);
                let id = entity.get_id();
                self.id_to_file_entities.insert(id.to_lowercase(), entity);
                self.update(&id);
                return id;
            }

            pub fn get_mut(&mut self, id: &str) -> Option<&mut T> {
                return self.id_to_file_entities.get_mut(&id.to_lowercase());
            }

            pub fn get(&self, id: &str) -> Option<&T> {
                return self.id_to_file_entities.get(&id.to_lowercase());
            }

            pub fn get_thread_safe(id: &str) -> Option<T> {
                let path = String::from("entities/") + T::get_folder_name() + "/" + &id.to_lowercase() + ".json";
                let json = fs::read_to_string(path);
                match json {
                    Ok(json) => {
                        let entity: Result<T, Error> = serde_json::from_str(&json);
                        match entity {
                            Ok(entity) => {
                                match id.eq_ignore_ascii_case(&entity.get_id()) {
                                    true => {
                                        return Some(entity);
                                    },
                                    false => {
                                        println!("Le nom du fichier '{}' ne correspond pas à l'id de l'entité '{}' !", id.to_lowercase(), entity.get_id());
                                    }
                                }
                            }
                            Err(err) => {
                                println!("{}", err);
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }

                return None;
            }

            pub fn get_entities(&self) -> Iter<String, T> {
                self.id_to_file_entities.iter()
            }

            pub fn update(&mut self, id: &str) {
                let entity = self.id_to_file_entities.get(&id.to_lowercase());
                match entity {
                    Some(entity) => {
                        let json = serde_json::to_string_pretty(&entity);
                        match json {
                            Ok(json) => {
                                let path = String::from("entities/") + T::get_folder_name() + "/" + &entity.get_id().to_lowercase() + ".json";
                                println!("Mise à jour de l'entité d'id {}", entity.get_id());
                                let handle = thread::spawn(move || {
                                    let res = fs::create_dir_all(String::from("entities/") + T::get_folder_name());
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

            pub fn wait_handles(self) {
                for handle in self.handles {
                    handle.join().unwrap();
                }
            }

            fn load(&mut self) {
                let path = String::from("entities/") + T::get_folder_name();
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
    }
}