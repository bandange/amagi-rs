pub(crate) fn parse_cookie_entries(cookie: &str) -> Vec<(String, String)> {
    cookie
        .split(';')
        .filter_map(|pair| {
            let trimmed = pair.trim();
            let index = trimmed.find('=')?;
            Some((
                trimmed[..index].trim().to_owned(),
                trimmed[index + 1..].trim().to_owned(),
            ))
        })
        .collect()
}
