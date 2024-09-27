use std::{fmt::Display, path::PathBuf};
use url::Url;
use serde::{
  Deserialize, 
  Serialize
};
use git2::{
  build::RepoBuilder,
  Error,
  FetchOptions,
  RemoteCallbacks
};
use crate::error::Result;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Branch {
  Single(String),

  #[default]
  All 
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repository {
  pub url: Url,
  pub branch: Branch,
  pub recurse_submodules: bool
}

impl Display for Branch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Single(s) => write!(f, "{}", s),
      Self::All => write!(f, "all")
    }
  }
}

impl Repository {
  pub async fn clone_from_git<T>(&self, target: T) -> Result<&Self> where 
    T: Into<PathBuf> {
    let source: String = self.url.to_string();
    let target: PathBuf = target.into();
    let branch = self.branch.clone();

    tokio::task::spawn_blocking(move || -> Result<()> {
      let callbacks = RemoteCallbacks::new();
      // progress
      // credentials
      let mut fetch_opts = FetchOptions::new();
      fetch_opts.remote_callbacks(callbacks);
      let mut builder = RepoBuilder::new();
      builder
        .bare(false)
        .fetch_options(fetch_opts);
      match branch {
        Branch::Single(s) => { builder.branch(s.as_str()); },
        Branch::All => {}
      };
      builder.clone(&source, &target)?;

      Ok(())
    }).await??;

    Ok(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialize() -> crate::error::Result<()> {
    let repo = Repository {
      url: Url::parse("https://github.com/whs31/conserver")?,
      branch: Branch::Single("master".to_owned()),
      recurse_submodules: true
    };

    let content = serde_yml::to_string(&repo)?;
    assert_eq!(content, "url: https://github.com/whs31/conserver\nbranch: !Single master\nrecurse_submodules: true\n"); 

    Ok(())
  }

  #[test]
  fn test_deserialize() -> crate::error::Result<()> {
    let repo = serde_yml::from_str::<Repository>("url: https://github.com/whs31/conserver\nbranch: !Single master\nrecurse_submodules: true\n")?;
    assert_eq!(repo, Repository {
      url: Url::parse("https://github.com/whs31/conserver")?, 
      branch: Branch::Single("master".to_owned()),
      recurse_submodules: true
    });

    Ok(())
  }

  #[tokio::test]
  async fn test_clone_from_git() -> crate::error::Result<()> {
    let repo = Repository {
      url: Url::parse("https://github.com/octocat/Hello-World")?,
      branch: Branch::All,
      recurse_submodules: false
    };

    let _ = repo.clone_from_git("target/tests/test-clone").await?;
    
    std::fs::remove_dir_all("target/tests/test-clone")?;

    Ok(())
  }
}