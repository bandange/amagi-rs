use super::args::{
    BilibiliCommand, DouyinCommand, KuaishouCommand, RunTaskArgs, TwitterCommand,
    XiaohongshuCommand,
};
use crate::config::{
    BilibiliRunTask, DouyinRunTask, KuaishouRunTask, RunTask, TwitterRunTask, XiaohongshuRunTask,
};

pub(super) fn map_run_task(task: Option<RunTaskArgs>) -> RunTask {
    match task {
        None => RunTask::Ready,
        Some(RunTaskArgs::Bilibili(bilibili)) => map_bilibili_command(bilibili.command),
        Some(RunTaskArgs::Douyin(douyin)) => map_douyin_command(douyin.command),
        Some(RunTaskArgs::Kuaishou(kuaishou)) => map_kuaishou_command(kuaishou.command),
        Some(RunTaskArgs::Twitter(twitter)) => map_twitter_command(twitter.command),
        Some(RunTaskArgs::Xiaohongshu(xiaohongshu)) => map_xiaohongshu_command(xiaohongshu.command),
    }
}

fn map_bilibili_command(command: BilibiliCommand) -> RunTask {
    RunTask::Bilibili(match command {
        BilibiliCommand::VideoInfo { bvid } => BilibiliRunTask::VideoInfo { bvid },
        BilibiliCommand::VideoStream { aid, cid } => BilibiliRunTask::VideoStream { aid, cid },
        BilibiliCommand::VideoDanmaku { cid, segment_index } => {
            BilibiliRunTask::VideoDanmaku { cid, segment_index }
        }
        BilibiliCommand::Comments {
            oid,
            comment_type,
            number,
            mode,
        } => BilibiliRunTask::Comments {
            oid,
            comment_type,
            number,
            mode,
        },
        BilibiliCommand::CommentReplies {
            oid,
            root,
            comment_type,
            number,
        } => BilibiliRunTask::CommentReplies {
            oid,
            comment_type,
            root,
            number,
        },
        BilibiliCommand::UserCard { host_mid } => BilibiliRunTask::UserCard { host_mid },
        BilibiliCommand::UserDynamicList { host_mid } => {
            BilibiliRunTask::UserDynamicList { host_mid }
        }
        BilibiliCommand::UserSpaceInfo { host_mid } => BilibiliRunTask::UserSpaceInfo { host_mid },
        BilibiliCommand::UploaderTotalViews { host_mid } => {
            BilibiliRunTask::UploaderTotalViews { host_mid }
        }
        BilibiliCommand::DynamicDetail { dynamic_id } => {
            BilibiliRunTask::DynamicDetail { dynamic_id }
        }
        BilibiliCommand::DynamicCard { dynamic_id } => BilibiliRunTask::DynamicCard { dynamic_id },
        BilibiliCommand::BangumiInfo { bangumi_id } => BilibiliRunTask::BangumiInfo { bangumi_id },
        BilibiliCommand::BangumiStream { ep_id, cid } => {
            BilibiliRunTask::BangumiStream { ep_id, cid }
        }
        BilibiliCommand::LiveRoomInfo { room_id } => BilibiliRunTask::LiveRoomInfo { room_id },
        BilibiliCommand::LiveRoomInit { room_id } => BilibiliRunTask::LiveRoomInit { room_id },
        BilibiliCommand::LoginStatus => BilibiliRunTask::LoginStatus,
        BilibiliCommand::LoginQrcode => BilibiliRunTask::LoginQrcode,
        BilibiliCommand::QrcodeStatus { qrcode_key } => {
            BilibiliRunTask::QrcodeStatus { qrcode_key }
        }
        BilibiliCommand::EmojiList => BilibiliRunTask::EmojiList,
        BilibiliCommand::AvToBv { aid } => BilibiliRunTask::AvToBv { aid },
        BilibiliCommand::BvToAv { bvid } => BilibiliRunTask::BvToAv { bvid },
        BilibiliCommand::ArticleContent { article_id } => {
            BilibiliRunTask::ArticleContent { article_id }
        }
        BilibiliCommand::ArticleCards { ids } => BilibiliRunTask::ArticleCards { ids },
        BilibiliCommand::ArticleInfo { article_id } => BilibiliRunTask::ArticleInfo { article_id },
        BilibiliCommand::ArticleListInfo { list_id } => {
            BilibiliRunTask::ArticleListInfo { list_id }
        }
        BilibiliCommand::CaptchaFromVoucher { v_voucher, csrf } => {
            BilibiliRunTask::CaptchaFromVoucher { v_voucher, csrf }
        }
        BilibiliCommand::ValidateCaptcha {
            challenge,
            token,
            validate,
            seccode,
            csrf,
        } => BilibiliRunTask::ValidateCaptcha {
            challenge,
            token,
            validate,
            seccode,
            csrf,
        },
    })
}

fn map_douyin_command(command: DouyinCommand) -> RunTask {
    RunTask::Douyin(match command {
        DouyinCommand::ParseWork { aweme_id } => DouyinRunTask::ParseWork { aweme_id },
        DouyinCommand::VideoWork { aweme_id } => DouyinRunTask::VideoWork { aweme_id },
        DouyinCommand::ImageAlbumWork { aweme_id } => DouyinRunTask::ImageAlbumWork { aweme_id },
        DouyinCommand::SlidesWork { aweme_id } => DouyinRunTask::SlidesWork { aweme_id },
        DouyinCommand::TextWork { aweme_id } => DouyinRunTask::TextWork { aweme_id },
        DouyinCommand::WorkComments {
            aweme_id,
            number,
            cursor,
        } => DouyinRunTask::WorkComments {
            aweme_id,
            number,
            cursor,
        },
        DouyinCommand::CommentReplies {
            aweme_id,
            comment_id,
            number,
            cursor,
        } => DouyinRunTask::CommentReplies {
            aweme_id,
            comment_id,
            number,
            cursor,
        },
        DouyinCommand::UserProfile { sec_uid } => DouyinRunTask::UserProfile { sec_uid },
        DouyinCommand::UserVideoList {
            sec_uid,
            number,
            max_cursor,
        } => DouyinRunTask::UserVideoList {
            sec_uid,
            number,
            max_cursor,
        },
        DouyinCommand::UserFavoriteList {
            sec_uid,
            number,
            max_cursor,
        } => DouyinRunTask::UserFavoriteList {
            sec_uid,
            number,
            max_cursor,
        },
        DouyinCommand::UserRecommendList {
            sec_uid,
            number,
            max_cursor,
        } => DouyinRunTask::UserRecommendList {
            sec_uid,
            number,
            max_cursor,
        },
        DouyinCommand::Search {
            query,
            search_type,
            number,
            search_id,
        } => DouyinRunTask::Search {
            query,
            search_type,
            number,
            search_id,
        },
        DouyinCommand::SuggestWords { query } => DouyinRunTask::SuggestWords { query },
        DouyinCommand::MusicInfo { music_id } => DouyinRunTask::MusicInfo { music_id },
        DouyinCommand::LiveRoomInfo { room_id, web_rid } => {
            DouyinRunTask::LiveRoomInfo { room_id, web_rid }
        }
        DouyinCommand::LoginQrcode { verify_fp } => DouyinRunTask::LoginQrcode { verify_fp },
        DouyinCommand::EmojiList => DouyinRunTask::EmojiList,
        DouyinCommand::DynamicEmojiList => DouyinRunTask::DynamicEmojiList,
        DouyinCommand::DanmakuList {
            aweme_id,
            duration,
            start_time,
            end_time,
        } => DouyinRunTask::DanmakuList {
            aweme_id,
            duration,
            start_time,
            end_time,
        },
    })
}

fn map_kuaishou_command(command: KuaishouCommand) -> RunTask {
    RunTask::Kuaishou(match command {
        KuaishouCommand::VideoWork { photo_id } => KuaishouRunTask::VideoWork { photo_id },
        KuaishouCommand::WorkComments { photo_id } => KuaishouRunTask::WorkComments { photo_id },
        KuaishouCommand::EmojiList => KuaishouRunTask::EmojiList,
        KuaishouCommand::UserProfile { principal_id } => {
            KuaishouRunTask::UserProfile { principal_id }
        }
        KuaishouCommand::UserWorkList {
            principal_id,
            pcursor,
            count,
        } => KuaishouRunTask::UserWorkList {
            principal_id,
            pcursor,
            count,
        },
        KuaishouCommand::LiveRoomInfo { principal_id } => {
            KuaishouRunTask::LiveRoomInfo { principal_id }
        }
    })
}

fn map_xiaohongshu_command(command: XiaohongshuCommand) -> RunTask {
    RunTask::Xiaohongshu(match command {
        XiaohongshuCommand::HomeFeed {
            cursor_score,
            num,
            refresh_type,
            note_index,
            category,
            search_key,
        } => XiaohongshuRunTask::HomeFeed {
            cursor_score,
            num,
            refresh_type,
            note_index,
            category,
            search_key,
        },
        XiaohongshuCommand::NoteDetail {
            note_id,
            xsec_token,
        } => XiaohongshuRunTask::NoteDetail {
            note_id,
            xsec_token,
        },
        XiaohongshuCommand::NoteComments {
            note_id,
            xsec_token,
            cursor,
        } => XiaohongshuRunTask::NoteComments {
            note_id,
            xsec_token,
            cursor,
        },
        XiaohongshuCommand::UserProfile {
            user_id,
            xsec_token,
            xsec_source,
        } => XiaohongshuRunTask::UserProfile {
            user_id,
            xsec_token,
            xsec_source,
        },
        XiaohongshuCommand::UserNoteList {
            user_id,
            xsec_token,
            xsec_source,
            cursor,
            num,
        } => XiaohongshuRunTask::UserNoteList {
            user_id,
            xsec_token,
            xsec_source,
            cursor,
            num,
        },
        XiaohongshuCommand::EmojiList => XiaohongshuRunTask::EmojiList,
        XiaohongshuCommand::Search {
            keyword,
            page,
            page_size,
            sort,
            note_type,
        } => XiaohongshuRunTask::Search {
            keyword,
            page,
            page_size,
            sort,
            note_type,
        },
    })
}

fn map_twitter_command(command: TwitterCommand) -> RunTask {
    RunTask::Twitter(match command {
        TwitterCommand::SearchTweets {
            query,
            search_type,
            count,
            cursor,
        } => TwitterRunTask::SearchTweets {
            query,
            search_type,
            count,
            cursor,
        },
        TwitterCommand::UserProfile { screen_name } => TwitterRunTask::UserProfile { screen_name },
        TwitterCommand::UserTimeline {
            screen_name,
            count,
            cursor,
        } => TwitterRunTask::UserTimeline {
            screen_name,
            count,
            cursor,
        },
        TwitterCommand::UserReplies {
            screen_name,
            count,
            cursor,
        } => TwitterRunTask::UserReplies {
            screen_name,
            count,
            cursor,
        },
        TwitterCommand::UserMedia {
            screen_name,
            count,
            cursor,
        } => TwitterRunTask::UserMedia {
            screen_name,
            count,
            cursor,
        },
        TwitterCommand::UserFollowers {
            screen_name,
            count,
            cursor,
        } => TwitterRunTask::UserFollowers {
            screen_name,
            count,
            cursor,
        },
        TwitterCommand::UserFollowing {
            screen_name,
            count,
            cursor,
        } => TwitterRunTask::UserFollowing {
            screen_name,
            count,
            cursor,
        },
        TwitterCommand::UserLikes { count, cursor } => TwitterRunTask::UserLikes { count, cursor },
        TwitterCommand::UserBookmarks { count, cursor } => {
            TwitterRunTask::UserBookmarks { count, cursor }
        }
        TwitterCommand::UserFollowed { count, cursor } => {
            TwitterRunTask::UserFollowed { count, cursor }
        }
        TwitterCommand::UserRecommended { count, cursor } => {
            TwitterRunTask::UserRecommended { count, cursor }
        }
        TwitterCommand::SearchUsers {
            query,
            count,
            cursor,
        } => TwitterRunTask::SearchUsers {
            query,
            count,
            cursor,
        },
        TwitterCommand::TweetDetail { tweet_id } => TwitterRunTask::TweetDetail { tweet_id },
        TwitterCommand::TweetReplies {
            tweet_id,
            cursor,
            sort_by,
        } => TwitterRunTask::TweetReplies {
            tweet_id,
            cursor,
            sort_by,
        },
        TwitterCommand::TweetLikers {
            tweet_id,
            count,
            cursor,
        } => TwitterRunTask::TweetLikers {
            tweet_id,
            count,
            cursor,
        },
        TwitterCommand::TweetRetweeters {
            tweet_id,
            count,
            cursor,
        } => TwitterRunTask::TweetRetweeters {
            tweet_id,
            count,
            cursor,
        },
        TwitterCommand::SpaceDetail { space_id } => TwitterRunTask::SpaceDetail { space_id },
    })
}
