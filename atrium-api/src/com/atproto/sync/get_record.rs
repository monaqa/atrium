// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.sync.getRecord` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub collection: String,
    ///An optional past commit CID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    ///The DID of the repo.
    pub did: String,
    pub rkey: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
