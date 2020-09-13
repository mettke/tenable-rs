use crate::{
    types::{AcrDriver, ScanFrequency, Source},
    Error, HttpRequest, Response, Tenable,
};
use http::{header::HeaderValue, status::StatusCode, Method, Request};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// Request Object for the `asset_by_uuid` function
#[derive(Clone, Debug)]
pub struct AssetByUuidReq<'a> {
    /// Inner tenable Client
    pub tenable: &'a Tenable<'a>,
    /// UUID which identifies the asset
    pub asset_uuid: Cow<'a, str>,
}

impl<RE: fmt::Debug> HttpRequest<RE> for AssetByUuidReq<'_> {
    type Output = Option<AssetByUuid>;

    #[inline]
    fn to_request(&self) -> Result<Request<Vec<u8>>, Error<RE>> {
        let req = Request::builder()
            .uri(format!(
                "{}/assets/{asset_uuid}",
                self.tenable.uri,
                asset_uuid = self.asset_uuid
            ))
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
            StatusCode::NOT_FOUND => return Ok(None),
            code => return Err(Error::UnexpectedStatusCode(code)),
        }
        let data = serde_json::from_slice(&res.body)?;
        Ok(Some(data))
    }
}

impl<'a> From<AssetByUuidReq<'a>> for Cow<'a, AssetByUuidReq<'a>> {
    #[inline]
    fn from(req: AssetByUuidReq<'a>) -> Self {
        Cow::Owned(req)
    }
}

impl<'a> From<&'a AssetByUuidReq<'a>> for Cow<'a, AssetByUuidReq<'a>> {
    #[inline]
    fn from(req: &'a AssetByUuidReq<'a>) -> Self {
        Cow::Borrowed(req)
    }
}

/// Represents the `Asset` returned by `asset_by_uuid`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetByUuid {
    /// The UUID of the asset. Use this value as the unique key for the asset.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A value specifying whether a Nessus agent scan detected the asset.
    #[serde(rename = "has_agent", skip_serializing_if = "Option::is_none")]
    pub has_agent: Option<bool>,
    /// The time and date when Tenable.io created the asset record.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time and date when the asset record was last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The time and date when a scan first identified the asset.
    #[serde(rename = "first_seen", skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    /// The time and date of the scan that most recently identified the asset.
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// The IPv4 address, IPv6 address, or FQDN that the scanner last used to evaluate the asset.
    #[serde(rename = "last_scan_target", skip_serializing_if = "Option::is_none")]
    pub last_scan_target: Option<String>,
    /// The time and date of the last credentialed scan run on the asset.
    #[serde(
        rename = "last_authenticated_scan_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_authenticated_scan_date: Option<String>,
    /// The time and date of the last scan that identified the asset as licensed. Tenable.io categorizes an asset as licensed if a scan of that asset has returned results from a non-discovery plugin within the last 90 days.
    #[serde(
        rename = "last_licensed_scan_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_licensed_scan_date: Option<String>,
    /// The sources of the scans that identified the asset.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// Category tags assigned to the asset in Tenable.io.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tags>>,
    /// The Asset Criticality Rating (ACR) for the asset. Tenable assigns an ACR to each asset on your network to represent the asset's relative risk as an integer from 1 to 10. For more information, see [Lumin Metrics](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminMetrics.htm) in the *Tenable.io Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license.
    #[serde(rename = "acr_score", skip_serializing_if = "Option::is_none")]
    pub acr_score: Option<i32>,
    /// The key drivers that Tenable uses to calculate an asset's Tenable-provided ACR. For more information, see [Lumin Metrics](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminMetrics.htm) in the *Tenable.io Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license.
    #[serde(rename = "acr_drivers", skip_serializing_if = "Option::is_none")]
    pub acr_drivers: Option<Vec<AcrDriver>>,
    /// The Asset Exposure Score (AES) for the asset. For more information, see [Lumin Metrics](https://docs.tenable.com/tenableio/vulnerabilitymanagement/Content/Analysis/LuminMetrics.htm) in the *Tenable.io Vulnerability Management User Guide*.
    #[serde(rename = "exposure_score", skip_serializing_if = "Option::is_none")]
    pub exposure_score: Option<i32>,
    /// Information about how often scans ran against the asset during specified intervals.
    #[serde(rename = "scan_frequency", skip_serializing_if = "Option::is_none")]
    pub scan_frequency: Option<Vec<ScanFrequency>>,
    /// The ID of the network object to which the asset belongs. For more information, see [Manage Networks](doc:manage-networks-tio).
    #[serde(rename = "network_id", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<Vec<String>>,
    /// The IPv4 addresses that scans have associated with the asset record.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    /// The IPv6 addresses that scans have associated with the asset record.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
    /// The fully-qualified domain names that scans have associated with the asset record.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<Vec<String>>,
    /// The MAC addresses that scans have associated with the asset record.
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Vec<String>>,
    /// The NetBIOS names that scans have associated with the asset record.
    #[serde(rename = "netbios_name", skip_serializing_if = "Option::is_none")]
    pub netbios_name: Option<Vec<String>>,
    /// The operating systems that scans have associated with the asset record.
    #[serde(rename = "operating_system", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<Vec<String>>,
    /// The system types as reported by Plugin ID 54615. Possible values include `router`, `general-purpose`, `scan-host`, and `embedded`.
    #[serde(rename = "system_type", skip_serializing_if = "Option::is_none")]
    pub system_type: Option<Vec<String>>,
    /// The UUID of the agent present on the asset. This attribute is empty if no agent is present on the asset.
    #[serde(rename = "tenable_uuid", skip_serializing_if = "Option::is_none")]
    pub tenable_uuid: Option<Vec<String>>,
    /// The hostnames that scans have associated with the asset record.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Vec<String>>,
    /// The names of any Nessus agents that scanned and identified the asset.
    #[serde(rename = "agent_name", skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<Vec<String>>,
    /// The BIOS UUID that scans have associated with the asset.
    #[serde(rename = "bios_uuid", skip_serializing_if = "Option::is_none")]
    pub bios_uuid: Option<Vec<String>>,
    /// The unique identifier of the Linux instance in Amazon EC2. For more information, see the Amazon Elastic Compute Cloud Documentation.
    #[serde(
        rename = "aws_ec2_instance_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_ec2_instance_id: Option<Vec<String>>,
    /// The unique identifier of the Linux AMI image in Amazon Elastic Compute Cloud (Amazon EC2). For more information, see the Amazon Elastic Compute Cloud Documentation.
    #[serde(
        rename = "aws_ec2_instance_ami_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_ec2_instance_ami_id: Option<Vec<String>>,
    /// The canonical user identifier for the AWS account associated with the virtual machine instance. For example, `79a59df900b949e55d96a1e698fbacedfd6e09d98eacf8f8d5218e7cd47ef2be`. For more information, see AWS Account Identifiers in the AWS documentation.
    #[serde(rename = "aws_owner_id", skip_serializing_if = "Option::is_none")]
    pub aws_owner_id: Option<Vec<String>>,
    /// The availability zone where Amazon Web Services hosts the virtual machine instance, for example, `us-east-1a`. Availability zones are subdivisions of AWS regions. For more information, see Regions and Availability Zones in the AWS documentation.
    #[serde(
        rename = "aws_availability_zone",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_availability_zone: Option<Vec<String>>,
    /// The region where AWS hosts the virtual machine instance, for example, `us-east-1`. For more information, see Regions and Availability Zones in the AWS documentation.
    #[serde(rename = "aws_region", skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<Vec<String>>,
    /// The unique identifier for the public cloud that hosts the AWS virtual machine instance. For more information, see the Amazon Virtual Private Cloud User Guide.
    #[serde(rename = "aws_vpc_id", skip_serializing_if = "Option::is_none")]
    pub aws_vpc_id: Option<Vec<String>>,
    /// The virtual machine instance's group in AWS.
    #[serde(
        rename = "aws_ec2_instance_group_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_ec2_instance_group_name: Option<Vec<String>>,
    /// The state of the virtual machine instance in AWS at the time of the scan. For more information on instance states, see the AWS documentation.
    #[serde(
        rename = "aws_ec2_instance_state_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_ec2_instance_state_name: Option<Vec<String>>,
    /// The type of instance in AWS EC2.
    #[serde(
        rename = "aws_ec2_instance_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_ec2_instance_type: Option<Vec<String>>,
    /// The unique identifier of the AWS subnet where the virtual machine instance was running at the time of the scan.
    #[serde(rename = "aws_subnet_id", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_id: Option<Vec<String>>,
    /// The product code associated with the AMI used to launch the virtual machine instance in AWS EC2.
    #[serde(
        rename = "aws_ec2_product_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_ec2_product_code: Option<Vec<String>>,
    /// The name of the virtual machine instance in AWS EC2.
    #[serde(rename = "aws_ec2_name", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_name: Option<Vec<String>>,
    /// The unique identifier of the Microsoft Azure virtual machine instance. For more information, see \"Accessing and Using Azure VM Unique ID\" in the Microsoft Azure documentation.
    #[serde(rename = "azure_vm_id", skip_serializing_if = "Option::is_none")]
    pub azure_vm_id: Option<Vec<String>>,
    /// The unique identifier of the resource in the Azure Resource Manager. For more information, see the Azure Resource Manager Documentation.
    #[serde(rename = "azure_resource_id", skip_serializing_if = "Option::is_none")]
    pub azure_resource_id: Option<Vec<String>>,
    /// The customized name of the project to which the virtual machine instance belongs in Google Cloud Platform (GCP). For more information, see \"Creating and Managing Projects\" in the GCP documentation.
    #[serde(rename = "gcp_project_id", skip_serializing_if = "Option::is_none")]
    pub gcp_project_id: Option<Vec<String>>,
    /// The zone where the virtual machine instance runs in GCP. For more information, see \"Regions and Zones\" in the GCP documentation.
    #[serde(rename = "gcp_zone", skip_serializing_if = "Option::is_none")]
    pub gcp_zone: Option<Vec<String>>,
    /// The unique identifier of the virtual machine instance in GCP.
    #[serde(rename = "gcp_instance_id", skip_serializing_if = "Option::is_none")]
    pub gcp_instance_id: Option<Vec<String>>,
    /// The SSH key fingerprints that scans have associated with the asset record.
    #[serde(rename = "ssh_fingerprint", skip_serializing_if = "Option::is_none")]
    pub ssh_fingerprint: Option<Vec<String>>,
    /// The unique identifier of the asset in McAfee ePolicy Orchestrator (ePO). For more information, see the McAfee documentation.
    #[serde(rename = "mcafee_epo_guid", skip_serializing_if = "Option::is_none")]
    pub mcafee_epo_guid: Option<Vec<String>>,
    /// The unique identifier of the McAfee ePO agent that identified the asset. For more information, see the McAfee documentation.
    #[serde(
        rename = "mcafee_epo_agent_guid",
        skip_serializing_if = "Option::is_none"
    )]
    pub mcafee_epo_agent_guid: Option<Vec<String>>,
    /// The Asset ID of the asset in Qualys. For more information, see the Qualys documentation.
    #[serde(rename = "qualys_asset_id", skip_serializing_if = "Option::is_none")]
    pub qualys_asset_id: Option<Vec<String>>,
    /// The Host ID of the asset in Qualys. For more information, see the Qualys documentation.
    #[serde(rename = "qualys_host_id", skip_serializing_if = "Option::is_none")]
    pub qualys_host_id: Option<Vec<String>>,
    /// The unique record identifier of the asset in ServiceNow. For more information, see the ServiceNow documentation.
    #[serde(rename = "servicenow_sysid", skip_serializing_if = "Option::is_none")]
    pub servicenow_sysid: Option<Vec<String>>,
    /// A list of Common Platform Enumeration (CPE) values that represent software applications a scan identified as present on an asset. This attribute supports the CPE 2.2 format. For more information, see the \"Component Syntax\" section of the [CPE Specification, Version 2.2](https://cpe.mitre.org/files/cpe-specification_2.2.pdf). For assets identified in Tenable scans, this attribute contains data only if a scan using [Nessus Plugin ID 45590](https://www.tenable.com/plugins/nessus/45590) has evaluated the asset.  **Note:** If no scan detects an application within 30 days of the scan that originally detected the application, Tenable.io considers the detection of that application expired. As a result, the next time a scan evaluates the asset, Tenable.io removes the expired application from the installed_software attribute. This activity is logged as a `remove` type of `attribute_change` update in the asset activity log.
    #[serde(rename = "installed_software", skip_serializing_if = "Option::is_none")]
    pub installed_software: Option<Vec<String>>,
}

/// `Tags` blueprint
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    /// The UUID of the tag.
    #[serde(rename = "tag_uuid", skip_serializing_if = "Option::is_none")]
    pub tag_uuid: Option<String>,
    /// The tag category (the first half of the category:value pair).
    #[serde(rename = "tag_key", skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// The tag value (the second half of the category:value pair).
    #[serde(rename = "tag_value", skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// The UUID of the user who assigned the tag to the asset.
    #[serde(rename = "added_by", skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    /// The ISO timestamp when the tag was assigned to the asset.
    #[serde(rename = "added_at", skip_serializing_if = "Option::is_none")]
    pub added_at: Option<String>,
}
