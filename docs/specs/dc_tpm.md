---
layout: default
title: TPM-resident Device Credential
parent: Specifications
---

## TPM-resident Device Credential

The [FIDO Device Onboard specification](https://fidoalliance.org/specs/FDO/FIDO-Device-Onboard-RD-v1.1-20211214/FIDO-device-onboard-spec-v1.1-rd-20211214.html#to1hellorvack-type-31) does not define the way a Device Credential is stored, but just gives a non-normative example as to how it could be stored.

This specification aims to define how to store the information for the Device Credential in a Trusted Platform Module 2.0, as [specified](https://trustedcomputinggroup.org/resource/tpm-library-specification/) by the Trusted Computing Group.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and
"OPTIONAL" in this document are to be interpreted as described in
RFC 2119.

This specification assumes knowledge of TPM features like Key Derivation, hierarchies, and NV Indexes.

### General description

The TPM will be asked to generate a primary key used to generate two further keys: the device signing key, and the HMAC key.
These keys will be persisted into the Non-Volatile memory of the TPM.

Additionally, a NVIndex will be written to contain the rest of the Device Credential information, so the Device GUID and rendezvous info.

This means this specification will use up three NVIndex slots on the TPM to save the required information (one for the general info, two for the sub-keys).

### Hierarchies

If the FDO ROE is executed in the platform firmware, the Platform TPM hierarchy is used.
In all other cases, the Owner hierarchy is used.

From now on, the chosen hierarchy will be referred to as "the FDO TPM hierarchy".

### Keys

#### Primary Key

The primary key will be created under the FDO TPM hierarchy as a Primary Object, with the following template:

| Parameter | Type | Content |
|---|---|---|
| type | TPMI_ALG_PUBLIC | TPM_ALG_ECC |
| nameAlg | TPMI_ALG_HASH | TPM_ALG_SHA256 |
| objectAttributes | TPMA_OBJECT | fixedTPM = 1
| | | stClear = 0|
| | | fixedParent = 1|
| | | sensitiveDataOrigin = 1 |
| | | userWithAuth = 1 |
| | | adminWithPolicy = 0 |
| | | noDA = 0 |
| | | encryptedDuplication = 0 |
| | | restricted = 1 |
| | | decrypt = 1 |
| | | sign = 0 |
| authPolicy | TPM2B_DIGEST |  |
|     size | UINT16 | 256 |
|     buffer | BYTE | All 0 |
| parameters | TPMS_ECC_PARMS |  |
|     symmetric->algorithm | TPMI_ALG_SYM_OBJECT | TPM_ALG_AES |
|     symmetric->keyBits | TPMI_AES_KEY_BITS | 128 |
|     symmetric->mode | TPMI_SYM_MODE | TPM_ALG_CFB |
|     symmetric->details |  | NULL |
|     scheme->scheme | TPMI_ALG_ASYM_SCHEME | TPM_ALG_NULL |
|     scheme->details |  | NULL |
|     curveID | TPMI_ECC_CURVE | TPM_ECC_NIST_P256 |
|     kdf | TPMT_KDF_SCHEME | NULL |
| unique | TPM2B_PUBLIC_KEY_ECC |  |
|     size | UINT16 | 256 |
|     buffer | BYTE | All 0 |

#### Signing key

The signing key will be created under the Primary Key, as an Ordinary Key.
Its type depends on the key to be used as the device key: either `SECP256R1` or `SECP384R1`.
It will be created as per the following template:

| Parameter | Type | Content |
|---|---|---|
| type | TPMI_ALG_PUBLIC | TPM_ALG_ECC |
| nameAlg | TPMI_ALG_HASH | TPM_ALG_SHA256 for `SECP256R1`, TPM_ALG_SHA384 for `SECP384R1` keys |
| objectAttributes | TPMA_OBJECT | fixedTPM = 1
| | | stClear = 0|
| | | fixedParent = 1|
| | | sensitiveDataOrigin = 1 |
| | | userWithAuth = 1 |
| | | adminWithPolicy = 0 |
| | | noDA = 0 |
| | | encryptedDuplication = 0 |
| | | restricted = 0 |
| | | decrypt = 0 |
| | | sign = 1 |
| authPolicy | TPM2B_DIGEST |  |
|     size | UINT16 | 256 |
|     buffer | BYTE | All 0 |
| parameters | TPMS_ECC_PARMS |  |
|     symmetric->algorithm | TPMI_ALG_SYM_OBJECT | TPM_ALG_AES |
|     symmetric->keyBits | TPMI_AES_KEY_BITS | 128 |
|     symmetric->mode | TPMI_SYM_MODE | TPM_ALG_CFB |
|     symmetric->details |  | NULL |
|     scheme->scheme | TPMI_ALG_ASYM_SCHEME | TPM_ALG_NULL |
|     scheme->details |  | NULL |
|     curveID | TPMI_ECC_CURVE | TPM_ECC_NIST_P256 |
|     kdf | TPMT_KDF_SCHEME | NULL |
| unique | TPM2B_PUBLIC_KEY_ECC |  |
|     size | UINT16 | 256 |
|     buffer | BYTE | All 0 |
