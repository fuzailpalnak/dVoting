use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/dVoting-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/dVoting.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn no_option_in_storage() {
    let (_instance, _id) = get_contract_instance().await;

    let result = _instance
    .methods()
    .is_option(4)
    .call()
    .await
    .unwrap();
    
    assert_eq!(false, result.value);

}

#[tokio::test]
async fn success_when_vote() {
    let (_instance, _id) = get_contract_instance().await;

    let result = _instance
    .methods()
    .add_option(2)
    .call()
    .await
    .unwrap();


    let result = _instance
    .methods()
    .cast_vote(2)
    .call()
    .await
    .unwrap();

    assert_eq!(2, result.value);

    let result = _instance
    .methods()
    .has_voted()
    .call()
    .await
    .unwrap();

    assert_eq!(true, result.value);


}



#[tokio::test]
async fn fail_when_option_not_present_to_vote() {
    let (_instance, _id) = get_contract_instance().await;

    let result = _instance
    .methods()
    .add_option(2)
    .call()
    .await
    .unwrap();

    let result = _instance
    .methods()
    .is_option(2)
    .call()
    .await
    .unwrap();
    
    assert_eq!(true, result.value);

    let result = _instance
    .methods()
    .is_option(5)
    .call()
    .await
    .unwrap();
    
    assert_eq!(false, result.value);


    let result = _instance
    .methods()
    .cast_vote(5)
    .call()
    .await
    .unwrap();

    assert_eq!(0, result.value);

}

#[tokio::test]
async fn fail_when_voting_again() {
    let (_instance, _id) = get_contract_instance().await;

    let result = _instance
    .methods()
    .add_option(2)
    .call()
    .await
    .unwrap();

    let result = _instance
    .methods()
    .cast_vote(2)
    .call()
    .await
    .unwrap();

    assert_eq!(2, result.value);

    let result = _instance
    .methods()
    .cast_vote(2)
    .call()
    .await
    .unwrap();

    assert_eq!(1, result.value);

}

#[tokio::test]
async fn success_for_option_present_after_vote() {
    let (_instance, _id) = get_contract_instance().await;
    let result = _instance
    .methods()
    .add_option(2)
    .call()
    .await
    .unwrap();

    let result = _instance
    .methods()
    .cast_vote(2)
    .call()
    .await
    .unwrap();

    assert_eq!(2, result.value);

    let result = _instance
    .methods()
    .is_option(2)
    .call()
    .await
    .unwrap();
    
    assert_eq!(true, result.value);


}

#[tokio::test]
async fn voting_count_match() {
    let (_instance, _id) = get_contract_instance().await;

    let result = _instance
    .methods()
    .add_option(5)
    .call()
    .await
    .unwrap();
    
    let result = _instance
    .methods()
    .cast_vote(5)
    .call()
    .await
    .unwrap();

    assert_eq!(5, result.value);

    let result = _instance
    .methods()
    .is_option(5)
    .call()
    .await
    .unwrap();
    
    assert_eq!(true, result.value);


    let result = _instance
    .methods()
    .get_count(5)
    .call()
    .await
    .unwrap();
    
    assert_eq!(1, result.value);


}
