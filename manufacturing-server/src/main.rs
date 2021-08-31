use std::convert::{TryFrom, TryInto};
use std::fs;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{Context, Error, Result};
use openssl::{
    pkey::{PKey, Private},
    x509::X509,
};
use serde::Deserialize;
use warp::Filter;

use fdo_data_formats::{
    constants::{KeyStorageType, MfgStringType},
    enhanced_types::X5Bag,
    ownershipvoucher::OwnershipVoucher,
    publickey::{PublicKey, PublicKeyBody},
    types::Guid,
};
use fdo_store::{Store, StoreDriver};

mod handlers;

struct DiunConfiguration {
    requires_attestation: bool,
    allowed_key_types: Vec<KeyTypes>,

    diun_key: PKey<Private>,
    diun_key_public: Vec<u8>,
}

struct ManufacturingServiceUD {
    // Stores
    session_store: Arc<fdo_http_wrapper::server::SessionStore>,
    ownership_voucher_store: Box<dyn Store<fdo_store::WriteOnlyOpen, Guid, OwnershipVoucher>>,
    public_key_store: Option<Box<dyn Store<fdo_store::ReadOnlyOpen, String, PublicKey>>>,

    // Certificates
    manufacturer_cert: X509,
    manufacturer_key: Option<PKey<Private>>,
    device_cert_key: PKey<Private>,
    device_cert_chain: Vec<X509>,
    owner_cert: Option<X509>,

    // Protocols
    enable_di: bool,

    // DIUN settings
    diun_configuration: Option<DiunConfiguration>,
}

type ManufacturingServiceUDT = Arc<ManufacturingServiceUD>;

#[derive(Debug, Deserialize)]
enum KeyTypes {
    FileSystem,
}

impl From<KeyTypes> for KeyStorageType {
    fn from(key_type: KeyTypes) -> Self {
        match key_type {
            KeyTypes::FileSystem => KeyStorageType::FileSystem,
        }
    }
}

#[derive(Debug, Deserialize)]
struct DiunSettings {
    requires_attestation: bool,
    allowed_key_types: Vec<KeyTypes>,

    diun_key_path: String,
}

impl TryFrom<DiunSettings> for DiunConfiguration {
    type Error = Error;

    fn try_from(value: DiunSettings) -> Result<DiunConfiguration, Error> {
        let diun_key = fs::read(value.diun_key_path).context("Error reading DIUN key")?;
        let diun_key = PKey::private_key_from_der(&diun_key).context("Error parsing DIUN key")?;
        let diun_key_public = diun_key.public_key().to_der().context("Error building public DIUN key")?;

        Ok(DiunConfiguration {
            requires_attestation: value.requires_attestation,
            allowed_key_types: value.allowed_key_types,

            diun_key,
            diun_key_public,
        })
    }
}

#[derive(Debug, Deserialize)]
struct ManufacturingSettings {
    manufacturer_cert_path: String,
    device_cert_ca_private_key: String,
    device_cert_ca_chain: String,

    owner_cert_path: Option<String>,
    manufacturer_private_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProtocolSetting {
    plain_di: Option<bool>,
    diun: Option<DiunSettings>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    // Session store info
    session_store_driver: StoreDriver,
    session_store_config: Option<config::Value>,

    // Ownership Voucher store info
    ownership_voucher_store_driver: StoreDriver,
    ownership_voucher_store_config: Option<config::Value>,

    // Public key store info
    public_key_store_driver: Option<StoreDriver>,
    public_key_store_config: Option<config::Value>,

    // Bind information
    bind: String,

    protocols: ProtocolSetting,

    manufacturing: ManufacturingSettings,
}

const MAINTENANCE_INTERVAL: u64 = 60;

async fn perform_maintenance(
    udt: ManufacturingServiceUDT,
) -> std::result::Result<(), &'static str> {
    log::info!(
        "Scheduling maintenance every {} seconds",
        MAINTENANCE_INTERVAL
    );

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(MAINTENANCE_INTERVAL)).await;

        let ov_maint = udt.ownership_voucher_store.perform_maintenance();
        let ses_maint = udt.session_store.perform_maintenance();

        #[allow(unused_must_use)]
        let (ov_res, ses_res) = tokio::join!(ov_maint, ses_maint);
        if let Err(e) = ov_res {
            log::warn!("Error during ownership voucher store maintenance: {:?}", e);
        }
        if let Err(e) = ses_res {
            log::warn!("Error during session store maintenance: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("manufacturing-service"))
        .context("Loading configuration files")?
        .merge(config::Environment::with_prefix("manufacturing-server"))
        .context("Loading configuration from environment variables")?;
    let settings: Settings = settings.try_into().context("Error parsing configuration")?;

    // Bind information
    let bind_addr = SocketAddr::from_str(&settings.bind)
        .with_context(|| format!("Error parsing bind string '{}'", &settings.bind))?;

    // Initialize stores
    let session_store = settings
        .session_store_driver
        .initialize(settings.session_store_config)
        .context("Error initializing session store")?;
    let session_store = fdo_http_wrapper::server::SessionStore::new(session_store);
    let ownership_voucher_store = settings
        .ownership_voucher_store_driver
        .initialize(settings.ownership_voucher_store_config)
        .context("Error initializing ownership voucher store")?;
    let public_key_store = match settings.public_key_store_driver {
        None => None,
        Some(driver) => Some(
            driver
                .initialize(settings.public_key_store_config)
                .context("Error initializing public key store")?,
        ),
    };

    // Read keys and certificates
    let device_cert_key = PKey::private_key_from_der(
        &fs::read(settings.manufacturing.device_cert_ca_private_key)
            .context("Error reading device CA private key")?,
    )
    .context("Error parsing device CA private key")?;
    let device_cert_chain = X509::stack_from_pem(
        &fs::read(settings.manufacturing.device_cert_ca_chain)
            .context("Error reading device CA chain")?,
    )
    .context("Error parsing device CA chain")?;
    let manufacturer_cert = X509::from_pem(
        &fs::read(settings.manufacturing.manufacturer_cert_path)
            .context("Error reading manufacturer certificate")?,
    )
    .context("Error parsing manufacturer certificate")?;

    let manufacturer_key = match settings.manufacturing.manufacturer_private_key {
        None => None,
        Some(path) => Some(
            PKey::private_key_from_der(
                &fs::read(path).context("Error reading manufacturer private key")?,
            )
            .context("Error parsing manufacturer private key")?,
        ),
    };
    let owner_cert = match settings.manufacturing.owner_cert_path {
        None => None,
        Some(path) => Some(
            X509::from_pem(&fs::read(path).context("Error reading owner certificate")?)
                .context("Error parsing owner certificate")?,
        ),
    };

    let diun_configuration = match settings.protocols.diun {
        None => None,
        Some(v) => Some(v.try_into().context("Error parsing DIUN configuration")?),
    };

    // Initialize user data
    let user_data = Arc::new(ManufacturingServiceUD {
        // Stores
        session_store: session_store.clone(),
        ownership_voucher_store,
        public_key_store,

        device_cert_key,
        device_cert_chain,
        manufacturer_cert,
        manufacturer_key,
        owner_cert,

        enable_di: settings.protocols.plain_di.unwrap_or(false),
        diun_configuration,
    });

    // Initialize handlers
    let hello = warp::get().map(|| "Hello from the manufacturing server");

    // DI
    let handler_di_app_start = fdo_http_wrapper::server::fdo_request_filter(
        user_data.clone(),
        session_store.clone(),
        handlers::di::app_start,
    );
    let handler_di_set_hmac = fdo_http_wrapper::server::fdo_request_filter(
        user_data.clone(),
        session_store.clone(),
        handlers::di::set_hmac,
    );

    // DIUN
    let handler_diun_connect = fdo_http_wrapper::server::fdo_request_filter(
        user_data.clone(),
        session_store.clone(),
        handlers::diun::connect,
    );
    let handler_diun_request_key_parameters = fdo_http_wrapper::server::fdo_request_filter(
        user_data.clone(),
        session_store.clone(),
        handlers::diun::request_key_parameters,
    );
    let handler_diun_provide_key = fdo_http_wrapper::server::fdo_request_filter(
        user_data.clone(),
        session_store.clone(),
        handlers::diun::provide_key,
    );

    let routes = warp::post()
        .and(
            hello
                // DI
                .or(handler_di_app_start)
                .or(handler_di_set_hmac)
                // DIUN
                .or(handler_diun_connect)
                .or(handler_diun_request_key_parameters)
                .or(handler_diun_provide_key),
        )
        .recover(fdo_http_wrapper::server::handle_rejection)
        .with(warp::log("manufacturing-server"));

    log::info!("Listening on {}", bind_addr);
    let server = warp::serve(routes);

    let maintenance_runner =
        tokio::spawn(async move { perform_maintenance(user_data.clone()).await });

    let server = server.run(bind_addr);
    let _ = tokio::join!(server, maintenance_runner);

    Ok(())
}
