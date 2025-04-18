use crate::http_client::HttpClient;
use reqwest::{
    Method,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{Value, json};
use super::{
    resp::{
        AddressSummary, BalanceMulti, OkLinkBalanceDetail, OkLinkBalancePage, OkLinkResp,
        PublishTxInfo,
    },
    types::{InscriptionOk, OkApiUri, UtxoList},
};

/// `OkLinkClient` 是一个用于与 OkLink API 进行交互的客户端结构体。
/// 它封装了 HTTP 请求逻辑，并提供了多种方法来查询链上数据。
#[derive(Debug)]
pub struct OkLinkClient {
    client: HttpClient, // HTTP 客户端实例
    chain: String,      // 链名称（如 "BTC"）
    chain_id: u64,      // 链 ID
}

impl OkLinkClient {
    /// 创建一个新的 `OkLinkClient` 实例。
    ///
    /// # 参数
    /// - `base_url`: OkLink API 的基础 URL。
    /// - `api_key`: 用户的 API 密钥，用于身份验证。
    /// - `chain`: 链名称（如 "BTC"）。
    /// - `chain_id`: 链 ID。
    ///
    /// # 返回值
    /// 返回一个 `OkLinkClient` 实例。
    pub fn new(base_url: String, api_key: String, chain: String, chain_id: u64) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Ok-Access-Key", HeaderValue::from_str(&api_key).unwrap());

        OkLinkClient {
            client: HttpClient::new(base_url, Some(headers)),
            chain,
            chain_id,
        }
    }

    /// 获取链上代币的价格和市场数据。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<Value>`，包含价格和市场数据的 JSON 响应。
    pub async fn get_token_price_market_data(&self) -> anyhow::Result<Value> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?chainId={}",
                    OkApiUri::TokenPriceMarketData.as_str(),
                    self.chain_id
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(response)
    }

    /// 获取地址的汇总信息。
    ///
    /// # 参数
    /// - `address`: 要查询的地址。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<OkLinkResp<AddressSummary>>`，包含地址的汇总信息。
    pub async fn get_address_summary_oklink(
        &self,
        address: &str,
    ) -> anyhow::Result<OkLinkResp<AddressSummary>> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?chainShortName={}&address={}",
                    OkApiUri::AddressSummary.as_str(),
                    self.chain,
                    address
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// 获取地址的多币种余额信息。
    ///
    /// # 参数
    /// - `address`: 要查询的地址。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<OkLinkResp<BalanceMulti>>`，包含地址的多币种余额信息。
    pub async fn get_address_balance_oklink_multi(
        &self,
        address: &str,
    ) -> anyhow::Result<OkLinkResp<BalanceMulti>> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?chainShortName={}&address={}",
                    OkApiUri::BalanceMulti.as_str(),
                    self.chain,
                    address
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// 获取 BRC-20 代币的余额分页信息。
    ///
    /// # 参数
    /// - `address`: 要查询的地址。
    /// - `page`: 当前页码。
    /// - `page_size`: 每页的大小。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<OkLinkResp<OkLinkBalancePage>>`，包含 BRC-20 代币的余额分页信息。
    pub async fn get_brc20_balance_oklink(
        &self,
        address: &str,
        page: usize,
        page_size: usize,
    ) -> anyhow::Result<OkLinkResp<OkLinkBalancePage>> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?address={}&limit={}&page={}",
                    OkApiUri::BtcAddressBalanceList.as_str(),
                    address,
                    page_size,
                    page
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// 获取 BRC-20 代币的详细余额信息。
    ///
    /// # 参数
    /// - `address`: 要查询的地址。
    /// - `tick`: 代币符号。
    /// - `page`: 当前页码。
    /// - `page_size`: 每页的大小。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<OkLinkResp<OkLinkBalanceDetail>>`，包含 BRC-20 代币的详细余额信息。
    pub async fn get_brc20_token_detail_oklink(
        &self,
        address: &str,
        tick: &str,
        page: usize,
        page_size: usize,
    ) -> anyhow::Result<OkLinkResp<OkLinkBalanceDetail>> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?address={}&token={}&page={}&limit={}",
                    OkApiUri::BtcAddressBalanceDetail.as_str(),
                    address,
                    tick,
                    page,
                    page_size
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// 获取地址的铭文列表。
    ///
    /// # 参数
    /// - `address`: 要查询的地址。
    /// - `page`: 当前页码。
    /// - `page_size`: 每页的大小。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<Value>`，包含地址的铭文列表。
    pub async fn address_inscription_list_oklink(
        &self,
        address: &str,
        page: usize,
        page_size: usize,
    ) -> anyhow::Result<Value> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?chainShortName={}&protocolType=brc20&address={}&page={}&limit={}",
                    OkApiUri::InscriptionAddressInscriptionList.as_str(),
                    self.chain,
                    address,
                    page,
                    page_size
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(response)
    }

    /// 发布交易。
    ///
    /// # 参数
    /// - `signed_tx`: 已签名的交易数据。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<PublishTxInfo>`，包含交易发布的结果信息。
    pub async fn publish_tx(&self, signed_tx: &str) -> anyhow::Result<PublishTxInfo> {
        let response = self
            .client
            .request(
                OkApiUri::TransactionPublicshTx.as_str(),
                Method::POST,
                Some(&json!({
                    "chainShortName":self.chain,
                    "signedTx":signed_tx
                })),
                true,
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// 获取地址的 UTXO 列表。
    ///
    /// # 参数
    /// - `address`: 要查询的地址。
    /// - `cursor`: 当前游标位置。
    /// - `size`: 每次请求的大小。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<OkLinkResp<UtxoList>>`，包含地址的 UTXO 列表。
    pub async fn get_btc_utxo_oklink(
        &self,
        address: &str,
        cursor: usize,
        size: usize,
    ) -> anyhow::Result<OkLinkResp<UtxoList>> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?chainShortName={}&address={}&page={}&limit={}",
                    OkApiUri::AddressUtxo.as_str(),
                    self.chain,
                    address,
                    cursor,
                    size
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// 根据铭文 ID 获取 UTXO 信息。
    ///
    /// # 参数
    /// - `inscription_id`: 铭文 ID。
    /// - `inscription_number`: 铭文编号。
    ///
    /// # 返回值
    /// 返回一个 `anyhow::Result<OkLinkResp<InscriptionOk>>`，包含铭文对应的 UTXO 信息。
    pub async fn get_utxo_by_inscription_id_oklink(
        &self,
        inscription_id: &str,
        inscription_number: &str,
    ) -> anyhow::Result<OkLinkResp<InscriptionOk>> {
        let response = self
            .client
            .request(
                &format!(
                    "{}?inscriptionId={}&inscriptionNumber={}",
                    OkApiUri::BtcTransactionList.as_str(),
                    inscription_id,
                    inscription_number
                ),
                Method::GET,
                None,
                true,
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}
