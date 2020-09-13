//! [![Crates.io](https://img.shields.io/crates/l/tenable?label=license)](https://crates.io/crates/tenable)
//! [![Crates.io](https://img.shields.io/crates/v/tenable?label=version)](https://crates.io/crates/tenable)
//! [![Crates.io](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/tenable)
//!
//! This is an API Abstraction for the [Tenable API](https://developer.tenable.com/reference).
//!
//! The API itself is far too big for one person to develop. That is the reason why this crate does not provide methods for all endpoints, but instead focuses on modularity and extensability. Instead of providing methods for all endpoints, this crate makes it as easy as possible for users to add their own endpoints and hopefully contribute them afterwards.
//!
//! # Usage
//!
//! Add this crate as a dependency to your `Cargo.toml`. Afterwards you can use it like this to execute api calls like fetching all assets:
//!
//! ## Sync
//!
//! ```rust,no_run
//! use std::convert::Infallible;
//! use reqwest::blocking::Client;
//! use tenable::{requests::AssetReq, Error, Response, Tenable};
//! use http::Request;
//!
//! pub fn request(req: Request<Vec<u8>>) -> Result<Response, Error<reqwest::Error>> {
//!     let (req, body) = req.into_parts();
//!     let res = Client::new()
//!         .request(req.method, &req.uri.to_string())
//!         .headers(req.headers)
//!         .body(body)
//!         .send()
//!         .map_err(Error::Request)?;
//!     Ok(Response {
//!         status: res.status(),
//!         body: res.bytes().map_err(Error::Request)?,
//!     })
//! }
//!
//! let tenable = Tenable::new(
//!     "0000000000000000000000000000000000000000000000000000000000000000",
//!     "0000000000000000000000000000000000000000000000000000000000000000",
//! );
//! let req = tenable.assets();
//! let _assets = Tenable::request(req, request).expect("Unable to list all assets");
//! ```
//!
//! # Async
//!
//! ```rust,no_run
//! # use tokio::runtime::Runtime;
//! use std::convert::Infallible;
//! use reqwest::Client;
//! use tenable::{requests::AssetReq, types::Assets, Error, Response, Tenable};
//! use http::Request;
//!
//! pub async fn request_async(req: Request<Vec<u8>>) -> Result<Response, Error<reqwest::Error>> {
//!    let (req, body) = req.into_parts();
//!    let res = Client::new()
//!        .request(req.method, &req.uri.to_string())
//!        .headers(req.headers)
//!        .body(body)
//!        .send()
//!        .await
//!        .map_err(Error::Request)?;
//!    Ok(Response {
//!        status: res.status(),
//!        body: res.bytes().await.map_err(Error::Request)?,
//!    })
//! }
//!
//! # let mut rt = Runtime::new().expect("Unable to create runtime");
//! # rt.block_on(async {
//! let tenable = Tenable::new(
//!     "0000000000000000000000000000000000000000000000000000000000000000",
//!     "0000000000000000000000000000000000000000000000000000000000000000",
//! );
//! let req = tenable.assets();
//! let _assets: Assets = Tenable::request_async(req, request_async).await
//!     .expect("Unable to list all assets");
//! # })
//! ```
//!
//! # Extending
//!
//! Extending the functionality is possible by creating a type that implements `HttpRequest`, which defines how a request looks like and how to handle the server response. The following shows how to do that using the `AssetsReq` type which handles the `/assets` endpoint:
//!
//! ```rust
//! use http::{header::HeaderValue, status::StatusCode, Method, Request};
//! use tenable::{
//!    types::Assets,
//!    Error, HttpRequest, Response, Tenable,
//! };
//! use std::fmt;
//!
//! #[derive(Clone, Debug)]
//! pub struct AssetsReq<'a> {
//!     pub tenable: &'a Tenable<'a>,
//! }
//!
//! impl<RE: fmt::Debug> HttpRequest<RE> for AssetsReq<'_> {
//!     // The final concret type returned on a successful call
//!     type Output = Assets;
//!
//!     #[inline]
//!     fn to_request(&self) -> Result<Request<Vec<u8>>, Error<RE>> {
//!         // Create a request...
//!         let req = Request::builder()
//!             // ...by specificing the endpoint...
//!             .uri(format!("{}/assets", self.tenable.uri))
//!             // ...the method...
//!             .method(Method::GET)
//!             // ...authorization...
//!             .header(
//!                 "X-ApiKeys",
//!                 HeaderValue::from_str(self.tenable.auth.as_ref())?,
//!             )
//!             // ...and more like required headers, form parameters, body...
//!             .header("Accept", HeaderValue::from_static("application/json"))
//!             .body(Vec::new())?;
//!         Ok(req)
//!     }
//!
//!     #[inline]
//!     fn from_response(&self, res: Response) -> Result<Self::Output, Error<RE>> {
//!         // Handles the server response
//!         match res.status {
//!             // When the call was successfull, continue with deserializing
//!             StatusCode::OK => Ok(serde_json::from_slice(&res.body)?),
//!             // Otherwise, check whether the server returned one of the known errors
//!             StatusCode::FORBIDDEN => Err(Error::InsufficientPermission),
//!             StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimitReached),
//!             // Every other error may be collected in catch all type
//!             code => Err(Error::UnexpectedStatusCode(code)),
//!         }
//!     }
//! }
//! ```
//!
//! To be able to directly use the type with the tenable struct, we can add a new trait and implement it for tenable
//!
//! ```rust
//! use tenable::{Tenable, types::AssetsReq};
//!
//! pub trait AssetReq {
//!     fn assets(&self) -> AssetsReq<'_>;
//! }
//!
//! impl AssetReq for Tenable<'_> {
//!     fn assets(&self) -> AssetsReq<'_> {
//!         AssetsReq { tenable: self }
//!     }
//! }
//! ```
//!
//! # License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.

#![forbid(unsafe_code)]
#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![warn(
    clippy::correctness,
    clippy::restriction,
    clippy::style,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::shadow_reuse,
    clippy::similar_names,
    clippy::else_if_without_else,
    clippy::multiple_crate_versions,
    clippy::module_name_repetitions,
    clippy::print_stdout,
    clippy::used_underscore_binding
)]

mod error;
pub mod requests;
pub mod types;

pub use error::Error;

use bytes::Bytes;
use http::{status::StatusCode, Request};
use std::{borrow::Cow, fmt, future::Future, time::Duration};

/// Tenable Client which allows requests against the tenable API
#[derive(Clone, Debug)]
pub struct Tenable<'a> {
    /// Authentication string
    pub auth: String,
    /// Uri to send requests against
    pub uri: Cow<'a, str>,
}

impl Tenable<'_> {
    /// Creates a new `Tenable` client with the given credentials
    ///
    /// # Arguments
    ///
    /// * `access_key`: Tenable User Access Key
    /// * `secret_key`: Tenable User Access Key
    ///
    /// # Example
    ///
    /// ```
    /// use tenable::Tenable;
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000"
    /// );
    /// ```
    #[must_use]
    #[inline]
    pub fn new(access_key: &str, secret_key: &str) -> Self {
        Tenable {
            auth: format!("accessKey={};secretKey={}", access_key, secret_key),
            uri: Cow::Borrowed("https://cloud.tenable.com"),
        }
    }

    /// Executes a synchronous http request using the given function
    ///
    /// # Arguments
    ///
    /// * `request`: Request to send. Use one of the functions in the `requests` module to create a request
    /// * `fun`: Function which implements sending synchronous requests.
    ///
    /// # Errors
    ///
    /// Fails in the following cases:
    ///
    /// * Unable to create a valid Request
    /// * Server responded with error code
    /// * Unable to deserialize the server response
    /// * Custom Errors returned by the function given as `fun` parameter
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable};
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets();
    /// let _assets = Tenable::request(req, |_| {
    ///     Result::<Response, Error<Infallible>>::Ok(todo!("Define a method to send http requests"))
    /// }).expect("Unable to list all assets");
    /// ```
    #[inline]
    #[allow(single_use_lifetimes)]
    pub fn request<'a, O, R, CR, RE, F>(request: CR, fun: F) -> Result<O, Error<RE>>
    where
        CR: Into<Cow<'a, R>>,
        R: 'a + HttpRequest<RE, Output = O>,
        RE: fmt::Debug,
        F: FnOnce(Request<Vec<u8>>) -> Result<Response, Error<RE>>,
    {
        let request = request.into();
        let req = request.to_request()?;
        let res = fun(req)?;
        request.from_response(res)
    }

    /// Executes a synchronous http request using the given function.
    /// Automatically backs off when a Rate Limit is hit
    ///
    /// # Arguments
    ///
    /// * `request`: Request to send. Use one of the functions in the `requests` module to create a request
    /// * `fun`: Function which implements sending synchronous requests.
    /// * `backoff_fun`: Function which waits for the given Duration
    ///
    /// # Errors
    ///
    /// Fails in the following cases:
    ///
    /// * Unable to create a valid Request
    /// * Server responded with error code
    /// * Unable to deserialize the server response
    /// * Custom Errors returned by the function given as `fun` parameter
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::{convert::Infallible, thread::sleep};
    /// use tenable::{requests::AssetReq, Error, Response, Tenable};
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets();
    /// let _assets = Tenable::request_with_backoff(req, |_| {
    ///     Result::<Response, Error<Infallible>>::Ok(todo!("Define a method to send http requests"))
    /// }, sleep).expect("Unable to list all assets");
    /// ```
    #[inline]
    #[allow(single_use_lifetimes)]
    pub fn request_with_backoff<'a, O, R, CR, RE, F, BF>(
        request: CR,
        fun: F,
        backoff_fun: BF,
    ) -> Result<O, Error<RE>>
    where
        CR: Into<Cow<'a, R>>,
        R: 'a + HttpRequest<RE, Output = O>,
        RE: fmt::Debug,
        F: Fn(Request<Vec<u8>>) -> Result<Response, Error<RE>>,
        BF: Fn(Duration) -> (),
    {
        let mut wait = Duration::from_millis(100);
        let request = request.into();
        loop {
            let req = request.to_request()?;
            let res = fun(req)?;
            #[allow(clippy::wildcard_enum_match_arm)]
            match request.from_response(res) {
                Err(Error::RateLimitReached) => {
                    backoff_fun(wait);
                    match wait.checked_add(Duration::from_millis(100)) {
                        Some(new_wait) => wait = new_wait,
                        None => return Err(Error::MaximumWaitTimeReached),
                    }
                }
                other => return other,
            }
        }
    }

    /// Executes an asynchronous http request using the given function
    ///
    /// # Arguments
    ///
    /// * `request`: Request to send. Use one of the functions in the `requests` module to create a request
    /// * `fun`: Function which implements sending asynchronous requests.
    ///
    /// # Errors
    ///
    /// Fails in the following cases:
    ///
    /// * Unable to create a valid Request
    /// * Server responded with error code
    /// * Unable to deserialize the server response
    /// * Custom Errors returned by the function given as `fun` parameter
    ///
    /// # Example
    ///
    /// ## `tokio`
    ///
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// use http::Request;
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable};
    /// async fn request(_req: Request<Vec<u8>>) -> Result::<Response, Error<Infallible>> { Ok(todo!("Define a method to send http requests")) }
    ///
    /// # let mut rt = Runtime::new().expect("Unable to create runtime");
    /// # rt.block_on(async {
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets();
    /// let _assets = Tenable::request_async(req, request).await
    ///     .expect("Unable to list all assets");
    /// # })
    /// ```
    ///
    /// ## `async_std`
    ///
    /// ```no_run
    /// # use async_std::task;
    /// use http::Request;
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable};
    /// async fn request(_req: Request<Vec<u8>>) -> Result::<Response, Error<Infallible>> { Ok(todo!("Define a method to send http requests")) }
    ///
    /// # task::block_on(async {
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets();
    /// let _assets = Tenable::request_async(req, request).await
    ///     .expect("Unable to list all assets");
    /// # })
    /// ```
    #[inline]
    #[allow(single_use_lifetimes, unused_lifetimes)]
    pub async fn request_async<'a, O, R, CR, RE, F, Fut>(
        request: CR,
        fun: F,
    ) -> Result<O, Error<RE>>
    where
        CR: Into<Cow<'a, R>>,
        R: 'a + HttpRequest<RE, Output = O>,
        RE: fmt::Debug,
        F: FnOnce(Request<Vec<u8>>) -> Fut,
        Fut: Future<Output = Result<Response, Error<RE>>>,
    {
        let request = request.into();
        let req = request.to_request()?;
        let res = fun(req).await?;
        request.from_response(res)
    }

    /// Executes an asynchronous http request using the given function.
    /// Automatically backs off when a Rate Limit is hit
    ///
    /// # Arguments
    ///
    /// * `request`: Request to send. Use one of the functions in the `requests` module to create a request
    /// * `fun`: Function which implements sending asynchronous requests.
    /// * `backoff_fun`: Function which waits for the given Duration
    ///
    /// # Errors
    ///
    /// Fails in the following cases:
    ///
    /// * Unable to create a valid Request
    /// * Server responded with error code
    /// * Unable to deserialize the server response
    /// * Custom Errors returned by the function given as `fun` parameter
    ///
    /// # Example
    ///
    /// ## `tokio`
    ///
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// use tokio::time::delay_for;
    /// use http::Request;
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable};
    /// async fn request(_req: Request<Vec<u8>>) -> Result::<Response, Error<Infallible>> { Ok(todo!("Define a method to send http requests")) }
    ///
    /// # let mut rt = Runtime::new().expect("Unable to create runtime");
    /// # rt.block_on(async {
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets();
    /// let _assets = Tenable::request_with_backoff_async(req, request, delay_for).await
    ///     .expect("Unable to list all assets");
    /// # })
    /// ```
    ///
    /// ## `async_std`
    ///
    /// ```no_run
    /// use async_std::task;
    /// use http::Request;
    /// use std::convert::Infallible;
    /// use tenable::{requests::AssetReq, Error, Response, Tenable};
    /// async fn request(_req: Request<Vec<u8>>) -> Result::<Response, Error<Infallible>> { Ok(todo!("Define a method to send http requests")) }
    ///
    /// # task::block_on(async {
    /// let tenable = Tenable::new(
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    ///     "0000000000000000000000000000000000000000000000000000000000000000",
    /// );
    /// let req = tenable.assets();
    /// let _assets = Tenable::request_with_backoff_async(req, request, task::sleep).await
    ///     .expect("Unable to list all assets");
    /// # })
    /// ```
    #[inline]
    #[allow(single_use_lifetimes, unused_lifetimes)]
    pub async fn request_with_backoff_async<'a, O, R, CR, RE, F, Fut, BF, FutBF>(
        request: CR,
        fun: F,
        backoff_fun: BF,
    ) -> Result<O, Error<RE>>
    where
        CR: Into<Cow<'a, R>>,
        R: 'a + HttpRequest<RE, Output = O>,
        RE: fmt::Debug,
        F: Fn(Request<Vec<u8>>) -> Fut,
        Fut: Future<Output = Result<Response, Error<RE>>>,
        BF: Fn(Duration) -> FutBF,
        FutBF: Future<Output = ()>,
    {
        let mut wait = Duration::from_millis(100);
        let request = request.into();
        loop {
            let req = request.to_request()?;
            let res = fun(req).await?;
            #[allow(clippy::wildcard_enum_match_arm)]
            match request.from_response(res) {
                Err(Error::RateLimitReached) => {
                    backoff_fun(wait).await;
                    match wait.checked_add(Duration::from_millis(100)) {
                        Some(new_wait) => wait = new_wait,
                        None => return Err(Error::MaximumWaitTimeReached),
                    }
                }
                other => return other,
            }
        }
    }
}

/// Server Response allowing further processing
#[derive(Clone, Debug)]
pub struct Response {
    /// The `StatusCode` returned by the Server
    pub status: StatusCode,
    /// The Server Body in bytes
    pub body: Bytes,
}

/// Generic Requests which provides information for further processing using
/// the HTTP function given by the user
pub trait HttpRequest<RE: fmt::Debug>: Clone {
    /// Type which is returned by the HTTP Endpoint
    type Output;

    /// Creates an HTTP Request which can be send using an Http Client
    ///
    /// # Errors
    /// Fails if it is not possible to create a valid Request
    fn to_request(&self) -> Result<Request<Vec<u8>>, Error<RE>>;

    /// Parses the Http Client Response to its concret Type.
    ///
    /// # Errors
    ///
    /// Fails in the following cases:
    ///
    /// * Server responded with error code
    /// * Unable to deserialize the server response
    fn from_response(&self, res: Response) -> Result<Self::Output, Error<RE>>;
}
