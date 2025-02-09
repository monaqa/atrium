// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.embed.images` namespace.
//!A set of images embedded in some other form of content.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub images: Vec<Image>,
}
///width:height represents an aspect ratio. It may be approximate, and may not correspond to absolute dimensions in any given unit.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AspectRatio {
    pub height: i32,
    pub width: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    pub image: crate::blob::BlobRef,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub images: Vec<ViewImage>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewImage {
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    pub fullsize: String,
    pub thumb: String,
}
