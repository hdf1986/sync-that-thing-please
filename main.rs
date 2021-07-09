use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde_yaml::{Value};

fn ensure_folders_exists() {
  let filename = "file_structure.yaml";

  fn iterate_and_create (node: &Value, path: &String) {
    match node {
      Value::String(_) => {
        println!("Hubiera creado carpeta {}{}\n", &path, node.as_str().unwrap());
      },
      Value::Mapping(content) => {
        for (key, item) in content {
          println!("Hubiera creado carpeta {}{}\n", &path, key.as_str().unwrap());

          let mut new_path: String = path.to_owned();
          new_path.push_str(key.as_str().unwrap());
          new_path.push_str("/");

          iterate_and_create(item, &new_path);
        }
      },
      Value::Null => (),
      _ => println!("{:?}\n", node)
    }
  }

  match File::open(filename) {
    Ok(mut file) => {
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();

      let application_data: HashMap<String, Value> = serde_yaml::from_str(&content).unwrap();
      iterate_and_create(application_data.get("structure").unwrap(), &String::from(""));
    }
    Err(error) => {
        println!("There is an error {}: {}", filename, error);
    }
  }
}

fn main () {
  ensure_folders_exists();
}