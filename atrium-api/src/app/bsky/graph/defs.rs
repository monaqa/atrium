// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.graph.defs` namespace.
///A list of actors used for curation purposes such as list feeds or interaction gating.
pub struct Curatelist;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListItemView {
    pub subject: crate::app::bsky::actor::defs::ProfileView,
    pub uri: String,
}
pub type ListPurpose = String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub cid: String,
    pub creator: crate::app::bsky::actor::defs::ProfileView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    pub indexed_at: String,
    pub name: String,
    pub purpose: ListPurpose,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ListViewerState>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListViewBasic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub cid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<String>,
    pub name: String,
    pub purpose: ListPurpose,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ListViewerState>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListViewerState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
}
///A list of actors to apply an aggregate moderation action (mute/block) on.
pub struct Modlist;
