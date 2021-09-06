use std::{convert::TryInto, env};

use anyhow::{anyhow, bail, Context, Result};

use fdo_data_formats::{constants::HeaderKeys, messages, publickey::{PublicKey, X5Chain}, types::{CipherSuite, Hash, KexSuite, KeyExchange, Nonce}};
use fdo_http_wrapper::client::{RequestResult, ServiceClient};

async fn perform_diun(client: &mut ServiceClient, pub_key_hash: Option<Hash>) -> Result<()> {
    log::info!("Performing DIUN");

    let nonce_diun_1 = Nonce::new().context("Error generating diun_nonce_1")?;
    let kexsuite = KexSuite::Ecdh384;
    let ciphersuite = CipherSuite::A256Gcm;
    let key_exchange = KeyExchange::new(kexsuite).context("Error initializing key exchange")?;

    // Send: Connect, Receive: Accept
    let accept: RequestResult<messages::diun::Accept> = client
        .send_request(
            messages::diun::Connect::new(nonce_diun_1, kexsuite, ciphersuite, key_exchange),
            None,
        )
        .await;
    let accept = accept.context("Error sending Connect")?.into_token();
    log::trace!("DIUN Accept token: {:?}", accept);
    let diun_pubkey = X5Chain::from_slice(&accept.get_unprotected_value::<Vec<u8>>(HeaderKeys::CUPHOwnerPubKey).context("Error getting diun_pubkey")?.context("No DIUN public key provided")?).context("Error parsing DIUN public chain")?;
    log::debug!("DIUN public key: {:?}", diun_pubkey);

    if let Some(pub_key_hash) = pub_key_hash {
        let first_cert = &diun_pubkey.chain()[0];
        first_cert.digest(pub_key_hash.get_type().try_into().context("Unsupported algorithm")?).context("Error computing first cert digest")?;
        if !first_cert_digest.eq(&pub_key_hash) {
            todo!();
        }
    }


    todo!();
}

async fn perform_di(client: &mut ServiceClient) -> Result<()> {
    todo!();
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let url = env::var("MANUFACTURING_SERVICE_URL")
        .context("Please provide MANUFACTURING_SERVICE_URL")?;
    let use_plain_di: bool = match env::var("USE_PLAIN_DI") {
        Ok(val) => val == "true",
        Err(_) => false,
    };
    let diun_pub_key_hash = if env::var("NO_DIUN_PUB_KEY_HASH").is_ok() {
        None
    } else {
        let hash = env::var("DIUN_PUB_KEY_HASH")
            .context("Please provide DIUN_PUB_KEY_HASH, or set NO_DIUN_PUB_KEY_HASH")?
            .replace(":", "");
        let hash = hex::decode(hash).context("DIUN_PUB_KEY_HASH is not valid hex")?;
        let hash =
            Hash::guess_new_from_data(hash).context("DIUN_PUB_KEY_HASH is not a valid hash")?;
        Some(hash)
    };

    log::info!(
        "Attempting manufacturing, url: {}, plain DI: {}, DIUN public key hash: {:?}",
        url,
        use_plain_di,
        diun_pub_key_hash
    );

    let mut client = ServiceClient::new(&url);

    if !use_plain_di {
        log::info!("Performing DIUN");
        perform_diun(&mut client, diun_pub_key_hash)
            .await
            .context("Error performing DIUN")?;
    }

    Ok(())
}