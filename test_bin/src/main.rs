use std::thread;
use manager::files::several::{IdEntity, SeveralFilesObject};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct Test {
    name: String,
    level: i16
}

impl Test {
    fn set_level(&mut self, level: i16) {
        self.level = level;
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

    fn get_folder_name() -> &'static str {
        "test"
    }
}

fn main() {
    let mut s: SeveralFilesObject<Test> = SeveralFilesObject::new();
    s.get_or_create("Test2");
    let id = s.create("Test2");
    {
        let f2 = s.get_mut(&id).unwrap();
        f2.set_level(5);
        s.update(&id);
    }
    s.create("Yo");
    let f2 = s.get(&id).unwrap();
    println!("{} {}", f2.get_id(), f2.level);

    for (key, value) in s.get_entities() {
        println!("{} {}", key, value.get_id());
    }

    let t = thread::spawn(move || {
        let ent: Option<Test> = SeveralFilesObject::get_thread_safe("yo");
        match ent {
            None => {}
            Some(ent) => {
                println!("{}", ent.get_id());
            }
        }
    });
    t.join().unwrap();
    s.wait_handles();
}
