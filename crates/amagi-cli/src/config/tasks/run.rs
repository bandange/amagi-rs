/// Supported tasks for the `run` command.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum RunTask {
    /// Print readiness metadata without performing a fetch.
    #[default]
    Ready,
    /// Execute a Bilibili-specific CLI task.
    Bilibili(super::BilibiliRunTask),
    /// Execute a Douyin-specific CLI task.
    Douyin(super::DouyinRunTask),
    /// Execute a Kuaishou-specific CLI task.
    Kuaishou(super::KuaishouRunTask),
    /// Execute a Twitter/X-specific CLI task.
    Twitter(super::TwitterRunTask),
    /// Execute a Xiaohongshu-specific CLI task.
    Xiaohongshu(super::XiaohongshuRunTask),
}
