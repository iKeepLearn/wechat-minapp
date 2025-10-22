use crate::Result;
use url::Url;

pub fn parse_url(url: impl Into<String>) -> Result<Url> {
    Ok(Url::parse(&url.into())?)
}

pub fn parse_query(url: impl Into<String>) -> Result<Vec<(String, String)>> {
    let url = parse_url(url)?;
    let paris = url
        .query_pairs()
        .into_iter()
        .map(|s| (s.0.to_string(), s.1.to_string()))
        .collect();
    Ok(paris)
}
