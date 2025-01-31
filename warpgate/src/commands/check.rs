use anyhow::{Context, Result};
use tracing::*;
use warpgate_common::{TlsCertificateBundle, TlsPrivateKey};

use crate::config::load_config;

pub(crate) async fn command(cli: &crate::Cli) -> Result<()> {
    let config = load_config(&cli.config, true)?;
    if config.store.http.enable {
        TlsCertificateBundle::from_file(
            config
                .paths_relative_to
                .join(&config.store.http.certificate),
        )
        .await
        .with_context(|| "Checking HTTPS certificate".to_string())?;
        TlsPrivateKey::from_file(config.paths_relative_to.join(&config.store.http.key))
            .await
            .with_context(|| "Checking HTTPS key".to_string())?;
    }
    if config.store.mysql.enable {
        TlsCertificateBundle::from_file(
            config
                .paths_relative_to
                .join(&config.store.mysql.certificate),
        )
        .await
        .with_context(|| "Checking MySQL certificate".to_string())?;
        TlsPrivateKey::from_file(config.paths_relative_to.join(&config.store.mysql.key))
            .await
            .with_context(|| "Checking MySQL key".to_string())?;
    }
    if !config.store.sso_providers.is_empty() && config.store.external_host.is_none() {
        anyhow::bail!("SSO requires the external_host config option");
    }
    info!("No problems found");
    Ok(())
}
