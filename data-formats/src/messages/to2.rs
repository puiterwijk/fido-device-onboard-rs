use serde::{Deserialize, Serialize};
use serde_tuple::Serialize_tuple;

use super::{ClientMessage, Message, ServerMessage};

use crate::{
    constants::MessageType,
    types::{COSESign, CipherSuite, Guid, HMac, KexSuite, Nonce, ServiceInfo, SigInfo},
};

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct HelloDevice {
    guid: Guid,
    nonce5: Nonce,
    kex_suite: KexSuite,
    cipher_suite: CipherSuite,
    a_signature_info: SigInfo,
}

impl HelloDevice {
    pub fn new(
        guid: Guid,
        nonce5: Nonce,
        kex_suite: KexSuite,
        cipher_suite: CipherSuite,
        a_signature_info: SigInfo,
    ) -> Self {
        HelloDevice {
            guid,
            nonce5,
            kex_suite,
            cipher_suite,
            a_signature_info,
        }
    }

    pub fn guid(&self) -> &Guid {
        &self.guid
    }

    pub fn nonce5(&self) -> &Nonce {
        &self.nonce5
    }

    pub fn kex_suite(&self) -> KexSuite {
        self.kex_suite
    }

    pub fn cipher_suite(&self) -> CipherSuite {
        self.cipher_suite
    }

    pub fn a_signature_info(&self) -> &SigInfo {
        &self.a_signature_info
    }
}

impl Message for HelloDevice {
    fn message_type() -> MessageType {
        MessageType::TO2HelloDevice
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ClientMessage for HelloDevice {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveOVHdr(COSESign);

impl ProveOVHdr {
    pub fn new(token: COSESign) -> Self {
        ProveOVHdr(token)
    }

    pub fn into_token(self) -> COSESign {
        self.0
    }
}

impl Message for ProveOVHdr {
    fn message_type() -> MessageType {
        MessageType::TO2ProveOVHdr
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ServerMessage for ProveOVHdr {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct GetOVNextEntry {
    entry_num: u8,
}

impl GetOVNextEntry {
    pub fn new(entry_num: u8) -> Self {
        GetOVNextEntry { entry_num }
    }

    pub fn entry_num(&self) -> u8 {
        self.entry_num
    }
}

impl Message for GetOVNextEntry {
    fn message_type() -> MessageType {
        MessageType::TO2GetOVNextEntry
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ClientMessage for GetOVNextEntry {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct OVNextEntry {
    entry_num: u16,
    entry: COSESign,
}

impl OVNextEntry {
    pub fn new(entry_num: u16, entry: COSESign) -> Self {
        OVNextEntry { entry_num, entry }
    }

    pub fn entry_num(&self) -> u16 {
        self.entry_num
    }

    pub fn entry(&self) -> &COSESign {
        &self.entry
    }
}

impl Message for OVNextEntry {
    fn message_type() -> MessageType {
        MessageType::TO2OVNextEntry
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ServerMessage for OVNextEntry {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveDevice(COSESign);

impl ProveDevice {
    pub fn new(token: COSESign) -> Self {
        ProveDevice(token)
    }

    pub fn into_token(self) -> COSESign {
        self.0
    }
}

impl Message for ProveDevice {
    fn message_type() -> MessageType {
        MessageType::TO2ProveDevice
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ClientMessage for ProveDevice {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupDevice(COSESign);

impl SetupDevice {
    pub fn new(token: COSESign) -> Self {
        SetupDevice(token)
    }

    pub fn into_token(self) -> COSESign {
        self.0
    }
}

impl Message for SetupDevice {
    fn message_type() -> MessageType {
        MessageType::TO2SetupDevice
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ServerMessage for SetupDevice {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct DeviceServiceInfoReady {
    replacement_hmac: Option<HMac>,
    max_owner_service_info_size: Option<u64>,
}

impl DeviceServiceInfoReady {
    pub fn new(replacement_hmac: Option<HMac>, max_owner_service_info_size: Option<u64>) -> Self {
        DeviceServiceInfoReady {
            replacement_hmac,
            max_owner_service_info_size,
        }
    }

    pub fn replacement_hmac(&self) -> Option<&HMac> {
        self.replacement_hmac.as_ref()
    }

    pub fn max_owner_service_info_size(&self) -> Option<u64> {
        self.max_owner_service_info_size
    }
}

impl Message for DeviceServiceInfoReady {
    fn message_type() -> MessageType {
        MessageType::TO2DeviceServiceInfoReady
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ClientMessage for DeviceServiceInfoReady {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct OwnerServiceInfoReady {
    max_device_service_info_size: Option<u64>,
}

impl OwnerServiceInfoReady {
    pub fn new(max_device_service_info_size: Option<u64>) -> Self {
        OwnerServiceInfoReady {
            max_device_service_info_size,
        }
    }

    pub fn max_device_service_info_size(&self) -> Option<u64> {
        self.max_device_service_info_size
    }
}

impl Message for OwnerServiceInfoReady {
    fn message_type() -> MessageType {
        MessageType::TO2OwnerServiceInfoReady
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ServerMessage for OwnerServiceInfoReady {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct DeviceServiceInfo {
    is_more_service_info: bool,
    service_info: ServiceInfo,
}

impl DeviceServiceInfo {
    pub fn new(is_more_service_info: bool, service_info: ServiceInfo) -> Self {
        DeviceServiceInfo {
            is_more_service_info,
            service_info,
        }
    }

    pub fn is_more_service_info(&self) -> bool {
        self.is_more_service_info
    }

    pub fn service_info(&self) -> &ServiceInfo {
        &self.service_info
    }
}

impl Message for DeviceServiceInfo {
    fn message_type() -> MessageType {
        MessageType::TO2DeviceServiceInfo
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ClientMessage for DeviceServiceInfo {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct OwnerServiceInfo {
    is_more_service_info: bool,
    is_done: bool,
    service_info: ServiceInfo,
}

impl OwnerServiceInfo {
    pub fn new(is_more_service_info: bool, is_done: bool, service_info: ServiceInfo) -> Self {
        OwnerServiceInfo {
            is_more_service_info,
            is_done,
            service_info,
        }
    }

    pub fn is_more_service_info(&self) -> bool {
        self.is_more_service_info
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn service_info(&self) -> &ServiceInfo {
        &self.service_info
    }
}

impl Message for OwnerServiceInfo {
    fn message_type() -> MessageType {
        MessageType::TO2OwnerServiceInfo
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ServerMessage for OwnerServiceInfo {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct Done {
    nonce6: Nonce,
}

impl Done {
    pub fn new(nonce6: Nonce) -> Self {
        Done { nonce6 }
    }

    pub fn nonce6(&self) -> &Nonce {
        &self.nonce6
    }
}

impl Message for Done {
    fn message_type() -> MessageType {
        MessageType::TO2Done
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ClientMessage for Done {}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct Done2 {
    nonce7: Nonce,
}

impl Done2 {
    pub fn new(nonce7: Nonce) -> Self {
        Done2 { nonce7 }
    }

    pub fn nonce7(&self) -> &Nonce {
        &self.nonce7
    }
}

impl Message for Done2 {
    fn message_type() -> MessageType {
        MessageType::TO2Done2
    }

    fn is_valid_previous_message(message_type: Option<MessageType>) -> bool {
        todo!()
    }
}

impl ServerMessage for Done2 {}
