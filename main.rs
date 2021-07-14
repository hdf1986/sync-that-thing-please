mod packages;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde_yaml::{Value};
use std::fs;
use std::io::Write;
use std::path::Path;

static REQUIRED_FOLDERS: &str = "
structure:
  $HOME:
    .sync-that-thing-please:
      tmp:
      config:
      bin:
      source:
";

static DEFAULT_FOLDER_FILE: &str = "
structure:
  $HOME:
    workspace2
";

static DEFAULT_PACKAGES_FILE: &str = "
snap_packages:
apt_packages:
git_repos:
";

fn ensure_folders_exists(yaml_content: &String) {
  fn mkdirp(path: &String) {
    let expanded_path = String::from(shellexpand::env(path).unwrap());
    match fs::create_dir_all(expanded_path) {
      Ok(_) => (),
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

  let application_data: HashMap<String, Value> = serde_yaml::from_str(&yaml_content).unwrap();
  iterate_and_create(application_data.get("structure").unwrap(), &String::from(""));
}

fn ensure_packages (yaml_content: &String) {
  let application_data: HashMap<String, Value> = serde_yaml::from_str(&yaml_content).unwrap();
  match application_data.get("snap_packages").unwrap() {
    Value::Sequence(vector) => {
      
      for package in vector {
        match package {
          Value::String(name) => {
            // TODO: Potential command injection here
            packages::snap_install(&name);
          },
          _ => println!("{:?}", package)
        }
      }
      println!("{:?}", vector);
    },
    _ => println!("{:?}\n", application_data)
  }
}

fn review_packages () {
  let packages_filename = &String::from(shellexpand::env("$HOME/.sync-that-thing-please/config/packages.yaml").unwrap());
  match File::open(packages_filename) {
    Ok(mut file) => {
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();
      ensure_packages(&content);
    },
    Err(_) => {
      if !Path::new(packages_filename).exists() {
        let mut file = File::create(packages_filename).unwrap();
        file.write_all(DEFAULT_PACKAGES_FILE.as_bytes()).unwrap();
      }
    }
  }
}

fn main () {
  ensure_folders_exists(&String::from(REQUIRED_FOLDERS));
  let filename = &String::from(shellexpand::env("$HOME/.sync-that-thing-please/config/folder_structure.yaml").unwrap());

  match File::open(filename) {
    Ok(mut file) => {
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();
      ensure_folders_exists(&content);
    },
    Err(_) => {
      if !Path::new(filename).exists() {
        let mut file = File::create(filename).unwrap();
        file.write_all(DEFAULT_FOLDER_FILE.as_bytes()).unwrap();
      }
    }
  }
  review_packages()
}