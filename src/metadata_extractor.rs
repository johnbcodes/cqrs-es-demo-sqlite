use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use std::collections::HashMap;
use std::convert::Infallible;

// This is a custom Axum extension that builds metadata from the inbound request.
pub struct MetadataExtractor(pub HashMap<String, String>);

const USER_AGENT_HDR: &str = "User-Agent";

impl<S> FromRequestParts<S> for MetadataExtractor
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Here we are including the current date/time, the uri that was called and the user-agent
        // in a HashMap that we will submit as metadata with the command.
        let odt = time::OffsetDateTime::now_utc();
        let time = odt
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap();
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), time);
        metadata.insert("uri".to_string(), parts.uri.to_string());
        if let Some(user_agent) = parts.headers.get(USER_AGENT_HDR) {
            if let Ok(value) = user_agent.to_str() {
                metadata.insert(USER_AGENT_HDR.to_string(), value.to_string());
            }
        }
        Ok(MetadataExtractor(metadata))
    }
}
