mod common;
use common::*;

use anyhow::{bail, Context, Result};
#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne};

const L: LogSide = LogSide::Test;

#[tokio::test]
async fn test_diun() -> Result<()> {
    let mut ctx = TestContext::new().context("Error building test context")?;

    let mfg_server = ctx
        .start_test_server(
            Binary::ManufacturingServer,
            |cfg| {
                cfg.prepare_config_file(None, |_| Ok(()))?;
                cfg.create_empty_storage_folder("sessions")?;
                cfg.create_empty_storage_folder("ownership_vouchers")?;
                Ok(())
            },
            |_| Ok(()),
        )
        .context("Error creating manufacturing server")?;
    ctx.wait_until_servers_ready()
        .await
        .context("Error waiting for servers to start")?;

    let client_result = ctx
        .run_client(Binary::ManufacturingClient, Some(&mfg_server), |cfg| {
            cfg.env("DEVICE_CREDENTIAL_FILENAME", "devicecredential.dc")
                .env("MANUFACTURING_INFO", "testdevice")
                .env("DIUN_PUB_KEY_INSECURE", "true");
            Ok(())
        })
        .context("Error running manufacturing client")?;

    client_result.expect_success()?;
    client_result.expect_stderr_line("Trusting any certificate as root")?;

    let dc_path = client_result.client_path().join("devicecredential.dc");
    L.l(format!("Device Credential should be in {:?}", dc_path));

    todo!();
}
