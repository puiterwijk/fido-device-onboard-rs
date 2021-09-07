use crate::{ManufacturingServiceUD, ManufacturingServiceUDT};

use fdo_data_formats::{
    constants::HeaderKeys,
    messages,
    types::{COSEHeaderMap, COSESign, KeyDeriveSide, KeyExchange, Nonce},
};

use fdo_http_wrapper::{
    server::{Error, SessionWithStore},
    EncryptionKeys,
};

fn fail_if_no_diun(user_data: &ManufacturingServiceUD) -> Result<(), warp::Rejection> {
    if user_data.diun_configuration.is_none() {
        todo!();
    }
    Ok(())
}

pub(crate) async fn connect(
    user_data: ManufacturingServiceUDT,
    mut ses_with_store: SessionWithStore,
    msg: messages::diun::Connect,
) -> Result<(messages::diun::Accept, SessionWithStore), warp::Rejection> {
    fail_if_no_diun(&user_data)?;

    let mut session = ses_with_store.session;

    let b_key_exchange = KeyExchange::new(*msg.kex_suite())
        .map_err(Error::from_error::<messages::diun::Connect, _>)?;

    let new_keys = b_key_exchange
        .derive_key(
            KeyDeriveSide::OwnerService,
            *msg.cipher_suite(),
            msg.key_exchange(),
        )
        .map_err(Error::from_error::<messages::diun::Connect, _>)?;
    let new_keys = EncryptionKeys::from_derived(*msg.cipher_suite(), new_keys);
    log::trace!("Got new keys, setting {:?}", new_keys);
    // Note to self: set the keys after getting RequestKeyParameters - we encrypt as of the next reply
    /*fdo_http_wrapper::server::set_encryption_keys::<messages::diun::Connect>(
        &mut session,
        new_keys,
    )?;*/

    let accept_payload = messages::diun::AcceptPayload::new(
        b_key_exchange
            .get_public()
            .map_err(Error::from_error::<messages::diun::Connect, _>)?,
    );

    let mut accept_protected_header = COSEHeaderMap::new();
    accept_protected_header
        .insert(HeaderKeys::CUPHNonce, msg.nonce_diun_1())
        .unwrap();
    let mut accept_unprotected_header = COSEHeaderMap::new();
    accept_unprotected_header
        .insert(
            HeaderKeys::CUPHOwnerPubKey,
            &user_data.diun_configuration.as_ref().unwrap().public_keys,
        )
        .unwrap();

    let accept_payload = COSESign::new_with_protected(
        &accept_payload,
        accept_protected_header,
        Some(accept_unprotected_header),
        &user_data.diun_configuration.as_ref().unwrap().key,
    )
    .map_err(Error::from_error::<messages::diun::Connect, _>)?;

    ses_with_store.session = session;

    Ok((messages::diun::Accept::new(accept_payload), ses_with_store))
}

pub(crate) async fn request_key_parameters(
    user_data: ManufacturingServiceUDT,
    mut ses_with_store: SessionWithStore,
    msg: messages::diun::RequestKeyParameters,
) -> Result<(messages::diun::ProvideKeyParameters, SessionWithStore), warp::Rejection> {
    fail_if_no_diun(&user_data)?;

    todo!()
}

pub(crate) async fn provide_key(
    user_data: ManufacturingServiceUDT,
    mut ses_with_store: SessionWithStore,
    msg: messages::diun::ProvideKey,
) -> Result<(messages::diun::Done, SessionWithStore), warp::Rejection> {
    fail_if_no_diun(&user_data)?;

    todo!()
}
