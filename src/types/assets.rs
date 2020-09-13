use crate::{
    types::{AcrDriver, ScanFrequency, Source},
    Error, HttpRequest, Response, Tenable,
};
use http::{header::HeaderValue, status::StatusCode, Method, Request};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// Request Object for the `assets` function
#[derive(Clone, Debug)]
pub struct AssetsReq<'a> {
    /// Inner tenable Client
    pub tenable: &'a Tenable<'a>,
}

impl<RE: fmt::Debug> HttpRequest<RE> for AssetsReq<'_> {
    type Output = Assets;

    #[inline]
    fn to_request(&self) -> Result<Request<Vec<u8>>, Error<RE>> {
        let req = Request::builder()
            .uri(format!("{}/assets", self.tenable.uri))
            .method(Method::GET)
            .header(
                "X-ApiKeys",
                HeaderValue::from_str(self.tenable.auth.as_ref())?,
            )
            .header("Accept", HeaderValue::from_static("application/json"))
            .body(Vec::new())?;
        Ok(req)
    }

    #[inline]
    fn from_response(&self, res: Response) -> Result<Self::Output, Error<RE>> {
        match res.status {
            StatusCode::OK => {}
            StatusCode::FORBIDDEN => return Err(Error::InsufficientPermission),
            StatusCode::TOO_MANY_REQUESTS => return Err(Error::RateLimitReached),
            code => return Err(Error::UnexpectedStatusCode(code)),
        }
        let data = serde_json::from_slice(&res.body)?;
        Ok(data)
    }
}

impl<'a> From<AssetsReq<'a>> for Cow<'a, AssetsReq<'a>> {
    #[inline]
    fn from(req: AssetsReq<'a>) -> Self {
        Cow::Owned(req)
    }
}

impl<'a> From<&'a AssetsReq<'a>> for Cow<'a, AssetsReq<'a>> {
    #[inline]
    fn from(req: &'a AssetsReq<'a>) -> Self {
        Cow::Borrowed(req)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// `Assets` blueprint
pub struct Assets {
    /// A list of assets with details.
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
    /// The total number of assets in your Tenable.io instance.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Represents the `Asset` returned by `assets`
pub struct Asset {
    /// The UUID of the asset. Use this value as the unique key for the asset.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A value specifying whether a Nessus agent scan detected the asset (`true`).
    #[serde(rename = "has_agent", skip_serializing_if = "Option::is_none")]
    pub has_agent: Option<bool>,
    /// The ISO timestamp of the scan that most recently detected the asset.
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// The IPv4 address, IPv6 address, or FQDN that the scanner last used to evaluate the asset.
    #[serde(rename = "last_scan_target", skip_serializing_if = "Option::is_none")]
    pub last_scan_target: Option<String>,
    /// The sources of the scans that identified the asset.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// The Asset Criticality Rating (ACR) for the asset. With Lumin, Tenable assigns an ACR to each asset on your network to represent the asset's relative risk as an integer from 1 to 10. For more information, see [Lumin Metrics](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminMetrics.htm) in the *Tenable.io Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license.
    #[serde(rename = "acr_score", skip_serializing_if = "Option::is_none")]
    pub acr_score: Option<i32>,
    /// The key drivers that Tenable uses to calculate an asset's Tenable-provided ACR. For more information, see [Lumin Metrics](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminMetrics.htm) in the *Tenable.io Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license.
    #[serde(rename = "acr_drivers", skip_serializing_if = "Option::is_none")]
    pub acr_drivers: Option<Vec<AcrDriver>>,
    /// The Asset Exposure Score (AES) for the asset. For more information, see [Lumin Metrics](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminMetrics.htm) in the *Tenable.io Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license.
    #[serde(rename = "exposure_score", skip_serializing_if = "Option::is_none")]
    pub exposure_score: Option<i32>,
    /// Information about how often scans ran against the asset during specified intervals. This attribute is only present if you have a Lumin license.
    #[serde(rename = "scan_frequency", skip_serializing_if = "Option::is_none")]
    pub scan_frequency: Option<Vec<ScanFrequency>>,
    /// A list of IPv4 addresses for the asset.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    /// A list of IPv6 addresses for the asset.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
    /// A list of fully-qualified domain names (FQDNs) for the asset.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<Vec<String>>,
    /// The NetBIOS name for the asset.
    #[serde(rename = "netbios_name", skip_serializing_if = "Option::is_none")]
    pub netbios_name: Option<Vec<String>>,
    /// The operating systems that scans have associated with the asset record.
    #[serde(rename = "operating_system", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<Vec<String>>,
    /// The names of any Nessus agents that scanned and identified the asset.
    #[serde(rename = "agent_name", skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<Vec<String>>,
    /// The name of the virtual machine instance in AWS EC2.
    #[serde(rename = "aws_ec2_name", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_name: Option<Vec<String>>,
    /// A list of MAC addresses for the asset.
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Vec<String>>,
}
