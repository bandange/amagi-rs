use crate::{
    error::AppError,
    platforms::xiaohongshu::{OrderedJson, generate_runtime_search_id},
};

use super::super::api::{
    XiaohongshuCommentsOptions, XiaohongshuHomeFeedOptions, XiaohongshuNoteDetailOptions,
    XiaohongshuRequestSpec, XiaohongshuSearchNotesOptions, XiaohongshuUserNotesOptions,
    XiaohongshuUserProfileOptions,
};

/// Host set used by the Xiaohongshu platform request builders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct XiaohongshuApiEndpoints {
    pub(crate) api_base_url: String,
    pub(crate) web_base_url: String,
}

impl Default for XiaohongshuApiEndpoints {
    fn default() -> Self {
        Self {
            api_base_url: "https://edith.xiaohongshu.com".into(),
            web_base_url: "https://www.xiaohongshu.com".into(),
        }
    }
}

/// Request builder mirroring the original TypeScript Xiaohongshu API layer.
#[derive(Debug, Clone)]
pub(crate) struct XiaohongshuRequestBuilder {
    endpoints: XiaohongshuApiEndpoints,
}

impl XiaohongshuRequestBuilder {
    pub(crate) fn new(endpoints: XiaohongshuApiEndpoints) -> Self {
        Self { endpoints }
    }

    pub(crate) fn home_feed(
        &self,
        options: &XiaohongshuHomeFeedOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        let api_path = "/api/sns/web/v1/homefeed";
        let body = OrderedJson::object(vec![
            (
                "cursor_score",
                OrderedJson::string(
                    options
                        .cursor_score
                        .clone()
                        .unwrap_or_else(|| "1.7599348899670024E9".to_owned()),
                ),
            ),
            ("num", OrderedJson::uint(options.num.unwrap_or(33).into())),
            (
                "refresh_type",
                OrderedJson::uint(options.refresh_type.unwrap_or(3).into()),
            ),
            (
                "note_index",
                OrderedJson::uint(options.note_index.unwrap_or(33).into()),
            ),
            (
                "category",
                OrderedJson::string(
                    options
                        .category
                        .clone()
                        .unwrap_or_else(|| "homefeed_recommend".to_owned()),
                ),
            ),
            (
                "search_key",
                OrderedJson::string(options.search_key.clone().unwrap_or_default()),
            ),
            (
                "image_formats",
                OrderedJson::Array(vec!["jpg".into(), "webp".into(), "avif".into()]),
            ),
        ]);

        Ok(XiaohongshuRequestSpec {
            api_path: api_path.to_owned(),
            url: join_base(&self.endpoints.api_base_url, api_path),
            params: None,
            body: Some(body),
        })
    }

    pub(crate) fn note_detail(
        &self,
        options: &XiaohongshuNoteDetailOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        let api_path = "/api/sns/web/v1/feed";
        let body = OrderedJson::object(vec![
            (
                "source_note_id",
                OrderedJson::string(options.note_id.clone()),
            ),
            (
                "image_formats",
                OrderedJson::Array(vec!["jpg".into(), "webp".into(), "avif".into()]),
            ),
            (
                "extra",
                OrderedJson::object(vec![("need_body_topic", OrderedJson::string("1"))]),
            ),
            ("xsec_source", OrderedJson::string("pc_feed")),
            (
                "xsec_token",
                OrderedJson::string(options.xsec_token.clone()),
            ),
        ]);

        Ok(XiaohongshuRequestSpec {
            api_path: api_path.to_owned(),
            url: join_base(&self.endpoints.api_base_url, api_path),
            params: None,
            body: Some(body),
        })
    }

    pub(crate) fn note_comments(
        &self,
        options: &XiaohongshuCommentsOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        let api_path = "/api/sns/web/v2/comment/page";
        let params = OrderedJson::object(vec![
            ("note_id", OrderedJson::string(options.note_id.clone())),
            (
                "cursor",
                OrderedJson::string(options.cursor.clone().unwrap_or_default()),
            ),
            ("image_formats", OrderedJson::string("jpg,webp,avif")),
            (
                "xsec_token",
                OrderedJson::string(options.xsec_token.clone()),
            ),
        ]);
        let url = super::super::utils::build_url(
            &join_base(&self.endpoints.api_base_url, api_path),
            Some(&params),
        )?;

        Ok(XiaohongshuRequestSpec {
            api_path: api_path.to_owned(),
            url,
            params: Some(params),
            body: None,
        })
    }

    pub(crate) fn user_profile(
        &self,
        options: &XiaohongshuUserProfileOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        let api_path = "/api/sns/web/v1/user/otherinfo";
        let mut params_entries = Vec::new();
        if !options.xsec_token.trim().is_empty() {
            params_entries.push((
                "xsec_token",
                OrderedJson::string(options.xsec_token.clone()),
            ));
        }
        if let Some(xsec_source) = resolve_user_page_xsec_source(options) {
            params_entries.push(("xsec_source", OrderedJson::string(xsec_source)));
        }
        let params = (!params_entries.is_empty()).then(|| OrderedJson::object(params_entries));
        let url = super::super::utils::build_url(
            &join_base(
                &self.endpoints.web_base_url,
                &format!("/user/profile/{}", options.user_id),
            ),
            params.as_ref(),
        )?;

        Ok(XiaohongshuRequestSpec {
            api_path: api_path.to_owned(),
            url,
            params,
            body: None,
        })
    }

    pub(crate) fn user_note_list(
        &self,
        options: &XiaohongshuUserNotesOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        let api_path = "/api/sns/web/v1/user_posted";
        let params = OrderedJson::object(vec![
            ("num", OrderedJson::uint(options.num.unwrap_or(30).into())),
            (
                "cursor",
                OrderedJson::string(options.cursor.clone().unwrap_or_default()),
            ),
            ("user_id", OrderedJson::string(options.user_id.clone())),
            ("image_formats", OrderedJson::string("jpg,webp,avif")),
            (
                "xsec_source",
                OrderedJson::string(
                    options
                        .xsec_source
                        .clone()
                        .unwrap_or_else(|| "pc_feed".to_owned()),
                ),
            ),
        ]);
        let url = super::super::utils::build_url(
            &join_base(&self.endpoints.api_base_url, api_path),
            Some(&params),
        )?;

        Ok(XiaohongshuRequestSpec {
            api_path: api_path.to_owned(),
            url,
            params: Some(params),
            body: None,
        })
    }

    pub(crate) fn emoji_list(&self) -> Result<XiaohongshuRequestSpec, AppError> {
        let api_path = "/api/im/redmoji/detail";
        Ok(XiaohongshuRequestSpec {
            api_path: api_path.to_owned(),
            url: join_base(&self.endpoints.api_base_url, api_path),
            params: None,
            body: None,
        })
    }

    pub(crate) fn search_notes(
        &self,
        options: &XiaohongshuSearchNotesOptions,
        search_id: Option<String>,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        let api_path = "/api/sns/web/v1/search/notes";
        let body = OrderedJson::object(vec![
            ("keyword", OrderedJson::string(options.keyword.clone())),
            ("page", OrderedJson::uint(options.page.unwrap_or(1).into())),
            (
                "page_size",
                OrderedJson::uint(options.page_size.unwrap_or(20).into()),
            ),
            (
                "sort",
                OrderedJson::string(options.sort.unwrap_or_default().as_api_str()),
            ),
            (
                "note_type",
                OrderedJson::uint(options.note_type.unwrap_or_default().as_api_value().into()),
            ),
            (
                "search_id",
                OrderedJson::string(search_id.unwrap_or_else(generate_runtime_search_id)),
            ),
            (
                "image_formats",
                OrderedJson::Array(vec!["jpg".into(), "webp".into(), "avif".into()]),
            ),
        ]);

        Ok(XiaohongshuRequestSpec {
            api_path: api_path.to_owned(),
            url: join_base(&self.endpoints.api_base_url, api_path),
            params: None,
            body: Some(body),
        })
    }
}

fn join_base(base_url: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

fn resolve_user_page_xsec_source(options: &XiaohongshuUserProfileOptions) -> Option<String> {
    if options.xsec_token.trim().is_empty() {
        return None;
    }

    Some(match options.xsec_source.as_deref() {
        Some(source) if !source.trim().is_empty() && source != "pc_feed" => source.to_owned(),
        _ => "pc_user".to_owned(),
    })
}
