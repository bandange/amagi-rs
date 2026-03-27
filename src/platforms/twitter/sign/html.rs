use crate::error::AppError;

const DEFAULT_ASSET_BASE_URL: &str = "https://abs.twimg.com/responsive-web/client-web/";
const ON_DEMAND_CHUNK_NAME: &str = "ondemand.s";

#[derive(Debug, Clone)]
pub(super) struct TwitterHomePage {
    html: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MigrationForm {
    pub(super) action: String,
    pub(super) method: String,
    pub(super) fields: Vec<(String, String)>,
}

impl TwitterHomePage {
    pub(super) fn new(html: String) -> Self {
        Self { html }
    }

    pub(super) fn site_verification_key(&self) -> Result<String, AppError> {
        let tag = extract_tag_around(&self.html, "twitter-site-verification").ok_or_else(|| {
            AppError::UpstreamResponse {
                status: None,
                message: "twitter homepage did not contain `twitter-site-verification`".into(),
            }
        })?;

        extract_attribute_value(tag, "content").ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: "twitter homepage `twitter-site-verification` tag did not contain `content`"
                .into(),
        })
    }

    pub(super) fn resolve_ondemand_file_url(&self) -> Result<String, AppError> {
        let mut cursor = 0usize;
        while let Some(relative_start) = self.html[cursor..].find("<script") {
            let tag_start = cursor + relative_start;
            let Some(open_end) = self.html[tag_start..].find('>') else {
                break;
            };
            let content_start = tag_start + open_end + 1;
            let Some(relative_close) = self.html[content_start..].find("</script>") else {
                break;
            };
            let content_end = content_start + relative_close;
            let script = &self.html[content_start..content_end];

            if script.contains(ON_DEMAND_CHUNK_NAME)
                && let Some(url) = resolve_ondemand_file_url_from_runtime(script)
            {
                return Ok(url);
            }

            cursor = content_end + "</script>".len();
        }

        resolve_ondemand_file_url_from_runtime(&self.html).ok_or_else(|| {
            AppError::UpstreamResponse {
                status: None,
                message: "twitter homepage runtime did not expose the current ondemand chunk url"
                    .into(),
            }
        })
    }

    pub(super) fn frame_paths(&self) -> Result<Vec<String>, AppError> {
        let mut frames = Vec::new();
        let mut cursor = 0usize;

        while let Some(relative_anchor) = self.html[cursor..].find("loading-x-anim-") {
            let anchor = cursor + relative_anchor;
            let Some(svg_start) = self.html[..anchor].rfind("<svg") else {
                break;
            };
            let Some(relative_svg_end) = self.html[anchor..].find("</svg>") else {
                break;
            };
            let svg_end = anchor + relative_svg_end + "</svg>".len();
            let svg = &self.html[svg_start..svg_end];
            let mut path_values = extract_path_d_values(svg);
            if path_values.len() >= 2 {
                frames.push(path_values.swap_remove(1));
            }
            cursor = svg_end;
        }

        if frames.len() < 4 {
            return Err(AppError::UpstreamResponse {
                status: None,
                message: format!(
                    "twitter homepage did not expose 4 loading animation frames, got {}",
                    frames.len()
                ),
            });
        }

        Ok(frames)
    }

    pub(super) fn migration_redirect_url(&self) -> Option<String> {
        extract_migration_redirect_url(&self.html)
    }

    pub(super) fn migration_form(&self) -> Option<MigrationForm> {
        extract_migration_form(&self.html)
    }
}

pub(super) fn extract_key_byte_indices(chunk: &str) -> Result<(usize, Vec<usize>), AppError> {
    let bytes = chunk.as_bytes();
    let mut indices = Vec::new();
    let mut cursor = 0usize;

    while cursor + 7 < bytes.len() {
        if bytes[cursor] == b'(' && is_word(bytes[cursor + 1]) && bytes[cursor + 2] == b'[' {
            let digits_start = cursor + 3;
            let mut digits_end = digits_start;
            while digits_end < bytes.len() && bytes[digits_end].is_ascii_digit() {
                digits_end += 1;
            }

            if digits_end > digits_start && digits_end < bytes.len() && bytes[digits_end] == b']' {
                let mut index = digits_end + 1;
                if index < bytes.len() && bytes[index] == b',' {
                    index += 1;
                    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                        index += 1;
                    }
                    if index + 1 < bytes.len() && bytes[index] == b'1' && bytes[index + 1] == b'6' {
                        index += 2;
                        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                            index += 1;
                        }
                        if index < bytes.len() && bytes[index] == b')' {
                            if let Ok(value) = chunk[digits_start..digits_end].parse::<usize>() {
                                indices.push(value);
                            }
                        }
                    }
                }
            }
        }

        cursor += 1;
    }

    if indices.is_empty() {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: "twitter ondemand chunk did not expose key byte indices".into(),
        });
    }

    Ok((indices[0], indices[1..].to_vec()))
}

fn resolve_ondemand_file_url_from_runtime(runtime: &str) -> Option<String> {
    let mut search_from = 0usize;

    while let Some(relative_pos) = runtime[search_from..].find(ON_DEMAND_CHUNK_NAME) {
        let position = search_from + relative_pos;
        if let Some(chunk_id) = extract_chunk_id_before(runtime, position)
            && let Some(hash) = extract_chunk_hash_after(runtime, position, &chunk_id)
        {
            let base_url = extract_js_string_after(runtime, "g.p=")
                .unwrap_or_else(|| DEFAULT_ASSET_BASE_URL.to_owned());
            let base_url = ensure_trailing_slash(&base_url);
            return Some(format!("{base_url}{ON_DEMAND_CHUNK_NAME}.{hash}a.js"));
        }
        search_from = position + ON_DEMAND_CHUNK_NAME.len();
    }

    None
}

fn extract_chunk_id_before(source: &str, position: usize) -> Option<String> {
    let prefix = &source[..position];
    let colon = prefix.rfind(':')?;
    let mut start = colon;
    while start > 0 && prefix.as_bytes()[start - 1].is_ascii_digit() {
        start -= 1;
    }
    (start < colon).then(|| prefix[start..colon].to_owned())
}

fn extract_chunk_hash_after(source: &str, position: usize, chunk_id: &str) -> Option<String> {
    for pattern in [format!("{chunk_id}:\""), format!("{chunk_id}:'")] {
        if let Some(relative_pos) = source[position..].find(&pattern) {
            let start = position + relative_pos + pattern.len();
            let quote = pattern.as_bytes()[pattern.len() - 1] as char;
            let end = source[start..].find(quote)? + start;
            return Some(source[start..end].to_owned());
        }
    }

    None
}

fn extract_js_string_after(source: &str, prefix: &str) -> Option<String> {
    let start = source.find(prefix)? + prefix.len();
    let remainder = source[start..].trim_start();
    let quote = remainder.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let value_start = start + (source[start..].len() - remainder.len()) + 1;
    let value_end = source[value_start..].find(quote)? + value_start;
    Some(source[value_start..value_end].to_owned())
}

fn ensure_trailing_slash(value: &str) -> String {
    if value.ends_with('/') {
        value.to_owned()
    } else {
        format!("{value}/")
    }
}

fn extract_path_d_values(svg: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut cursor = 0usize;

    while let Some(relative_start) = svg[cursor..].find("<path") {
        let tag_start = cursor + relative_start;
        let Some(relative_end) = svg[tag_start..].find('>') else {
            break;
        };
        let tag_end = tag_start + relative_end + 1;
        let tag = &svg[tag_start..tag_end];
        if let Some(value) = extract_attribute_value(tag, "d") {
            values.push(value);
        }
        cursor = tag_end;
    }

    values
}

fn extract_migration_redirect_url(source: &str) -> Option<String> {
    let mut search_from = 0usize;

    while let Some(relative_pos) = source[search_from..].find("http") {
        let start = search_from + relative_pos;
        let remainder = &source[start..];
        let end = remainder
            .find(|ch: char| {
                ch == '"' || ch == '\'' || ch == '<' || ch == '>' || ch.is_whitespace()
            })
            .unwrap_or(remainder.len());
        let candidate = decode_html_entities(&remainder[..end]);
        if (candidate.contains("x.com") || candidate.contains("twitter.com"))
            && candidate.contains("migrate")
            && candidate.contains("tok=")
        {
            return Some(candidate);
        }

        search_from = start + 4;
    }

    None
}

fn extract_migration_form(source: &str) -> Option<MigrationForm> {
    let mut cursor = 0usize;

    while let Some(relative_start) = source[cursor..].find("<form") {
        let tag_start = cursor + relative_start;
        let relative_tag_end = source[tag_start..].find('>')?;
        let tag_end = tag_start + relative_tag_end + 1;
        let tag = &source[tag_start..tag_end];
        let name = extract_attribute_value(tag, "name");
        let action = extract_attribute_value(tag, "action");

        let is_migration_form = name.as_deref() == Some("f")
            || action
                .as_deref()
                .is_some_and(|value| value.contains("/migrate"));

        if is_migration_form {
            let relative_close = source[tag_end..].find("</form>")?;
            let body = &source[tag_end..tag_end + relative_close];
            return Some(MigrationForm {
                action: action.unwrap_or_else(|| "https://x.com/x/migrate".to_owned()),
                method: extract_attribute_value(tag, "method").unwrap_or_else(|| "POST".to_owned()),
                fields: extract_input_fields(body),
            });
        }

        cursor = tag_end;
    }

    None
}

fn extract_input_fields(form_body: &str) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    let mut cursor = 0usize;

    while let Some(relative_start) = form_body[cursor..].find("<input") {
        let tag_start = cursor + relative_start;
        let Some(relative_end) = form_body[tag_start..].find('>') else {
            break;
        };
        let tag_end = tag_start + relative_end + 1;
        let tag = &form_body[tag_start..tag_end];
        if let (Some(name), Some(value)) = (
            extract_attribute_value(tag, "name"),
            extract_attribute_value(tag, "value"),
        ) {
            fields.push((name, value));
        }
        cursor = tag_end;
    }

    fields
}

fn extract_tag_around<'a>(source: &'a str, anchor: &str) -> Option<&'a str> {
    let anchor_pos = source.find(anchor)?;
    let tag_start = source[..anchor_pos].rfind('<')?;
    let tag_end = source[anchor_pos..].find('>')? + anchor_pos + 1;
    Some(&source[tag_start..tag_end])
}

fn extract_attribute_value(tag: &str, name: &str) -> Option<String> {
    let pattern = format!("{name}=");
    let mut search_from = 0usize;

    while let Some(relative_pos) = tag[search_from..].find(&pattern) {
        let position = search_from + relative_pos;
        if position > 0 {
            let prefix = tag.as_bytes()[position - 1];
            if !(prefix.is_ascii_whitespace() || prefix == b'<' || prefix == b'/') {
                search_from = position + pattern.len();
                continue;
            }
        }

        let value_start = position + pattern.len();
        let remainder = &tag[value_start..];
        let first = remainder.chars().next()?;
        if first == '"' || first == '\'' {
            let end = remainder[1..].find(first)? + value_start + 1;
            return Some(decode_html_entities(&tag[value_start + 1..end]));
        }

        let end = remainder
            .find(|ch: char| ch.is_whitespace() || ch == '>' || ch == '/')
            .unwrap_or(remainder.len());
        return Some(decode_html_entities(&remainder[..end]));
    }

    None
}

fn decode_html_entities(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn is_word(value: u8) -> bool {
    value.is_ascii_alphanumeric() || value == b'_'
}
