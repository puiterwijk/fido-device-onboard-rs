use serde::{Deserialize, Serialize};
use serde_tuple::Serialize_tuple;

use super::{ClientMessage, Message, ServerMessage};
use crate::{
    constants::{KeyStorageType, MfgStringType, PublicKeyType},
    publickey::PublicKey,
    types::{COSESign, CipherSuite, KexSuite, KeyExchange, Nonce},
};

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct Connect {
    nonce_diun_1: Nonce,
    kex_suite: KexSuite,
    cipher_suite: CipherSuite,
    key_exchange: KeyExchange,
}

impl Connect {
    pub fn new(
        nonce_diun_1: Nonce,
        kex_suite: KexSuite,
        cipher_suite: CipherSuite,
        key_exchange: KeyExchange,
    ) -> Self {
        Connect {
            nonce_diun_1,
            kex_suite,
            cipher_suite,
            key_exchange,
        }
    }

    pub fn nonce_diun_1(&self) -> &Nonce {
        &self.nonce_diun_1
    }

    pub fn kex_suite(&self) -> &KexSuite {
        &self.kex_suite
    }

    pub fn cipher_suite(&self) -> &CipherSuite {
        &self.cipher_suite
    }

    pub fn key_exchange(&self) -> &KeyExchange {
        &self.key_exchange
    }
}

impl Message for Connect {
    fn message_type() -> u8 {
        210
    }
}

impl ClientMessage for Connect {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accept(COSESign);

impl Accept {
    pub fn new(token: COSESign) -> Self {
        Accept(token)
    }

    pub fn into_token(self) -> COSESign {
        self.0
    }
}

impl Message for Accept {
    fn message_type() -> u8 {
        211
    }
}

impl ServerMessage for Accept {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct AcceptPayload {
    nonce_diun_1: Nonce,
    key_exchange: KeyExchange,
}

impl AcceptPayload {
    pub fn new(nonce_diun_1: Nonce, key_exchange: KeyExchange) -> Self {
        AcceptPayload {
            nonce_diun_1,
            key_exchange,
        }
    }

    pub fn key_exchange(&self) -> &KeyExchange {
        &self.key_exchange
    }
}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct RequestKeyParameters {
    empty: bool,
}

impl RequestKeyParameters {
    pub fn new() -> Self {
        RequestKeyParameters { empty: false }
    }
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

impl ProvideKeyParameters {
    pub fn new(
        public_key_type: PublicKeyType,
        public_key_storage_allowed: Option<Vec<KeyStorageType>>,
        attestation_challenge: Option<Vec<u8>>,
    ) -> Self {
        ProvideKeyParameters {
            public_key_type,
            public_key_storage_allowed,
            attestation_challenge,
        }
    }

    pub fn public_key_type(&self) -> &PublicKeyType {
        &self.public_key_type
    }

    pub fn public_key_storage_allowed(&self) -> Option<&Vec<KeyStorageType>> {
        self.public_key_storage_allowed.as_ref()
    }

    pub fn attestation_challenge(&self) -> Option<&Vec<u8>> {
        self.attestation_challenge.as_ref()
    }
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

impl ProvideKey {
    pub fn new(
        public_key: PublicKey,
        public_key_storage: Option<KeyStorageType>,
        attestation_response: Option<Vec<u8>>,
    ) -> Self {
        ProvideKey {
            public_key,
            public_key_storage,
            attestation_response,
        }
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn public_key_storage(&self) -> Option<&KeyStorageType> {
        self.public_key_storage.as_ref()
    }

    pub fn attestation_response(&self) -> Option<&Vec<u8>> {
        self.attestation_response.as_ref()
    }
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

impl Done {
    pub fn new(mfg_string_type: MfgStringType) -> Self {
        Done { mfg_string_type }
    }

    pub fn mfg_string_type(&self) -> &MfgStringType {
        &self.mfg_string_type
    }
}

impl Message for Done {
    fn message_type() -> u8 {
        215
    }
}

impl ServerMessage for Done {}
