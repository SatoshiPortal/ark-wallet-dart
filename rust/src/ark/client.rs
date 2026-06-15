use crate::ark::esplora::EsploraClient;
use crate::ark::storage::InMemoryDb;
use anyhow::{anyhow, Result};
// use ark_client::wallet::OnchainWallet;
use bitcoin::secp256k1::{Keypair, Secp256k1};
use bitcoin::Network;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
// Re-export types that flutter_rust_bridge needs
pub(crate) use ark_bdk_wallet::Wallet;
pub(crate) use ark_client::{Client, InMemorySwapStorage, OfflineClient};

#[derive(Clone)]
pub struct ArkWallet {
    pub(crate) inner: Arc<Client<EsploraClient, Wallet<InMemoryDb>, InMemorySwapStorage>>,
}

impl ArkWallet {
    pub async fn init(
        secret_key: Vec<u8>,
        network: String,
        esplora: String,
        server: String,
        boltz: String,
    ) -> Result<ArkWallet> {
        let network = Network::from_str(network.as_str())?;
        let secp = Secp256k1::new();
        let kp = Keypair::from_seckey_slice(&secp, secret_key.as_slice())?;

        let db = InMemoryDb::default();
        let wallet = ark_bdk_wallet::Wallet::new(kp, secp, network, esplora.as_str(), db)?;
        // wallet.sync().await?;
        let wallet = Arc::new(wallet);

        let esplora = EsploraClient::new(esplora.as_str())?;
        esplora.check_connection().await?;

        let client = OfflineClient::new(
            "Bull Bitcoin".to_string(),
            kp,
            Arc::new(esplora),
            wallet,
            server.clone(),
            Arc::new(InMemorySwapStorage::new()),
            boltz.clone(),
            Duration::from_secs(30),
        )
        .connect()
        .await
        .map_err(|err| anyhow!("Failed to connect to Ark server at '{}': {}", server, err))?;

        Ok(ArkWallet {
            inner: Arc::new(client),
        })
    }
}
