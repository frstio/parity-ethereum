use ethereum_types::{H256};
use v1::types::{RichBlock, Log, Receipt};
use serde::{Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CombinedBlock {
    /// Block
    pub block: RichBlock,

    /// Logs
    pub logs: Vec<Log>,

    /// Uncles
    pub uncles: Vec<H256>,

    /// Transaction receipts
    pub tx_receipts: Vec<Receipt>
}