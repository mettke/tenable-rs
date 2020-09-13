use http::Request;
use reqwest::{blocking::Client as BClient, Client as AClient};
use tenable::{Error, Response, Tenable};

pub const ACCESS_KEY: &str = "";
pub const SECRET_KEY: &str = "";
pub const ASSET_ID: &str = "";

pub fn get_tenable() -> Tenable<'static> {
    Tenable::new(ACCESS_KEY, SECRET_KEY)
}

pub fn request(req: Request<Vec<u8>>) -> Result<Response, Error<reqwest::Error>> {
    let (req, body) = req.into_parts();
    let res = BClient::new()
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

pub async fn request_async(req: Request<Vec<u8>>) -> Result<Response, Error<reqwest::Error>> {
    let (req, body) = req.into_parts();
    let res = AClient::new()
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
