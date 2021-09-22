mod common;
use common::*;

use anyhow::{bail, Context, Result};
#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne};

const L: LogSide = LogSide::Test;

#[tokio::test]
async fn test_diun() -> Result<()> {
    let mut ctx = TestContext::new().context("Error building test context")?;

    ctx.start_test_server(
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

    L.l(format!("Test context: {:?}", ctx));

    bail!("This test has not been implemented yet.");
}
