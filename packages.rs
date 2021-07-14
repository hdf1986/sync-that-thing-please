use std::process::Command;
use std::path::Path;

pub fn is_available(package_name: &str) -> bool {
  let which_command =  Command::new("which")
                        .arg(package_name)
                        .status()
                        .expect("which command failed to start");

  which_command.success()
}

pub fn snap_install(package_name: &str) -> bool {
  let mut package_filename: String = "/snap/".to_owned();
  package_filename.push_str(package_name);
  package_filename.push_str("/current");
  
  if Path::new(&package_filename).exists() {
    return true;
  }
  let which_command =  Command::new("snap")
                        .arg("install")
                        .arg(package_name)
                        .status()
                        .expect("which command failed to start");

  which_command.success()
}