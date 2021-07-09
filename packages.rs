use std::process::Command;

pub fn is_available(package_name: &str) -> bool {
  let which_command =  Command::new("which")
                        .arg(package_name)
                        .status()
                        .expect("which command failed to start");

  which_command.success()
}