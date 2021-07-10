use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde_yaml::{Value};
use std::fs;

fn ensure_folders_exists() {
  let filename = "file_structure.yaml";
  fn mkdirp(path: &String) {
    let expanded_path = String::from(shellexpand::env(path).unwrap());
    match fs::create_dir_all(expanded_path) {
      Ok(_) => println!("Created {} and it's parent nodes", &path),
      Err(error) => panic!("Error creating {}: {:?}", path, error)
    }
  }

  fn iterate_and_create (node: &Value, path: &String) {
    match node {
      Value::String(_) => {
        let mut new_path: String = path.to_owned();
        new_path.push_str(node.as_str().unwrap());
        new_path.push_str("/");

        mkdirp(&new_path);
      },
      Value::Mapping(content) => {
        for (key, item) in content {
          let mut new_path: String = path.to_owned();
          new_path.push_str(key.as_str().unwrap());
          new_path.push_str("/");

          iterate_and_create(item, &new_path);
        }
      },
      Value::Null => mkdirp(&path),
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