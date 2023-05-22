// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `app.bsky.feed.defs` namespace."]
#[doc = "`app.bsky.feed.defs#blockedPost`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockedPost {
    pub blocked: bool,
    pub uri: String,
}
#[doc = "`app.bsky.feed.defs#feedViewPost`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedViewPost {
    pub post: PostView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<FeedViewPostReasonEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
}
#[doc = "`app.bsky.feed.defs#generatorView`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub cid: String,
    pub creator: crate::app::bsky::actor::defs::ProfileView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    pub display_name: String,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<GeneratorViewerState>,
}
#[doc = "`app.bsky.feed.defs#generatorViewerState`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorViewerState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like: Option<String>,
}
#[doc = "`app.bsky.feed.defs#notFoundPost`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundPost {
    pub not_found: bool,
    pub uri: String,
}
#[doc = "`app.bsky.feed.defs#postView`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PostView {
    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub cid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<PostViewEmbedEnum>,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    pub record: crate::records::Record,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repost_count: Option<i32>,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}
#[doc = "`app.bsky.feed.defs#reasonRepost`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReasonRepost {
    pub by: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub indexed_at: String,
}
#[doc = "`app.bsky.feed.defs#replyRef`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReplyRef {
    pub parent: ReplyRefParentEnum,
    pub root: ReplyRefRootEnum,
}
#[doc = "`app.bsky.feed.defs#skeletonFeedPost`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonFeedPost {
    pub post: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<SkeletonFeedPostReasonEnum>,
}
#[doc = "`app.bsky.feed.defs#skeletonReasonRepost`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonReasonRepost {
    pub repost: String,
}
#[doc = "`app.bsky.feed.defs#threadViewPost`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ThreadViewPost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ThreadViewPostParentEnum>,
    pub post: PostView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<ThreadViewPostRepliesItem>>,
}
#[doc = "`app.bsky.feed.defs#viewerState`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewerState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repost: Option<String>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum FeedViewPostReasonEnum {
    #[serde(rename = "app.bsky.feed.defs#reasonRepost")]
    ReasonRepost(Box<ReasonRepost>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum PostViewEmbedEnum {
    #[serde(rename = "app.bsky.embed.images#view")]
    AppBskyEmbedImagesView(Box<crate::app::bsky::embed::images::View>),
    #[serde(rename = "app.bsky.embed.external#view")]
    AppBskyEmbedExternalView(Box<crate::app::bsky::embed::external::View>),
    #[serde(rename = "app.bsky.embed.record#view")]
    AppBskyEmbedRecordView(Box<crate::app::bsky::embed::record::View>),
    #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
    AppBskyEmbedRecordWithMediaView(Box<crate::app::bsky::embed::record_with_media::View>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ReplyRefParentEnum {
    #[serde(rename = "app.bsky.feed.defs#postView")]
    PostView(Box<PostView>),
    #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
    NotFoundPost(Box<NotFoundPost>),
    #[serde(rename = "app.bsky.feed.defs#blockedPost")]
    BlockedPost(Box<BlockedPost>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ReplyRefRootEnum {
    #[serde(rename = "app.bsky.feed.defs#postView")]
    PostView(Box<PostView>),
    #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
    NotFoundPost(Box<NotFoundPost>),
    #[serde(rename = "app.bsky.feed.defs#blockedPost")]
    BlockedPost(Box<BlockedPost>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum SkeletonFeedPostReasonEnum {
    #[serde(rename = "app.bsky.feed.defs#skeletonReasonRepost")]
    SkeletonReasonRepost(Box<SkeletonReasonRepost>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ThreadViewPostParentEnum {
    #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
    ThreadViewPost(Box<ThreadViewPost>),
    #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
    NotFoundPost(Box<NotFoundPost>),
    #[serde(rename = "app.bsky.feed.defs#blockedPost")]
    BlockedPost(Box<BlockedPost>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ThreadViewPostRepliesItem {
    #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
    ThreadViewPost(Box<ThreadViewPost>),
    #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
    NotFoundPost(Box<NotFoundPost>),
    #[serde(rename = "app.bsky.feed.defs#blockedPost")]
    BlockedPost(Box<BlockedPost>),
}
