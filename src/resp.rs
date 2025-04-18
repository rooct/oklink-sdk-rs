use serde::{Deserialize, Serialize};

// 定义OKLink余额详情结构体
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OkLinkBalanceDetail {
    /// 当前页码
    pub page: String,
    /// 每页限制数量
    pub limit: String,
    /// 总页数
    pub total_page: String,
    /// 代币类型
    pub token_type: String,
    /// 总余额
    pub balance: String,
    /// 可用余额
    pub available_balance: String,
    /// 转账余额
    pub transfer_balance: String,
    /// 转账余额列表
    pub transfer_balance_list: Vec<OkLinkDetailInscription>,
}

// 定义OKLink详细铭文结构体
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OkLinkDetailInscription {
    /// 铭文ID
    pub inscription_id: String,
    /// 铭文编号
    pub inscription_number: String,
    /// 金额
    pub amount: String,
}

// 定义OKLink通用响应结构体
#[derive(Deserialize, Serialize, Debug)]
pub struct OkLinkResp<T: Serialize> {
    /// 响应码
    code: String,
    /// 响应消息
    msg: String,
    /// 数据列表
    pub data: Vec<T>,
}

// 定义Token价格结构体
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    /// 最后价格
    pub last_price: String,
}

// 定义OKLink余额分页结构体
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OkLinkBalancePage {
    /// 当前页码
    page: String,
    /// 每页限制数量
    limit: String,
    /// 总页数
    total_page: String,
    /// 余额列表
    pub balance_list: Vec<OkLinkBalance>,
}

// 定义OKLink余额结构体
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OkLinkBalance {
    /// 代币
    pub token: String,
    /// 代币类型
    token_type: String,
    /// 余额
    pub balance: String,
    /// 可用余额
    pub available_balance: String,
    /// 转账余额
    pub transfer_balance: String,
}

// 定义地址摘要结构体
#[derive(Serialize, Deserialize, Clone)]
pub struct AddressSummary {
    /// 链全名
    chain_full_name: String,
    /// 链简称
    chain_short_name: String,
    /// 地址
    address: String,
    /// 合约地址
    contract_address: String,
    /// 是否为生产者地址
    is_producer_address: bool,
    /// 余额
    pub balance: String,
    /// 余额符号
    balance_symbol: String,
    /// 交易计数
    transaction_count: String,
    /// 验证状态
    verifying: String,
    /// 发送金额
    send_amount: String,
    /// 接收金额
    receive_amount: String,
    /// 代币金额
    token_amount: String,
    /// 总代币价值
    total_token_value: String,
    /// 创建合约地址
    create_contract_address: String,
    /// 创建合约交易哈希
    create_contract_transaction_hash: String,
    /// 第一笔交易时间
    pub first_transaction_time: String,
    /// 最后一笔交易时间
    last_transaction_time: String,
    /// 代币
    token: String,
    /// 带宽
    bandwidth: String,
    /// 能量
    energy: String,
    /// 投票权
    voting_rights: String,
    /// 未领取的投票奖励
    unclaimed_voting_rewards: String,
    /// 是否为AA地址
    is_aa_address: bool,
}

// 定义余额项结构体
#[derive(Serialize, Deserialize, Clone)]
pub struct BalanceItem {
    /// 地址
    pub address: String,
    /// 余额
    pub balance: String,
}

// 定义余额列表结构体
#[derive(Serialize, Deserialize, Clone)]
pub struct BalanceList {
    /// 余额列表
    pub balance_list: Vec<BalanceItem>,
}

// 定义多余额结构体
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceMulti {
    /// 符号
    pub symbol: String,
    /// 余额列表
    pub balance_list: Vec<BalanceItem>,
}

// 定义发布交易信息结构体
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublishTxInfo {
    /// 链全名
    pub chain_full_name: String,
    /// 链简称
    pub chain_short_name: String,
    /// 交易ID
    pub txid: String,
}
