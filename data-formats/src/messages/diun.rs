use serde::Deserialize;
use serde_tuple::Serialize_tuple;

use super::{ClientMessage, Message, ServerMessage};
use crate::{
    publickey::PublicKey,
    constants::{KeyStorageType, MfgStringType, PublicKeyType},
    types::{COSESign, CipherSuite, KexSuite, Nonce, KeyExchange},
};

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct Connect {
    nonce_diun_1: Nonce,
    kex_suite: KexSuite,
    cipher_suite: CipherSuite,
    key_exchange: KeyExchange,
}

impl Message for Connect {
    fn message_type() -> u8 {
        210
    }
}

impl ClientMessage for Connect {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct Accept {
    pubkey: PublicKey,
    payload: COSESign,
}

impl Message for Accept {
    fn message_type() -> u8 {
        211
    }
}

impl ServerMessage for Accept {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct AcceptPayload {
    key_exchange: KeyExchange,
}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct RequestKeyParameters {
    empty: bool,
}

impl Message for RequestKeyParameters {
    fn message_type() -> u8 {
        212
    }
}

impl ClientMessage for RequestKeyParameters {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct ProvideKeyParameters {
    public_key_type: PublicKeyType,
    public_key_storage_allowed: Option<Vec<KeyStorageType>>,
    attestation_challenge: Option<Vec<u8>>,
}

impl Message for ProvideKeyParameters {
    fn message_type() -> u8 {
        213
    }
}

impl ServerMessage for ProvideKeyParameters {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct ProvideKey {
    public_key: PublicKey,
    public_key_storage: Option<KeyStorageType>,

    attestation_response: Option<Vec<u8>>,
}

impl Message for ProvideKey {
    fn message_type() -> u8 {
        214
    }
}

impl ClientMessage for ProvideKey {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct Done {
    mfg_string_type: MfgStringType,
}

impl Message for Done {
    fn message_type() -> u8 {
        215
    }
}

impl ServerMessage for Done {}
