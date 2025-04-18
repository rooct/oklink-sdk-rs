pub mod http_client;
pub mod oklink;
pub mod resp;
pub mod types;

#[cfg(test)]
mod testx {
    use crate::oklink::OkLinkClient;

    #[tokio::test]
    async fn test_get_btc_utxo_oklink() {
        dotenv::dotenv().ok();
        println!("=============");
        let api_key = std::env::var("OKLINK_API_KEY").expect("OKLINK_API_KEY must be set");
        let client = OkLinkClient::new(
            "https://www.oklink.com/api/v5/explorer".to_string(),
            api_key,
            "btc".to_string(),
            1,
        );
        let resp = client
            .get_btc_utxo_oklink("1BM1sAcrfV6d4zPKytzziu4McLQDsFC2Qc", 1, 100)
            .await
            .unwrap();
        println!("{:#?}", resp.data);
    }
}
