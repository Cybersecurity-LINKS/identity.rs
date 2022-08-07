// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Error, Result};
use async_trait::async_trait;
use identity_credential::validator::ValidatorDocument;
use identity_did::{
  did::{CoreDID, DID},
  document::{CoreDocument, Document},
};
#[async_trait]
/// A trait for resolving DID documents adhering to a given DID method.
pub trait ResolutionHandler {
  type D: for<'a> TryFrom<&'a str> + DID;
  type DOC: Document<D = Self::D>;

  /// Fetch the associated DID Document from the given DID.
  async fn resolve(&self, did: &Self::D) -> Result<Self::DOC>;

  /// The supported did method.
  fn method() -> String;

  /// Like [`Self::resolve`](Self::resolve), but operates with [`CoreDID`] and [`CoreDocument`] instead.
  async fn resolve_core(&self, did: &CoreDID) -> Result<CoreDocument>;
}

#[async_trait]
pub trait InteroperableResolver: private::Sealed {
  async fn resolve_validator(&self, did: &str) -> Result<Box<dyn ValidatorDocument>>;
  fn method(&self) -> String;
}

mod private {
  use super::InteroperableResolver;

  pub trait Sealed {}
  impl<T> Sealed for T where T: InteroperableResolver {}
}

#[async_trait]
impl<T> InteroperableResolver for T
where
  T: ResolutionHandler + Send + Sync,
  T::DOC: Send + Sync + 'static,
  T::D: Send + Sync + 'static,
{
  async fn resolve_validator(&self, did: &str) -> Result<Box<dyn ValidatorDocument>> {
    // TODO: Consider improving error handling.
    let parsed_did: <T as ResolutionHandler>::D = did.parse().map_err(|_| {
      Error::DIDSyntaxError(identity_did::did::DIDError::Other(
        "failed to convert DID during resolution",
      ))
    })?;

    let doc = self.resolve(&parsed_did).await?;

    Ok(Box::new(doc))
  }

  fn method(&self) -> String {
    <T as ResolutionHandler>::method()
  }
}
