use fuels::macros::abigen;

abigen!(Script(
    name = "MyScript",
    abi = "deposit-withdraw-script/out/debug/deposit-withdraw-script-abi.json"
));

mod success {
    use crate::functions::core::deposit_withdraw_script_test::MyScript;
    use fuels::{
        test_helpers::{launch_custom_provider_and_get_wallets, AssetConfig, WalletsConfig},
        types::{AssetId, ContractId, Identity},
    };
    use spark_market_sdk::SparkMarketContract;

    #[tokio::test]
    async fn deposit_withdraw_script_test() -> anyhow::Result<()> {
        let number_of_wallets = 4;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000_000_000;

        let eth_asset_id = AssetId::new([0; 32]);
        let btc_asset_id = AssetId::new([1; 32]);
        let usdc_asset_id = AssetId::new([2; 32]);

        let ids = vec![eth_asset_id, btc_asset_id, usdc_asset_id];
        let mut assets: Vec<AssetConfig> = Vec::with_capacity(3);

        for id in ids {
            assets.push(AssetConfig {
                id,
                num_coins: coins_per_wallet,
                coin_amount: amount_per_coin,
            });
        }
        let config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);

        let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await?;
        let owner = wallets.pop().unwrap();
        let user0 = wallets.pop().unwrap();
        // let user1 = wallets.pop().unwrap();
        // let matcher = wallets.pop().unwrap();

        let btc_usdc_market = SparkMarketContract::deploy(
            btc_asset_id,
            8,
            usdc_asset_id,
            6,
            owner.clone(),
            9,
            0xFAFBFC,
        )
        .await?;
        let eth_usdc_market = SparkMarketContract::deploy(
            eth_asset_id,
            8,
            usdc_asset_id,
            6,
            owner.clone(),
            9,
            0xFAFBFC,
        )
        .await?;

        let bin_path = "./deposit-withdraw-script/out/debug/deposit-withdraw-script.bin";
        let script_instance = MyScript::new(user0.clone(), bin_path);

        let user: Identity = Identity::Address(user0.address().into());
        let amount_to_transfer: u64 = 1000_000_000;
        let btc_usdc_market_contract: ContractId = btc_usdc_market.contract_id().into();
        let eth_usdc_market_contract: ContractId = eth_usdc_market.contract_id().into();
        let _ = script_instance
            .main(
                user,
                usdc_asset_id,
                amount_to_transfer,
                btc_usdc_market_contract,
                eth_usdc_market_contract,
            )
            .call()
            .await?;

        Ok(())
    }
}
