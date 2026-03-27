use std::collections::BTreeMap;

use crate::catalog::{HttpMethod, Platform};

pub(super) fn platform_default_method(platform: Platform) -> HttpMethod {
    match platform {
        Platform::Kuaishou => HttpMethod::Post,
        Platform::Bilibili | Platform::Douyin | Platform::Twitter | Platform::Xiaohongshu => {
            HttpMethod::Get
        }
    }
}

pub(super) fn platform_default_headers(platform: Platform) -> BTreeMap<String, String> {
    let mut headers = BTreeMap::new();

    match platform {
        Platform::Douyin => {
            headers.insert("Accept".into(), "application/json, text/plain, */*".into());
            headers.insert("Accept-Encoding".into(), "gzip, deflate, br, zstd".into());
            headers.insert(
                "Accept-Language".into(),
                "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".into(),
            );
            headers.insert("Priority".into(), "u=1, i".into());
            headers.insert("Referer".into(), "https://www.douyin.com/".into());
            headers.insert(
                "Sec-Ch-Ua".into(),
                "\"Not_A Brand\";v=\"99\", \"Chromium\";v=\"125\", \"Google Chrome\";v=\"125\""
                    .into(),
            );
            headers.insert("Sec-Ch-Ua-Mobile".into(), "?0".into());
            headers.insert("Sec-Ch-Ua-Platform".into(), "\"Windows\"".into());
            headers.insert("Sec-Fetch-Dest".into(), "empty".into());
            headers.insert("Sec-Fetch-Mode".into(), "cors".into());
            headers.insert("Sec-Fetch-Site".into(), "same-origin".into());
            headers.insert(
                "User-Agent".into(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
                    .into(),
            );
        }
        Platform::Bilibili => {
            headers.insert("Accept".into(), "application/json, text/plain, */*".into());
            headers.insert("Accept-Encoding".into(), "gzip, deflate, br, zstd".into());
            headers.insert("Accept-Language".into(), "zh-CN,zh;q=0.9".into());
            headers.insert("Cache-Control".into(), "no-cache".into());
            headers.insert("Pragma".into(), "no-cache".into());
            headers.insert("Priority".into(), "u=1, i".into());
            headers.insert("Referer".into(), "https://www.bilibili.com/".into());
            headers.insert(
                "Sec-Ch-Ua".into(),
                "\"Not_A Brand\";v=\"99\", \"Chromium\";v=\"142\", \"Google Chrome\";v=\"142\""
                    .into(),
            );
            headers.insert("Sec-Ch-Ua-Mobile".into(), "?0".into());
            headers.insert("Sec-Ch-Ua-Platform".into(), "\"Windows\"".into());
            headers.insert("Sec-Fetch-Dest".into(), "empty".into());
            headers.insert("Sec-Fetch-Mode".into(), "cors".into());
            headers.insert("Sec-Fetch-Site".into(), "same-site".into());
            headers.insert(
                "User-Agent".into(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"
                    .into(),
            );
        }
        Platform::Kuaishou => {
            headers.insert("Accept".into(), "application/json, text/plain, */*".into());
            headers.insert("Accept-Encoding".into(), "gzip, deflate, br, zstd".into());
            headers.insert(
                "Accept-Language".into(),
                "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".into(),
            );
            headers.insert("Content-Type".into(), "application/json".into());
            headers.insert("Origin".into(), "https://www.kuaishou.com".into());
            headers.insert("Priority".into(), "u=0, i".into());
            headers.insert("Referer".into(), "https://www.kuaishou.com/new-reco".into());
            headers.insert(
                "User-Agent".into(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0"
                    .into(),
            );
        }
        Platform::Xiaohongshu => {
            headers.insert("accept".into(), "application/json, text/plain, */*".into());
            headers.insert(
                "accept-language".into(),
                "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".into(),
            );
            headers.insert("cache-control".into(), "no-cache".into());
            headers.insert(
                "content-type".into(),
                "application/json;charset=UTF-8".into(),
            );
            headers.insert("origin".into(), "https://www.xiaohongshu.com".into());
            headers.insert("pragma".into(), "no-cache".into());
            headers.insert("priority".into(), "u=1, i".into());
            headers.insert("referer".into(), "https://www.xiaohongshu.com/".into());
            headers.insert(
                "sec-ch-ua".into(),
                "\"Microsoft Edge\";v=\"141\", \"Not?A_Brand\";v=\"8\", \"Chromium\";v=\"141\""
                    .into(),
            );
            headers.insert("sec-ch-ua-mobile".into(), "?0".into());
            headers.insert("sec-ch-ua-platform".into(), "\"Windows\"".into());
            headers.insert("sec-fetch-dest".into(), "empty".into());
            headers.insert("sec-fetch-mode".into(), "cors".into());
            headers.insert("sec-fetch-site".into(), "same-site".into());
            headers.insert(
                "user-agent".into(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 Edg/141.0.0.0"
                    .into(),
            );
        }
        Platform::Twitter => {
            headers.insert("Accept".into(), "*/*".into());
            headers.insert("Accept-Encoding".into(), "gzip, deflate, br, zstd".into());
            headers.insert("Accept-Language".into(), "en-US,en;q=0.9".into());
            headers.insert("Origin".into(), "https://x.com".into());
            headers.insert("Referer".into(), "https://x.com/".into());
            headers.insert(
                "Sec-Ch-Ua".into(),
                "\"Not_A Brand\";v=\"99\", \"Chromium\";v=\"142\", \"Google Chrome\";v=\"142\""
                    .into(),
            );
            headers.insert("Sec-Ch-Ua-Mobile".into(), "?0".into());
            headers.insert("Sec-Ch-Ua-Platform".into(), "\"Windows\"".into());
            headers.insert("Sec-Fetch-Dest".into(), "empty".into());
            headers.insert("Sec-Fetch-Mode".into(), "cors".into());
            headers.insert("Sec-Fetch-Site".into(), "same-origin".into());
            headers.insert("x-twitter-active-user".into(), "yes".into());
            headers.insert("x-twitter-client-language".into(), "en".into());
            headers.insert(
                "User-Agent".into(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"
                    .into(),
            );
        }
    }

    headers
}
