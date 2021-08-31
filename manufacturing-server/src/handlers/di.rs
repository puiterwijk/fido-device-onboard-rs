use crate::ManufacturingServiceUDT;

use fdo_data_formats::messages;

use fdo_http_wrapper::{
    server::{Error, SessionWithStore},
    EncryptionKeys,
};

pub(crate) async fn app_start(
    user_data: ManufacturingServiceUDT,
    mut ses_with_store: SessionWithStore,
    msg: messages::di::AppStart,
) -> Result<(messages::di::SetCredentials, SessionWithStore), warp::Rejection> {
    todo!()
}

pub(crate) async fn set_hmac(
    user_data: ManufacturingServiceUDT,
    mut ses_with_store: SessionWithStore,
    msg: messages::di::SetHMAC,
) -> Result<(messages::di::Done, SessionWithStore), warp::Rejection> {
    todo!()
}
