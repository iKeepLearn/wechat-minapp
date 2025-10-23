use crate::{Result, constants};
use http::{HeaderValue, Method, Request, header};
use serde_json::{Map, Value};
use tracing::debug;
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

pub fn build_request(
    url: &str,
    method: Method,
    headers: Option<Value>,
    query: Option<Value>,
    body: Option<Value>,
) -> Result<Request<Vec<u8>>> {
    let mut req_url = Url::parse(url)?;
    let default_map = Map::new();

    if let Some(value) = query
        && value.is_object()
    {
        debug!("req query value:{:?}", value);

        value
            .as_object()
            .unwrap_or(&default_map)
            .iter()
            .for_each(|item| {
                debug!("req query item:{:?}", item);
                let value_str = match item.1 {
                    Value::String(s) => s,
                    _ => &value.to_string(),
                };
                req_url.query_pairs_mut().append_pair(item.0, value_str);
            });
    }

    let req_builder = Request::builder()
        .uri(req_url.to_string())
        .header(header::USER_AGENT, constants::HTTP_CLIENT_USER_AGENT)
        .method(method);

    let req_builder_with_headers = if let Some(headers_value) = headers
        && headers_value.is_object()
    {
        headers_value
            .as_object()
            .unwrap_or(&default_map)
            .iter()
            .fold(req_builder, |current_builder, (key, value)| {
                let value_str = value.as_str().unwrap_or("");
                let header_value =
                    HeaderValue::from_str(value_str).unwrap_or(HeaderValue::from_static(""));
                current_builder.header(key.as_str(), header_value)
            })
    } else {
        req_builder
    };

    if let Some(value) = body
        && value.is_object()
    {
        debug!("builder body {:?}", &value);
        let body = serde_json::to_vec(&value)?;
        Ok(req_builder_with_headers.body(body)?)
    } else {
        Ok(req_builder_with_headers.body(Vec::new())?)
    }
}
