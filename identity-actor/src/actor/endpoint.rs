// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use crate::Result;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint {
  name: String,
  handler: String,
  is_hook: bool,
}

impl Endpoint {
  pub fn new(string: impl AsRef<str>) -> Result<Self> {
    let mut is_hook = false;
    let mut split = string.as_ref().split('/');

    let name = split.next().unwrap().to_owned();
    let handler = split.next().ok_or(Error::InvalidEndpoint)?.to_owned();
    let hook = split.next();

    if name.is_empty()
      || handler.is_empty()
      || !name.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
      || !handler.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
    {
      return Err(Error::InvalidEndpoint);
    }

    if let Some(hook) = hook {
      if hook != "hook" {
        return Err(Error::InvalidEndpoint);
      }
      is_hook = true;
    }

    if split.next().is_some() {
      return Err(Error::InvalidEndpoint);
    }

    Ok(Self { name, handler, is_hook })
  }

  pub fn new_hook(string: impl AsRef<str>) -> Result<Self> {
    let mut endpoint = Self::new(string)?;
    endpoint.is_hook = true;
    Ok(endpoint)
  }

  pub fn set_is_hook(&mut self, is_hook: bool) {
    self.is_hook = is_hook;
  }

  pub fn is_hook(&self) -> bool {
    self.is_hook
  }
}

impl Display for Endpoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.name, self.handler)?;
    if self.is_hook {
      write!(f, "/hook")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::Endpoint;
  use crate::Error;

  #[test]
  fn invalid_endpoints() {
    assert!(matches!(Endpoint::new("").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("//").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("/b").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/c").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/c/d").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("1a/b").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b2").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/钩").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(
      Endpoint::new("longer/hyphenated-word").unwrap_err(),
      Error::InvalidEndpoint
    ));
  }

  #[test]
  fn valid_endpoints() {
    assert!(!Endpoint::new("a/b").unwrap().is_hook());
    assert!(Endpoint::new("a/b/hook").unwrap().is_hook());
    assert!(!Endpoint::new("longer/word").unwrap().is_hook());
    assert!(!Endpoint::new("longer/underscored_word").unwrap().is_hook());
  }
}
