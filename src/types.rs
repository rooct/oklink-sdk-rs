use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkRespBalance {
    pub balance_list: Vec<OkDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkDetail {
    pub token: String,
    pub token_type: String,
    pub balance: String,
    pub available_balance: String,
    pub transfer_balance: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OkResp {
    pub data: Vec<OkRespBalance>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoList {
    page: String,
    limit: String,
    total_page: String,
    pub utxo_list: Vec<UtxoOk>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoOk {
    pub txid: String,
    pub height: String,
    pub block_time: String,
    pub address: String,
    pub unspent_amount: String,
    pub index: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InscriptionOk {
    page: String,
    limit: String,
    total_page: String,
    total_inscription: String,
    pub inscriptions_list: Vec<InscriptionData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InscriptionData {
    pub tx_id: String,
    block_height: String,
    state: String,
    token_type: String,
    action_type: String,
    from_address: String,
    pub to_address: String,
    pub amount: String,
    token: String,
    inscription_id: String,
    inscription_number: String,
    pub index: String,
    location: String,
    msg: String,
    time: String,
}

#[derive(Debug)]
pub enum OkApiUri {
    AddressSummary,
    BalanceMulti,
    BtcAddressBalanceList,
    BtcAddressBalanceDetail,
    InscriptionAddressInscriptionList,
    TransactionPublicshTx,
    AddressUtxo,
    BtcTransactionList,
    TokenPriceMarketData,
}

impl OkApiUri {
    pub fn as_str(&self) -> &str {
        match self {
            OkApiUri::AddressSummary => "/address/address-summary",
            OkApiUri::BalanceMulti => "/address/balance-multi",
            OkApiUri::BtcAddressBalanceList => "/btc/address-balance-list",
            OkApiUri::BtcAddressBalanceDetail => "/btc/address-balance-details",
            OkApiUri::InscriptionAddressInscriptionList => "/inscription/address-inscription-list",
            OkApiUri::TransactionPublicshTx => "/transaction/publish-tx",
            OkApiUri::AddressUtxo => "/address/utxo",
            OkApiUri::BtcTransactionList => "/btc/transaction-list",
            OkApiUri::TokenPriceMarketData => "/tokenprice/market-data",
        }
    }
}
