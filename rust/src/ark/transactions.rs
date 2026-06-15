use crate::ark::client::ArkWallet;
use anyhow::{anyhow, Result};

pub enum ArkTransaction {
    Boarding {
        txid: String,
        sats: i64,
        confirmed_at: Option<i64>,
    },
    Commitment {
        txid: String,
        sats: i64,
        created_at: i64,
    },
    Redeem {
        txid: String,
        sats: i64,
        is_settled: bool,
        created_at: i64,
    },
}

impl ArkWallet {
    pub async fn transaction_history(&self) -> Result<Vec<ArkTransaction>> {
        let mut txs = self
            .inner
            .transaction_history()
            .await
            .map_err(|error| anyhow!("Failed getting transaction history {error:#}"))?;

        // sort desc, i.e. newest transactions first
        txs.sort_by_key(|b| std::cmp::Reverse(b.created_at()));

        let result = txs
            .into_iter()
            .map(|tx| match tx {
                ark_core::history::Transaction::Boarding {
                    txid,
                    amount,
                    confirmed_at,
                } => ArkTransaction::Boarding {
                    txid: txid.to_string(),
                    sats: amount.to_sat() as i64,
                    confirmed_at: confirmed_at,
                },
                ark_core::history::Transaction::Commitment {
                    txid,
                    amount,
                    created_at,
                } => ArkTransaction::Commitment {
                    txid: txid.to_string(),
                    sats: amount.to_sat(),
                    created_at,
                },
                ark_core::history::Transaction::Ark {
                    txid,
                    amount,
                    is_settled,
                    created_at,
                } => ArkTransaction::Redeem {
                    txid: txid.to_string(),
                    sats: amount.to_sat(),
                    is_settled,
                    created_at,
                },
            })
            .collect();

        Ok(result)
    }
}
