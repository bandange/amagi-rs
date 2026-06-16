/// Kuaishou tasks exposed by the CLI runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KuaishouRunTask {
    /// Fetch one Kuaishou video work.
    VideoWork {
        /// Photo id of the target work.
        photo_id: String,
    },
    /// Fetch comments for one Kuaishou work.
    WorkComments {
        /// Photo id of the target work.
        photo_id: String,
    },
    /// Fetch the Kuaishou emoji catalog.
    EmojiList,
    /// Fetch the aggregated Kuaishou user profile page.
    UserProfile {
        /// Principal id of the target profile.
        principal_id: String,
    },
    /// Fetch paginated public works for one Kuaishou user.
    UserWorkList {
        /// Principal id of the target profile.
        principal_id: String,
        /// Optional pagination cursor.
        pcursor: Option<String>,
        /// Optional page size.
        count: Option<u32>,
    },
    /// Fetch the aggregated Kuaishou live room page.
    LiveRoomInfo {
        /// Principal id of the target live room.
        principal_id: String,
    },
}
