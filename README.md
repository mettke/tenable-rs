# tenable

[![Crates.io](https://img.shields.io/crates/l/tenable?label=license)](https://crates.io/crates/tenable)
[![Crates.io](https://img.shields.io/crates/v/tenable?label=version)](https://crates.io/crates/tenable)
[![Crates.io](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/tenable)

This is an API Abstraction for the [Tenable API](https://developer.tenable.com/reference).

The API itself is far too big for one person to develop. That is the reason why this crate does not provide methods for all endpoints, but instead focuses on modularity and extensability. Instead of providing methods for all endpoints, this crate makes it as easy as possible for users to add their own endpoints and hopefully contribute them afterwards.

## Usage

Add this crate as a dependency to your `Cargo.toml`. Afterwards you can use it like this to execute api calls like fetching all assets:

### Sync

```rust
use std::convert::Infallible;
use reqwest::blocking::Client;
use tenable::{requests::AssetReq, Error, Response, Tenable};
use http::Request;

pub fn request(req: Request<Vec<u8>>) -> Result<Response, Error<reqwest::Error>> {
    let (req, body) = req.into_parts();
    let res = Client::new()
        .request(req.method, &req.uri.to_string())
        .headers(req.headers)
        .body(body)
        .send()
        .map_err(Error::Request)?;
    Ok(Response {
        status: res.status(),
        body: res.bytes().map_err(Error::Request)?,
    })
}

let tenable = Tenable::new(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0000000000000000000000000000000000000000000000000000000000000000",
);
let req = tenable.assets();
let _assets = Tenable::request(req, request).expect("Unable to list all assets");
```

## Async

```rust
use std::convert::Infallible;
use reqwest::Client;
use tenable::{requests::AssetReq, types::Assets, Error, Response, Tenable};
use http::Request;

pub async fn request_async(req: Request<Vec<u8>>) -> Result<Response, Error<reqwest::Error>> {
   let (req, body) = req.into_parts();
   let res = Client::new()
       .request(req.method, &req.uri.to_string())
       .headers(req.headers)
       .body(body)
       .send()
       .await
       .map_err(Error::Request)?;
   Ok(Response {
       status: res.status(),
       body: res.bytes().await.map_err(Error::Request)?,
   })
}

let tenable = Tenable::new(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0000000000000000000000000000000000000000000000000000000000000000",
);
let req = tenable.assets();
let _assets: Assets = Tenable::request_async(req, request_async).await
    .expect("Unable to list all assets");
```

## Extending

Extending the functionality is possible by creating a type that implements `HttpRequest`, which defines how a request looks like and how to handle the server response. The following shows how to do that using the `AssetsReq` type which handles the `/assets` endpoint:

```rust
use http::{header::HeaderValue, status::StatusCode, Method, Request};
use tenable::{
   types::Assets,
   Error, HttpRequest, Response, Tenable,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct AssetsReq<'a> {
    pub tenable: &'a Tenable<'a>,
}

impl<RE: fmt::Debug> HttpRequest<RE> for AssetsReq<'_> {
    // The final concret type returned on a successful call
    type Output = Assets;

    #[inline]
    fn to_request(&self) -> Result<Request<Vec<u8>>, Error<RE>> {
        // Create a request...
        let req = Request::builder()
            // ...by specificing the endpoint...
            .uri(format!("{}/assets", self.tenable.uri))
            // ...the method...
            .method(Method::GET)
            // ...authorization...
            .header(
                "X-ApiKeys",
                HeaderValue::from_str(self.tenable.auth.as_ref())?,
            )
            // ...and more like required headers, form parameters, body...
            .header("Accept", HeaderValue::from_static("application/json"))
            .body(Vec::new())?;
        Ok(req)
    }

    #[inline]
    fn from_response(&self, res: Response) -> Result<Self::Output, Error<RE>> {
        // Handles the server response
        match res.status {
            // When the call was successfull, continue with deserializing
            StatusCode::OK => Ok(serde_json::from_slice(&res.body)?),
            // Otherwise, check whether the server returned one of the known errors
            StatusCode::FORBIDDEN => Err(Error::InsufficientPermission),
            StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimitReached),
            // Every other error may be collected in catch all type
            code => Err(Error::UnexpectedStatusCode(code)),
        }
    }
}
```

To be able to directly use the type with the tenable struct, we can add a new trait and implement it for tenable

```rust
use tenable::{Tenable, types::AssetsReq};

pub trait AssetReq {
    fn assets(&self) -> AssetsReq<'_>;
}

impl AssetReq for Tenable<'_> {
    fn assets(&self) -> AssetsReq<'_> {
        AssetsReq { tenable: self }
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

License: MIT OR Apache-2.0
