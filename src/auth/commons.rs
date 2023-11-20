use super::auth::PlexoAuthToken;
use cookie::Cookie;
use reqwest::header::HeaderMap;

pub const GITHUB_USER_API: &str = "https://api.github.com/user";
pub const COOKIE_SESSION_TOKEN_NAME: &str = "plexo-session-token";

pub fn get_token_from_headers(headers: &HeaderMap) -> Option<PlexoAuthToken> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| PlexoAuthToken(s.to_string())).ok())
}

pub fn get_token_from_cookie(headers: &HeaderMap) -> Option<PlexoAuthToken> {
    let raw_cookie = headers.get("Cookie").and_then(|c| c.to_str().ok())?;

    get_token_from_raw_cookie(raw_cookie)
}

pub fn get_token_from_raw_cookie(raw_cookie: &str) -> Option<PlexoAuthToken> {
    for cookie in Cookie::split_parse(raw_cookie) {
        let Ok(cookie) = cookie else {
            println!("Error parsing cookie");
            continue;
        };

        if cookie.name() == COOKIE_SESSION_TOKEN_NAME {
            return Some(PlexoAuthToken(cookie.value().to_string()));
        }
    }

    None
}
