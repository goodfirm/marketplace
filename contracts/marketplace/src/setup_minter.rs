use crate::{
    mock_collection_params::mock_collection_params_1,
    multitest::SetupContractsParams,
    setup_accounts_and_block::{setup_accounts, setup_block_time},
    setup_contracts::{contract_factory, contract_minter, contract_sg721, custom_mock_app},
};
use cosmwasm_std::{coin, coins, Addr, Timestamp};
use cw_multi_test::Executor;
use sg2::{
    msg::{CollectionParams, Sg2ExecuteMsg},
    tests::mock_collection_params,
};
use sg_multi_test::StargazeApp;
use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};
use vending_factory::{
    msg::{VendingMinterCreateMsg, VendingMinterInitMsgExtension},
    state::{ParamsExtension, VendingMinterParams},
};

// use crate::tests_folder::collection_constants::{
//     AIRDROP_MINT_FEE_BPS, AIRDROP_MINT_PRICE, CREATION_FEE, MAX_PER_ADDRESS_LIMIT, MAX_TOKEN_LIMIT,
//     MINT_FEE_BPS, MINT_PRICE, MIN_MINT_PRICE, SHUFFLE_FEE,
// };
// use crate::tests_folder::setup_accounts_and_block::{setup_accounts, setup_block_time};
// use crate::tests_folder::setup_contracts::{
//     contract_factory, contract_minter, contract_sg721, mock_minter,
// };

pub const CREATION_FEE: u128 = 5_000_000_000;
pub const INITIAL_BALANCE: u128 = 2_000_000_000;

pub const MINT_PRICE: u128 = 100_000_000;
pub const WHITELIST_AMOUNT: u128 = 66_000_000;
pub const WL_PER_ADDRESS_LIMIT: u32 = 1;
pub const MAX_TOKEN_LIMIT: u32 = 10000;

pub const MIN_MINT_PRICE: u128 = 50_000_000;
pub const AIRDROP_MINT_PRICE: u128 = 0;
pub const MINT_FEE_FAIR_BURN: u64 = 1_000; // 10%
pub const AIRDROP_MINT_FEE_FAIR_BURN: u64 = 10_000; // 100%
pub const SHUFFLE_FEE: u128 = 500_000_000;
pub const MAX_PER_ADDRESS_LIMIT: u32 = 50;

pub fn mock_init_extension(splits_addr: Option<String>) -> VendingMinterInitMsgExtension {
    vending_factory::msg::VendingMinterInitMsgExtension {
        base_token_uri: "ipfs://aldkfjads".to_string(),
        payment_address: splits_addr,
        start_time: Timestamp::from_nanos(GENESIS_MINT_START_TIME),
        num_tokens: 100,
        mint_price: coin(MIN_MINT_PRICE, NATIVE_DENOM),
        per_address_limit: 5,
        whitelist: None,
    }
}

pub fn mock_create_minter(
    splits_addr: Option<String>,
    collection_params: CollectionParams,
) -> VendingMinterCreateMsg {
    VendingMinterCreateMsg {
        init_msg: mock_init_extension(splits_addr),
        collection_params: collection_params,
    }
}

pub fn mock_params() -> VendingMinterParams {
    VendingMinterParams {
        code_id: 1,
        creation_fee: coin(CREATION_FEE, NATIVE_DENOM),
        min_mint_price: coin(MIN_MINT_PRICE, NATIVE_DENOM),
        mint_fee_bps: MINT_FEE_FAIR_BURN,
        max_trading_offset_secs: 60 * 60 * 24 * 7,
        extension: ParamsExtension {
            max_token_limit: MAX_TOKEN_LIMIT,
            max_per_address_limit: MAX_PER_ADDRESS_LIMIT,
            airdrop_mint_price: coin(AIRDROP_MINT_PRICE, NATIVE_DENOM),
            airdrop_mint_fee_bps: AIRDROP_MINT_FEE_FAIR_BURN,
            shuffle_fee: coin(SHUFFLE_FEE, NATIVE_DENOM),
        },
    }
}

pub struct MinterSetupParams<'a> {
    pub router: &'a mut StargazeApp,
    minter_admin: Addr,
    num_tokens: u32,
    collection_params: CollectionParams,
    splits_addr: Option<String>,
    minter_code_id: u64,
    factory_code_id: u64,
    sg721_code_id: u64,
}

pub struct MinterCollectionResponse {
    pub minter: Addr,
    pub collection: Addr,
}
// Upload contract code and instantiate minter contract
fn setup_minter_contract(setup_params: MinterSetupParams) -> MinterCollectionResponse {
    let minter_code_id = setup_params.minter_code_id;
    let router = setup_params.router;
    let factory_code_id = setup_params.factory_code_id;
    let sg721_code_id = setup_params.sg721_code_id;
    let minter_admin = setup_params.minter_admin;
    let num_tokens = setup_params.num_tokens;
    let splits_addr = setup_params.splits_addr;
    let collection_params = setup_params.collection_params;

    let mut params = mock_params();
    params.code_id = minter_code_id;

    let factory_addr = router
        .instantiate_contract(
            factory_code_id,
            minter_admin.clone(),
            &vending_factory::msg::InstantiateMsg { params },
            &[],
            "factory",
            None,
        )
        .unwrap();

    let mut msg = mock_create_minter(splits_addr, collection_params);
    msg.init_msg.mint_price = coin(MINT_PRICE, NATIVE_DENOM);
    msg.init_msg.num_tokens = num_tokens;
    msg.collection_params.code_id = sg721_code_id;
    msg.collection_params.info.creator = minter_admin.to_string();
    let creation_fee = coins(CREATION_FEE, NATIVE_DENOM);

    let msg = Sg2ExecuteMsg::CreateMinter(msg);

    let res = router.execute_contract(
        minter_admin.clone(),
        factory_addr.clone(),
        &msg,
        &creation_fee,
    );
    assert!(res.is_ok());
    println!("res factory is {:?}", res);
    let minter_addr = res.unwrap().events[3].attributes[0].value.clone();
    println!("minder addr return is {:?}", minter_addr);

    let config: vending_minter::msg::ConfigResponse = router
        .wrap()
        .query_wasm_smart(
            minter_addr.clone(),
            &vending_minter::msg::QueryMsg::Config {},
        )
        .unwrap();
    let collection_addr = config.sg721_address;

    MinterCollectionResponse {
        minter: Addr::unchecked(minter_addr),
        collection: Addr::unchecked(collection_addr),
    }
}

pub fn get_code_ids(router: &mut StargazeApp) -> (u64, u64, u64) {
    let minter_code_id = router.store_code(contract_minter());
    println!("minter_code_id: {}", minter_code_id);

    let factory_code_id = router.store_code(contract_factory());
    println!("factory_code_id: {}", factory_code_id);

    let sg721_code_id = router.store_code(contract_sg721());
    println!("sg721_code_id: {}", sg721_code_id);
    (minter_code_id, factory_code_id, sg721_code_id)
}

pub fn configure_minter(
    app: &mut StargazeApp,
    minter_admin: Addr,
    collection_params_vec: Vec<CollectionParams>,
    num_tokens: u32,
) -> Vec<MinterCollectionResponse> {
    let mut minter_collection_info: Vec<MinterCollectionResponse> = vec![];
    let (minter_code_id, factory_code_id, sg721_code_id) = get_code_ids(app);
    for collection_param in collection_params_vec {
        let setup_params: MinterSetupParams = MinterSetupParams {
            router: app,
            minter_admin: minter_admin.clone(),
            num_tokens,
            collection_params: collection_param,
            splits_addr: None,
            minter_code_id,
            factory_code_id,
            sg721_code_id,
        };
        let minter_collection_res = setup_minter_contract(setup_params);
        minter_collection_info.push(minter_collection_res);
    }
    minter_collection_info
}
