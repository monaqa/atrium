// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.repo.uploadBlob` namespace."]
#[doc = "`com.atproto.repo.uploadBlob`"]
#[doc = "Upload a new blob to be added to repo in a later request."]
#[async_trait::async_trait]
pub trait UploadBlob: crate::xrpc::XrpcClient {
    async fn upload_blob(&self, input: Vec<u8>) -> Result<Output, Box<dyn std::error::Error>> {
        let body = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::POST,
            "com.atproto.repo.uploadBlob",
            None,
            Some(input),
            Some(String::from("*/*")),
        )
        .await?;
        serde_json::from_slice(&body).map_err(|e| e.into())
    }
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub blob: crate::blob::BlobRef,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
