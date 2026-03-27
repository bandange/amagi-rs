/// Xiaohongshu tasks exposed by the CLI runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XiaohongshuRunTask {
    /// Fetch the Xiaohongshu home feed.
    HomeFeed {
        /// Optional cursor score.
        cursor_score: Option<String>,
        /// Optional page size.
        num: Option<u32>,
        /// Optional refresh type.
        refresh_type: Option<u32>,
        /// Optional note index.
        note_index: Option<u32>,
        /// Optional feed category.
        category: Option<String>,
        /// Optional feed search key.
        search_key: Option<String>,
    },
    /// Fetch one Xiaohongshu note detail payload.
    NoteDetail {
        /// Target note id.
        note_id: String,
        /// Required xsec token.
        xsec_token: String,
    },
    /// Fetch one page of Xiaohongshu note comments.
    NoteComments {
        /// Target note id.
        note_id: String,
        /// Required xsec token.
        xsec_token: String,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one Xiaohongshu user profile.
    UserProfile {
        /// Target user id.
        user_id: String,
        /// Required xsec token.
        xsec_token: String,
        /// Optional xsec source.
        xsec_source: Option<String>,
    },
    /// Fetch one page of Xiaohongshu user notes.
    UserNoteList {
        /// Target user id.
        user_id: String,
        /// Required xsec token.
        xsec_token: String,
        /// Optional xsec source.
        xsec_source: Option<String>,
        /// Optional pagination cursor.
        cursor: Option<String>,
        /// Optional page size.
        num: Option<u32>,
    },
    /// Fetch the Xiaohongshu emoji catalog.
    EmojiList,
    /// Search Xiaohongshu notes.
    Search {
        /// Search keyword.
        keyword: String,
        /// Optional page number.
        page: Option<u32>,
        /// Optional page size.
        page_size: Option<u32>,
        /// Optional sort order.
        sort: Option<crate::platforms::xiaohongshu::XiaohongshuSearchSortType>,
        /// Optional note-type filter.
        note_type: Option<crate::platforms::xiaohongshu::XiaohongshuSearchNoteType>,
    },
}
