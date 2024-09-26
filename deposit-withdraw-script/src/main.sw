script;

abi SparkMarket {
    #[payable]
    #[storage(read, write)]
    fn deposit();

    #[storage(read, write)]
    fn withdraw(amount: u64, asset_id: AssetId);

    #[storage(read)]
    fn account_balance(user: Identity, asset_id: AssetId) -> u64;
}

fn main(
    user: Identity,
    usdc_asset_id: AssetId,           // ID актива (например, USDC)
    amount_to_transfer: u64,          // Количество USDC для перевода
    btc_usdc_market_contract: ContractId,  // Адрес контракта BTC-USDC
    eth_usdc_market_contract: ContractId   // Адрес контракта ETH-USDC
) {
    // Получаем идентификатор вызывающего
    // let user = msg_sender().unwrap();

    let btc_usdc_market = abi(SparkMarket, btc_usdc_market_contract.into());
    let eth_usdc_market = abi(SparkMarket, eth_usdc_market_contract.into());

    // 1. Проверка баланса USDC на аккаунте пользователя
    let initial_balance = btc_usdc_market.account_balance(user, usdc_asset_id);
    require(initial_balance >= amount_to_transfer, "Недостаточно USDC для перевода.");

    // 2. Депозитим в BTC-USDC маркет
    btc_usdc_market.deposit{coins: amount_to_transfer, asset_id: usdc_asset_id.into()}();

    // Проверяем, что средства списаны с баланса пользователя и зачислены на маркет BTC-USDC
    let balance_after_btc_deposit = btc_usdc_market.account_balance(user, usdc_asset_id);
    require(balance_after_btc_deposit == initial_balance - amount_to_transfer, "Депозит в BTC-USDC не выполнен.");
    
    let btc_market_balance_after_deposit = btc_usdc_market.account_balance(user, usdc_asset_id);
    require(btc_market_balance_after_deposit >= amount_to_transfer, "Средства не зачислены на BTC-USDC маркет.");

    // 3. Выводим из BTC-USDC маркета
    btc_usdc_market.withdraw(amount_to_transfer, usdc_asset_id);

    // Проверяем, что средства вернулись на баланс пользователя и списаны с маркета
    let balance_after_btc_withdraw = btc_usdc_market.account_balance(user, usdc_asset_id);
    require(balance_after_btc_withdraw == initial_balance, "Вывод из BTC-USDC не выполнен.");

    let btc_market_balance_after_withdraw = btc_usdc_market.account_balance(user, usdc_asset_id);
    require(btc_market_balance_after_withdraw < amount_to_transfer, "Средства не списаны с BTC-USDC маркета.");

    // 4. Депозитим в ETH-USDC маркет
    eth_usdc_market.deposit{coins: amount_to_transfer, asset_id: usdc_asset_id.into()}();

    // Проверяем, что средства списаны с баланса пользователя и зачислены на маркет ETH-USDC
    let balance_after_eth_deposit = eth_usdc_market.account_balance(user, usdc_asset_id);
    require(balance_after_eth_deposit == balance_after_btc_withdraw - amount_to_transfer, "Депозит в ETH-USDC не выполнен.");

    let eth_market_balance_after_deposit = eth_usdc_market.account_balance(user, usdc_asset_id);
    require(eth_market_balance_after_deposit >= amount_to_transfer, "Средства не зачислены на ETH-USDC маркет.");
}
