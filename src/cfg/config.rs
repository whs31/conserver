use std::path::{
  Path,
  PathBuf
};
use anyhow::ensure;
use serde::{
  Deserialize, 
  Serialize
};
use crate::error::*;

nestify::nest! {
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]*
  pub struct Config {
    pub credentials: Option<pub struct Credentials {
      pub username: String,
      pub access_token: Option<String>
    }>
  }
}

#[derive(Debug, Clone)]
pub struct ConfigFile {
  pub path: PathBuf,
  pub config: Config
}

impl Default for Credentials {
  fn default() -> Self {
    Self {
      username: "user".to_owned(),
      access_token: None
    }
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      credentials: None
    }
  }
}

impl Default for ConfigFile {
  fn default() -> Self {
    Self {
      path: std::env::current_dir()
        .unwrap()
        .join("conserver.toml"),
      config: Config::default()
    }
  }
}

impl ConfigFile {
  pub fn new(path: &Path) -> Result<Self> {
    let mut this = Self {
      path: path.to_owned(),
      config: Config::default()
    };

    match Self::validate_path(&this.path) {
      Ok(_) => this.load()?,
      Err(_) => this.save()?
    };

    Ok(this)
  }

  pub fn save(&self) -> Result<&Self> {
    let content = serde_yml::to_string(&self.config)?;
    std::fs::write(&self.path, content)?;
    Ok(self)
  }

  pub fn load(&mut self) -> Result<&Self> {
    Self::validate_path(&self.path)?;
    let content = std::fs::read_to_string(&self.path)?;
    self.config = serde_yml::from_str(&content)?;
    Ok(self)
  }

  fn validate_path(path: &Path) -> Result<()> {
    ensure!(path.exists(), "config file does not exist: {}", path.display());
    ensure!(path.is_file(), "config file is not a file: {}", path.display());

    Ok(()) 
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const CONFIG_FILE: &str = r#"credentials:
  username: user
  access_token: null
"#;

  #[test]
  fn test_serialize() -> crate::error::Result<()> {
    let config = Config {
      credentials: Some(Credentials {
        username: "user".to_owned(),
        access_token: None
      })
    };
    let content = serde_yml::to_string(&config)?;
    assert_eq!(content, CONFIG_FILE);

    Ok(())
  }

  #[test]
  fn test_deserialize() -> crate::error::Result<()> {
    let config = serde_yml::from_str::<Config>(CONFIG_FILE)?;
    assert_eq!(config, Config {
      credentials: Some(Credentials {
        username: "user".to_owned(),
        access_token: None
      })
    });

    Ok(())
  }
}