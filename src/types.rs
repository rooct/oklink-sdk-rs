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

/*
{
            "page": "1",
            "limit": "1",
            "totalPage": "635",
            "chainFullName": "Ethereum",
            "chainShortName": "ETH",
            "blockList": [
                {
                    "txid": "0x5a597e627d67a4e9daa9b710bf217c6690a2ac09521b45ffbb0b82b0f6d84245",
                    "methodId": "0x771d503f",
                    "blockHash": "0xadaed44b8d75332a8627a490cdd49e8aab227c901859f7918aea2b7f6d54e297",
                    "height": "18126560",
                    "transactionTime": "1694598095000",
                    "from": "0x104da4efb22a7e560e6df9c813e5eb54ca038737",
                    "isFromContract": false,
                    "isToContract": true,
                    "to": "0x51c72848c68a965f66fa7a88855f9f7784502a7f",
                    "amount": "0",
                    "transactionSymbol": "ETH",
                    "txfee": "0.004454715411444375",
                    "state": "success",
                    "tokenId": "",
                    "tokenContractAddress": ""
                }
            ]
        }
*/
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransactionList {
    pub page: String,
    pub limit: String,
    pub total_page: String,
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub block_list: Vec<BlockTransaction>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransaction {
    pub txid: String,
    pub method_id: String,
    pub block_hash: String,
    pub height: String,
    pub transaction_time: String,
    pub from: String,
    pub is_from_contract: bool,
    pub is_to_contract: bool,
    pub to: String,
    pub amount: String,
    pub transaction_symbol: String,
    pub txfee: String,
    pub state: String,
    pub token_id: String,
    pub token_contract_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransactionListMulti {
    pub page: String,
    pub limit: String,
    pub total_page: String,
    pub transaction_list: Vec<BlockTransactionMulti>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransactionMulti {
    pub height: String,
    pub tx_id: String,
    pub block_hash: String,
    pub transaction_time: String,
    pub from: String,
    pub is_from_contract: bool,
    pub is_to_contract: bool,
    pub to: String,
    pub amount: String,
    pub transaction_symbol: String,
    pub tx_fee: String,
    pub state: String,
    pub token_id: String,
    pub token_contract_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InscriptionData {
    pub tx_id: String,
    pub block_height: String,
    pub state: String,
    pub token_type: String,
    pub action_type: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub token: String,
    pub inscription_id: String,
    pub inscription_number: String,
    pub index: String,
    pub location: String,
    pub  msg: String,
    pub  time: String,
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
    BrcTransactionList,
    TokenPriceMarketData,
    BlockTransactionMulti,
    BlockTransaction,
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
            OkApiUri::BrcTransactionList => "/btc/transaction-list",
            OkApiUri::TokenPriceMarketData => "/tokenprice/market-data",
            OkApiUri::BlockTransactionMulti => "/block/transaction-list-multi",
            OkApiUri::BlockTransaction => "/block/transaction-list",
        }
    }
}
