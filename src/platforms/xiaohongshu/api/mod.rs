//! Public Xiaohongshu request builders migrated from the TypeScript platform layer.

mod builder;
mod types;

pub use builder::{XiaohongshuApiUrls, create_xiaohongshu_api_urls};
pub use types::{
    XiaohongshuCommentsOptions, XiaohongshuHomeFeedOptions, XiaohongshuNoteDetailOptions,
    XiaohongshuRequestSpec, XiaohongshuSearchNotesOptions, XiaohongshuUserNotesOptions,
    XiaohongshuUserProfileOptions,
};
