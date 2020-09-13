use crate::{
    types::{Acr, AcrUpdate, AssetByUuidReq, AssetsMove, AssetsMoveDef, AssetsReq},
    Tenable,
};
use std::borrow::Cow;

/// Provides methods for the `Asset` Type.
pub trait AssetReq {
    /// Lists up to 5,000 assets.  
    ///
    /// **Note:** You can use the `assets_export` endpoint to export data for all
    /// assets.
    ///
    /// # Permission
    ///
    /// Requires BASIC [16] user permissions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable, types::Assets};
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets();
    /// let _assets: Assets = Tenable::request(req, |_| {
    ///     Result::<Response, Error<Infallible>>::Ok(todo!("Define a method to send http requests"))
    /// })
    /// .expect("Unable to list assets");
    /// ```
    fn assets(&self) -> AssetsReq<'_>;

    /// Returns details of the specified asset.
    ///
    /// # Permission
    ///
    /// Requires BASIC [16] user permissions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable, types::AssetByUuid};
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.asset_by_uuid("00000000-0000-0000-0000-000000000000");
    /// let _asset: AssetByUuid = Tenable::request(req, |_| {
    ///     Result::<Response, Error<Infallible>>::Ok(todo!("Define a method to send http requests"))
    /// })
    /// .expect("Unable to fetch asset")
    /// .expect("Asset not found");
    /// ```
    fn asset_by_uuid<'a, I: Into<Cow<'a, str>>>(&'a self, asset_uuid: I) -> AssetByUuidReq<'a>;

    /// Overwrites the Tenable-provided Asset Criticality Rating (ACR) for the specified
    /// assets. Tenable assigns an ACR to each asset on your network to represent the
    /// asset's relative risk as an integer from 1 to 10. For more information about ACR,
    /// see Lumin metrics in the Tenable.io Vulnerability Management User Guide.
    ///
    /// You must have a Lumin license to update the ACR for assets in your organization.
    ///
    /// # Permission
    ///
    /// Requires ADMINISTRATOR [64] user permissions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, types::Acr, Error, Response, Tenable};
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.acr_update(vec![Acr {
    ///     acr_score: 0,
    ///     asset: Vec::new(),
    ///     ..Acr::default()
    /// }]);
    /// Tenable::request(req, |_| {
    ///     Result::<Response, Error<Infallible>>::Ok(todo!("Define a method to send http requests"))
    /// })
    /// .expect("Unable to update acr");
    /// ```
    fn acr_update<'a, I: Into<Cow<'a, [Acr]>>>(&'a self, acrs: I) -> AcrUpdate<'a>;

    /// Moves assets from the specified network to another network. You can use this endpoint to move
    /// assets from the default network to a user-defined network, from a user-defined network to the
    /// default network, and from one user-defined network to another user-defined network. This request
    /// creates an asynchronous job in Tenable.io.
    ///
    /// For information about the assets move workflow and payload examples, see Bulk Asset Operations.
    ///     
    /// Requires ADMINISTRATOR [64] user permissions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable, types::{AssetsMoveDef, MovedAssets}};
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets_move(AssetsMoveDef {
    ///     source: "00000000-0000-0000-0000-000000000000".into(),
    ///     destination: "00000000-0000-0000-0000-000000000001".into(),
    ///     targets: "172.204.81.57-172.204.81.69".into()
    /// });
    /// let _move_info: MovedAssets = Tenable::request(req, |_| {
    ///     Result::<Response, Error<Infallible>>::Ok(todo!("Define a method to send http requests"))
    /// })
    /// .expect("Unable to move assets")
    /// .expect("Networks not found");
    /// ```
    fn assets_move<'a, I: Into<Cow<'a, AssetsMoveDef>>>(
        &'a self,
        assets_move_def: I,
    ) -> AssetsMove<'a>;
}

impl AssetReq for Tenable<'_> {
    #[inline]
    fn assets(&self) -> AssetsReq<'_> {
        AssetsReq { tenable: self }
    }

    #[inline]
    fn asset_by_uuid<'a, I: Into<Cow<'a, str>>>(&'a self, asset_uuid: I) -> AssetByUuidReq<'a> {
        AssetByUuidReq {
            tenable: self,
            asset_uuid: asset_uuid.into(),
        }
    }

    #[inline]
    fn acr_update<'a, I: Into<Cow<'a, [Acr]>>>(&'a self, acrs: I) -> AcrUpdate<'a> {
        AcrUpdate {
            tenable: self,
            acrs: acrs.into(),
        }
    }

    #[inline]
    fn assets_move<'a, I: Into<Cow<'a, AssetsMoveDef>>>(
        &'a self,
        assets_move_def: I,
    ) -> AssetsMove<'a> {
        AssetsMove {
            tenable: self,
            assets_move_def: assets_move_def.into(),
        }
    }
}
