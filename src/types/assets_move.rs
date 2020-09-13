use crate::{Error, HttpRequest, Response, Tenable};
use http::{header::HeaderValue, status::StatusCode, Method, Request};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// Request Object for the `assets_move` function
#[derive(Clone, Debug)]
pub struct AssetsMove<'a> {
    /// Inner tenable Client
    pub tenable: &'a Tenable<'a>,
    /// Definition which details the move operation
    pub assets_move_def: Cow<'a, AssetsMoveDef>,
}

impl<RE: fmt::Debug> HttpRequest<RE> for AssetsMove<'_> {
    type Output = Option<MovedAssets>;

    #[inline]
    fn to_request(&self) -> Result<Request<Vec<u8>>, Error<RE>> {
        let payload = serde_json::to_vec(&self.assets_move_def)?;
        let req = Request::builder()
            .uri(format!(
                "{}/api/v2/assets/bulk-jobs/move-to-network",
                self.tenable.uri
            ))
            .method(Method::POST)
            .header(
                "X-ApiKeys",
                HeaderValue::from_str(self.tenable.auth.as_ref())?,
            )
            .header("Accept", HeaderValue::from_static("application/json"))
            .body(payload)?;
        Ok(req)
    }

    #[inline]
    fn from_response(&self, res: Response) -> Result<Self::Output, Error<RE>> {
        match res.status {
            StatusCode::OK => {}
            StatusCode::FORBIDDEN => return Err(Error::InsufficientPermission),
            StatusCode::TOO_MANY_REQUESTS => return Err(Error::RateLimitReached),
            StatusCode::NOT_FOUND => return Ok(None),
            code => return Err(Error::UnexpectedStatusCode(code)),
        }
        let data = serde_json::from_slice(&res.body)?;
        Ok(Some(data))
    }
}

impl<'a> From<AssetsMove<'a>> for Cow<'a, AssetsMove<'a>> {
    #[inline]
    fn from(req: AssetsMove<'a>) -> Self {
        Cow::Owned(req)
    }
}

impl<'a> From<&'a AssetsMove<'a>> for Cow<'a, AssetsMove<'a>> {
    #[inline]
    fn from(req: &'a AssetsMove<'a>) -> Self {
        Cow::Borrowed(req)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Details what, from and where to move
/// TODO: Cow
pub struct AssetsMoveDef {
    /// The UUID of the network currently associated with the assets. Use the [GET /networks](ref:networks-list) endpoint with the name attribute as filter to find the UUID of the network.
    #[serde(rename = "source")]
    pub source: String,
    /// The UUID of the network to associate with the specified assets. Use the [GET /networks](ref:networks-list) endpoint with the name filter to find the UUID of the network.
    #[serde(rename = "destination")]
    pub destination: String,
    /// The IPv4 addresses of the assets to move. The addresses can be represented as a comma-separated list, a range, or CIDR, for example `1.1.1.1, 2.2.2.2-2.2.2.200, 3.3.3.0/24`.
    #[serde(rename = "targets")]
    pub targets: String,
}

impl From<AssetsMoveDef> for Cow<'_, AssetsMoveDef> {
    #[inline]
    fn from(req: AssetsMoveDef) -> Self {
        Cow::Owned(req)
    }
}

impl<'a> From<&'a AssetsMoveDef> for Cow<'a, AssetsMoveDef> {
    #[inline]
    fn from(req: &'a AssetsMoveDef) -> Self {
        Cow::Borrowed(req)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Copy)]
/// Information about the move operation
pub struct MovedAssets {
    /// The number of assets affected by the operation.
    #[serde(rename = "asset_count", skip_serializing_if = "Option::is_none")]
    pub asset_count: Option<i32>,
}
