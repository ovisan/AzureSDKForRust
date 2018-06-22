use azure::core::errors::AzureError;
use azure::core::headers::REQUEST_ID;
use azure::core::RequestId;
use azure::storage::container::Container;
use chrono::{DateTime, FixedOffset};
use http::HeaderMap;
use hyper::header;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetPropertiesResponse {
    pub container: Container,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
}

impl GetPropertiesResponse {
    pub(crate) fn from_response(container_name: String, headers: &HeaderMap) -> Result<GetPropertiesResponse, AzureError> {
        let request_id = match headers.get(REQUEST_ID) {
            Some(request_id) => Uuid::parse_str(request_id.to_str()?)?,
            None => return Err(AzureError::MissingHeaderError(REQUEST_ID.to_owned())),
        };

        let date = match headers.get(header::DATE) {
            Some(date) => DateTime::parse_from_rfc2822(date.to_str()?)?,
            None => return Err(AzureError::MissingHeaderError(header::DATE.as_str().to_owned())),
        };

        let container = Container::from_response(container_name, headers)?;

        Ok(GetPropertiesResponse {
            container,
            request_id,
            date,
        })
    }
}
