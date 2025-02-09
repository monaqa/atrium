// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.server.describeServer` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub available_user_domains: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_code_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
}
