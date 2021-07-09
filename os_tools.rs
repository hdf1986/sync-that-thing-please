use sysinfo::{SystemExt};
pub struct OsInfo {
  system: Option<sysinfo::System>
}

impl OsInfo {
  pub fn fetch_info() -> OsInfo {
    let mut info = OsInfo {
      system: None
    };

    info.refresh_info();
    info
  }

  pub fn refresh_info (&mut self) {
    self.system = Some(sysinfo::System::new_all());
  }

  pub fn version (&self) -> Option<String> {
    match &self.system {
      Some(system) => system.os_version(),
      None => panic!("No system info found!")
    }
  }

  pub fn name (&self) -> Option<String> {
    match &self.system {
      Some(system) => system.name(),
      None => panic!("No system info found!")
    }
  }
}