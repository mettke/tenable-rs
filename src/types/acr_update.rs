use crate::{Error, HttpRequest, Response, Tenable};
use http::{header::HeaderValue, status::StatusCode, Method, Request};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// Request Object for the `acr_update` function
#[derive(Clone, Debug)]
pub struct AcrUpdate<'a> {
    /// Inner tenable Client
    pub tenable: &'a Tenable<'a>,
    /// `Acr`s to send to tenable
    pub acrs: Cow<'a, [Acr]>,
}

impl<RE: fmt::Debug> HttpRequest<RE> for AcrUpdate<'_> {
    type Output = ();

    #[inline]
    fn to_request(&self) -> Result<Request<Vec<u8>>, Error<RE>> {
        let payload = serde_json::to_vec(&self.acrs)?;
        let req = Request::builder()
            .uri(format!("{}/api/v2/assets/bulk-jobs/acr", self.tenable.uri,))
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
            code => return Err(Error::UnexpectedStatusCode(code)),
        }
        Ok(())
    }
}

impl<'a> From<AcrUpdate<'a>> for Cow<'a, AcrUpdate<'a>> {
    #[inline]
    fn from(req: AcrUpdate<'a>) -> Self {
        Cow::Owned(req)
    }
}

impl<'a> From<&'a AcrUpdate<'a>> for Cow<'a, AcrUpdate<'a>> {
    #[inline]
    fn from(req: &'a AcrUpdate<'a>) -> Self {
        Cow::Borrowed(req)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
/// Parameters to update the ACR for an asset.
/// TODO: Cow
pub struct Acr {
    /// The ACR score you want to assign to the asset. The ACR must be an integer from 1 to 10.
    #[serde(rename = "acr_score")]
    pub acr_score: u64,
    /// The reasons you are updating the ACR for the assets. Supported values include:\n\n - Business Critical\n - In Scope For Compliance\n - Existing Mitigation Control\n - Dev only \n - Key drivers does not match \n - Other\n\nThis parameter corresponds to the **Overwrite Reasoning** parameter when editing an ACR in the Tenable.io Lumin user interface. For more information, see [Edit an ACR](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminEditACR.htm).
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<AcrUpdateReason>>,
    /// Any notes you want to add to clarify the circumstances behind the update. This parameter corresponds to the **Note** parameter when editing an ACR in the Tenable.io Lumin user interface. For more information, see [Edit an ACR](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminEditACR.htm).
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The identifiers of the assets to update to the specified ACR. At least one asset object is required in this array.
    #[serde(rename = "asset")]
    pub asset: Vec<AcrAsset>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Copy)]
#[allow(missing_docs)]
pub enum AcrUpdateReason {
    #[serde(rename = "Business Critical")]
    BusinessCritical,
    #[serde(rename = "In Scope For Compliance")]
    InScopeForCompliance,
    #[serde(rename = "Existing Mitigation Control")]
    ExistingMitigationControl,
    #[serde(rename = "Dev only")]
    DevOnly,
    #[serde(rename = "Key drivers does not match")]
    KeyDriversDoesNotMatch,
    #[serde(rename = "Other")]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
/// Each object can contain a single instance of the properties described below. You can combine multiple instances of this object, each containing a different single property.
/// TODO: Cow
/// TODO: Switch to enum
pub struct AcrAsset {
    /// The UUID for a specific asset. Use this value as the unique key for the asset.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Fully-qualified domain names (FQDNs) associated with the asset or assets.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<Vec<String>>,
    /// MAC addresses associated with the asset or assets.
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// The NetBIOS name for the asset.
    #[serde(rename = "netbios_name", skip_serializing_if = "Option::is_none")]
    pub netbios_name: Option<String>,
    /// IPv4 addresses associated with the asset or assets.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// `AccessGroupsRules` blueprint
pub struct AccessGroupsRules {
    /// The type of asset rule. The asset rule type corresponds to the type of data you can specify in the `terms` parameter. For a complete list of supported rule types, use the [GET /access-groups/filters](ref:io-v1-access-groups-list-filters) endpoint.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The operator that specifies how Tenable.io matches the terms value to asset data.   Possible operators include:   - eq—Tenable.io matches the rule to assets based on an exact match of the specified term. Note: Tenable.io interprets the operator as `equals` for ipv4 rules that specify a single IP address, but interprets the operator as `contains` for ipv4 rules that specify an IP range or CIDR range.  - match—Tenable.io matches the rule to assets based a partial match of the specified term.  - starts—Tenable.io matches the rule to assets that start with the specified term.  - ends—Tenable.io matches the rule to assets that end with the specified term.  For a complete list of operators by rule type, use the [GET /access-groups/rules/filters](ref:io-v1-access-groups-list-rule-filters) endpoint.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// The values that Tenable.io uses to match an asset to the rule. A term must correspond to the rule type.  For example:  - If the rule type is `aws_account`, the term is an AWS account ID.  - If the rule type is `fqdn`, the term is a hostname or a fully-qualified domain name (FQDN).  - If the rule type is `ipv4`, the term is an individual IPv4 address, a range of IPv4 addresses (for example, 172.204.81.57-172.204.81.60), or a CIDR range (for example, 172.204.81.57/24).   For a complete list of supported values by rule type, use the [GET /access-groups/rules/filters](ref:io-v1-access-groups-list-rule-filters) endpoint.    If you specify multiple terms values, Tenable.io includes an asset in the access group if the asset's attributes match any of the terms in the rule. <br >You can specify up to 100,000 terms per asset rule.
    #[serde(rename = "terms", skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// `AccessGroupsPrincipals` blueprint
pub struct AccessGroupsPrincipals {
    /// (Required) The type of principal. Valid values include:  - user—Grants access to the user you specify.  - group—Grants access to all users assigned to the user group you specify.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The UUID of a user or user group. This parameter is required if the request omits the `principal_name` parameter.
    #[serde(rename = "principal_id", skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// The name of the user or user group. This parameter is required if the request omits the `principal_id` parameter. If a request includes both `principal_id` and `principal_name`, Tenable.io assigns the user or user group to the access group based on the `principal_id` parameter, and ignores the `principal_name` parameter in the request.
    #[serde(rename = "principal_name", skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
}
