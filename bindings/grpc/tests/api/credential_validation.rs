use _credentials::vc_validation_client::VcValidationClient;
use _credentials::VcValidationRequest;
use identity_iota::core::FromJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::Subject;
use identity_iota::did::DID;
use identity_storage::JwkDocumentExt;
use identity_storage::JwsSignatureOptions;
use identity_stronghold::StrongholdStorage;
use serde_json::json;

use crate::helpers::make_stronghold;
use crate::helpers::Entity;
use crate::helpers::TestServer;

mod _credentials {
  tonic::include_proto!("credentials");
}

#[tokio::test]
async fn credential_validation() -> anyhow::Result<()> {
  let stronghold = StrongholdStorage::new(make_stronghold());
  let server = TestServer::new_with_stronghold(stronghold.clone()).await;
  let api_client = server.client();

  let mut issuer = Entity::new_with_stronghold(stronghold);
  issuer.create_did(api_client).await?;

  let mut holder = Entity::new();
  holder.create_did(api_client).await?;

  let subject = Subject::from_json_value(json!({
    "id": holder.document().unwrap().id().as_str(),
    "name": "Alice",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer.document().unwrap().id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  let credential_jwt = issuer
    .document()
    .unwrap()
    .create_credential_jwt(
      &credential,
      &issuer.storage(),
      &issuer.fragment().unwrap(),
      &JwsSignatureOptions::default(),
      None,
    )
    .await?
    .into();

  let mut grpc_client = VcValidationClient::connect(server.endpoint()).await?;
  let decoded_cred = grpc_client
    .validate(VcValidationRequest { credential_jwt })
    .await?
    .into_inner()
    .credential_json;

  let decoded_cred = serde_json::from_str::<Credential>(&decoded_cred)?;
  assert_eq!(decoded_cred, credential);

  Ok(())
}
