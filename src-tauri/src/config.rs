use home::home_dir;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string};

use crate::SSH_PROCESSES;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SSHConfig {
    is_running: bool,
    host: String,
    is_remote_forwarding: bool,
    arguments: HashMap<String, String>,
}

pub trait IRunningStatusReflection {
    fn apply_running_status(&mut self);
}

macro_rules! push_if_some {
    ($vec:ident, $item:ident) => {
        if let Some(tmp) = $item.clone() {
            $vec.push(tmp);
        }
    };
}

impl SSHConfig {
    pub fn read_all() -> Result<Vec<SSHConfig>, String> {
        let home_path = home_dir().ok_or("The home directory is not found.".to_string())?;
        let ssh_config_file =
            read_to_string(home_path.join(".ssh").join("config")).map_err(|err| err.to_string())?;

        let re = Regex::new(r"^\s*(\S+)\s+(.*)$").unwrap();
        let mut config_found = Vec::new();
        let mut current_config = None;

        for line in ssh_config_file.lines() {
            if let Some(matches) = re.captures(line) {
                let name = matches.get(1).unwrap().as_str();
                let params = matches.get(2).unwrap().as_str();
                match name {
                    "Host" => {
                        push_if_some!(config_found, current_config);
                        let mut next_config = Self::default();
                        next_config.host = params.to_string();
                        current_config = Some(next_config);
                    }
                    "RemoteForward" => {
                        if let Some(config) = current_config.as_mut() {
                            config.is_remote_forwarding = true;
                        }
                    }
                    name => {
                        if let Some(config) = current_config.as_mut() {
                            config
                                .arguments
                                .insert(name.to_string(), params.to_string());
                        }
                    }
                }
            }
        }
        push_if_some!(config_found, current_config);

        Ok(config_found)
    }
}

impl IRunningStatusReflection for Vec<SSHConfig> {
    fn apply_running_status(&mut self) {
        let processes = SSH_PROCESSES.lock().unwrap();
        for config in self {
            if processes.contains_key(&config.host) {
                config.is_running = true;
            } else {
                config.is_running = false;
            }
        }
    }
}
