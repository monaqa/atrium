// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.server.defs` namespace."]
#[doc = "`com.atproto.server.defs#inviteCode`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InviteCode {
    pub available: i32,
    pub code: String,
    pub created_at: String,
    pub created_by: String,
    pub disabled: bool,
    pub for_account: String,
    pub uses: Vec<InviteCodeUse>,
}
#[doc = "`com.atproto.server.defs#inviteCodeUse`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InviteCodeUse {
    pub used_at: String,
    pub used_by: String,
}
