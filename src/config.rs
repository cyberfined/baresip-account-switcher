use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind};
use std::path::Path;
use toml::de;

use crate::baresip::{config, UserAgent};

#[derive(Deserialize)]
struct AccountConfig {
    login: String,
    domain: String,
    transport: String,
    answermode: String,
    auth_pass: String,
}

#[derive(Deserialize)]
struct AccountsConfig {
    accounts: HashMap<String, AccountConfig>,
}

pub fn read_config() -> io::Result<HashMap<String, UserAgent>> {
    let cfg_dir_str = config::get_path()?;
    let cfg_dir = Path::new(&cfg_dir_str);
    let cfg_path = cfg_dir.join("accounts.toml");

    if !cfg_path.is_file() {
        if !cfg_dir.exists() {
            fs::create_dir_all(&cfg_dir)?;
        }
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&cfg_path)?;
    }

    let content = fs::read_to_string(cfg_path)?;
    let accounts_config: AccountsConfig = de::from_str(&content)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err.to_string()))?;
    let mut user_agents = HashMap::new();

    for (title, account_config) in accounts_config.accounts.iter() {
        let address = format!(
            "<sip:{}@{};transport={}>;answermode={};auth_pass={}",
            account_config.login,
            account_config.domain,
            account_config.transport,
            account_config.answermode,
            account_config.auth_pass
        );
        let user_agent = UserAgent::new(&address)?;
        user_agents.insert(title.clone(), user_agent);
    }

    Ok(user_agents)
}
