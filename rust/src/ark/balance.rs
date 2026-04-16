use crate::ark::client::ArkWallet;
use anyhow::{anyhow, Result};

pub use ark_client::OffChainBalance;

pub struct ArkBoarding {
    pub unconfirmed: i64,
    pub confirmed: i64,
    pub total: i64,
}

pub struct ArkBalance {
    pub preconfirmed: i64,
    pub settled: i64,
    pub available: i64,
    pub recoverable: i64,
    pub total: i64,
    pub boarding: ArkBoarding,
}

impl ArkWallet {
    pub async fn balance(&self) -> Result<ArkBalance> {
        let offchain_balance = self
            .inner
            .offchain_balance()
            .await
            .map_err(|error| anyhow!("Could not fetch balance {error}"))?;

        let preconfirmed = offchain_balance.pending().to_sat() as i64;
        let settled = offchain_balance.confirmed().to_sat() as i64;
        let available = settled;
        let recoverable = offchain_balance.pending().to_sat() as i64;
        let total = available + recoverable;

        let boarding_balance = self.calculate_boarding_balance().await?;

        Ok(ArkBalance {
            preconfirmed,
            settled,
            available,
            recoverable,
            total,
            boarding: boarding_balance,
        })
    }

    async fn calculate_boarding_balance(&self) -> Result<ArkBoarding> {
        let txs = self
            .inner
            .transaction_history()
            .await
            .map_err(|error| anyhow!("Could not fetch transaction history {error}"))?;

        let mut unconfirmed = 0i64;
        let mut confirmed = 0i64;

        for tx in txs {
            if let ark_core::history::Transaction::Boarding {
                amount,
                confirmed_at,
                ..
            } = tx
            {
                let sats = amount.to_sat() as i64;
                if confirmed_at.is_some() {
                    confirmed += sats;
                } else {
                    unconfirmed += sats;
                }
            }
        }

        Ok(ArkBoarding {
            unconfirmed,
            confirmed,
            total: unconfirmed + confirmed,
        })
    }
}
