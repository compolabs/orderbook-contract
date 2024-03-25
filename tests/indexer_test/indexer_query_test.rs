use std::path::PathBuf;

#[tokio::test]
async fn indexer_test() {
    let indexer_config = skar_client_fuel::Config {
        url: "http://fuel.hypersync.xyz/query".parse().unwrap(),
        bearer_token: None,
        http_req_timeout_millis: 60000.try_into().unwrap(),
    };
    let indexer = skar_client_fuel::Client::new(indexer_config).unwrap();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/indexer_test/query.json");
    let addresses_json = std::fs::read_to_string(path).unwrap();
    let query = serde_json::from_str(&addresses_json).unwrap();

    let res = indexer
        .get_arrow_data_with_retry::<skar_client_fuel::ArrowIpc>(&query)
        .await
        .unwrap();

    println!("{:#?}", res.data);
}
