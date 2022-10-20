// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::document::CoreDocument;
use identity_did::verification::MethodType;

use crate::create_method::CreateMethodBuilder;
use crate::BlobStorage;
use crate::KeyStorage;

pub struct IdentityUpdater<'updater> {
  pub document: &'updater mut CoreDocument,
}

impl<'updater> IdentityUpdater<'updater> {
  pub fn create_method<K, B>(&mut self) -> CreateMethodBuilder<'_, K, B>
  where
    K: KeyStorage,
    K::KeyType: TryFrom<MethodType>,
    <K::KeyType as TryFrom<MethodType>>::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    B: BlobStorage,
  {
    CreateMethodBuilder::new(self.document)
  }
}
