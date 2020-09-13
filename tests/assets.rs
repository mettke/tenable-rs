mod common;

#[test]
fn assets() {
    use tenable::{requests::AssetReq, Tenable};

    let tenable = common::get_tenable();
    let req = tenable.assets();
    let _assets = Tenable::request(req, common::request).expect("Unable to list all assets");
}

#[test]
fn asset_by_uuid() {
    use std::borrow::Cow;
    use tenable::{requests::AssetReq, Tenable};

    let tenable = common::get_tenable();
    // test str
    let _req = tenable.asset_by_uuid(common::ASSET_ID);
    // test string
    let _req = tenable.asset_by_uuid(String::from(common::ASSET_ID));
    // test cow
    let req = tenable.asset_by_uuid(Cow::Borrowed(common::ASSET_ID));
    assert!(Tenable::request(req, common::request)
        .expect("Unable to get asset")
        .is_some());

    // not found
    let req = tenable.asset_by_uuid(Cow::Borrowed("notfound"));
    assert!(Tenable::request(req, common::request)
        .expect("Unable to get asset")
        .is_none());
}

#[test]
fn asset_by_uuid_with_backoff() {
    use std::{borrow::Cow, thread::sleep};
    use tenable::{requests::AssetReq, Tenable};

    let tenable = common::get_tenable();
    let req = tenable.asset_by_uuid(common::ASSET_ID);
    assert!(Tenable::request_with_backoff(req, common::request, sleep)
        .expect("Unable to get asset")
        .is_some());

    // not found
    let req = tenable.asset_by_uuid(Cow::Borrowed("notfound"));
    assert!(Tenable::request_with_backoff(req, common::request, sleep)
        .expect("Unable to get asset")
        .is_none());
}

#[test]
fn asset_by_uuid_async() {
    use std::borrow::Cow;
    use tenable::{requests::AssetReq, Tenable};
    use tokio::runtime::Runtime;

    let mut rt = Runtime::new().expect("Unable to create runtime");
    rt.block_on(async {
        let tenable = common::get_tenable();
        let req = tenable.asset_by_uuid(common::ASSET_ID);
        assert!(Tenable::request_async(req, common::request_async)
            .await
            .expect("Unable to get asset")
            .is_some());

        // not found
        let req = tenable.asset_by_uuid(Cow::Borrowed("notfound"));
        assert!(Tenable::request_async(req, common::request_async)
            .await
            .expect("Unable to get asset")
            .is_none());
    });
}

#[test]
fn asset_by_uuid_with_backoff_async() {
    use std::borrow::Cow;
    use tenable::{requests::AssetReq, Tenable};
    use tokio::{runtime::Runtime, time::delay_for};

    let mut rt = Runtime::new().expect("Unable to create runtime");
    rt.block_on(async {
        let tenable = common::get_tenable();
        let req = tenable.asset_by_uuid(common::ASSET_ID);
        assert!(
            Tenable::request_with_backoff_async(req, common::request_async, delay_for)
                .await
                .expect("Unable to get asset")
                .is_some()
        );

        // not found
        let req = tenable.asset_by_uuid(Cow::Borrowed("notfound"));
        assert!(
            Tenable::request_with_backoff_async(req, common::request_async, delay_for)
                .await
                .expect("Unable to get asset")
                .is_none()
        );
    });
}

#[test]
fn acr_update() {
    use std::borrow::Cow;
    use tenable::{requests::AssetReq, types::Acr, Error, Tenable};

    let borrow: &[Acr] = &[];

    let tenable = common::get_tenable();
    // test str
    let _req = tenable.acr_update(Vec::new());
    // test string
    let _req = tenable.acr_update(borrow);
    // test cow
    let req = tenable.acr_update(Cow::Borrowed(borrow));

    // TODO: handle permission
    // Tenable::request(req, common::request).expect("Unable to update acrs");
    match Tenable::request(req, common::request) {
        Err(Error::InsufficientPermission) => {}
        res => panic!("Unexpected response: {:?}", res),
    }
}

#[test]
fn assets_move() {
    use std::borrow::Cow;
    use tenable::{requests::AssetReq, types::AssetsMoveDef, Error, Tenable};

    let payload = AssetsMoveDef {
        source: "not found".into(),
        destination: "not found".into(),
        targets: "all the things".into(),
    };

    let tenable = common::get_tenable();
    // test str
    let _req = tenable.assets_move(&payload);
    // test cow
    let _req = tenable.assets_move(Cow::Borrowed(&payload));
    // test string
    let req = tenable.assets_move(payload);

    // TODO: handle permission
    // assert!(Tenable::request(req, common::request).expect("Unable to get asset").is_none());
    match Tenable::request(req, common::request) {
        Err(Error::InsufficientPermission) => {}
        res => panic!("Unexpected response: {:?}", res),
    }
}
