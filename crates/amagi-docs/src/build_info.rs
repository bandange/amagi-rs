use crate::models::Language;

const PROJECT_REPOSITORY_URL: &str = "https://github.com/bandange/amagi-rs";
const PROJECT_COMMIT_URL_PREFIX: &str = "https://github.com/bandange/amagi-rs/commit/";

pub(crate) const PROJECT_LICENSE_URL: &str =
    "https://github.com/bandange/amagi-rs/blob/main/LICENSE";

pub(crate) fn docs_source_href() -> String {
    match docs_build_git_sha() {
        Some(sha) => format!("{PROJECT_COMMIT_URL_PREFIX}{sha}"),
        None => PROJECT_REPOSITORY_URL.to_owned(),
    }
}

pub(crate) fn docs_source_link_label(language: Language) -> String {
    match docs_build_git_sha_short() {
        Some(short_sha) => format!("{} @ {short_sha}", language.repository_label()),
        None => language.repository_label().to_owned(),
    }
}

fn docs_build_git_sha() -> Option<&'static str> {
    let sha = option_env!("AMAGI_DOCS_GIT_SHA")?.trim();
    if sha.is_empty() { None } else { Some(sha) }
}

fn docs_build_git_sha_short() -> Option<&'static str> {
    docs_build_git_sha().map(|sha| if sha.len() > 7 { &sha[..7] } else { sha })
}
