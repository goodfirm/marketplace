use cosmwasm_std::{coin, coins, Addr, Timestamp};
use cw_multi_test::Executor;
use sg2::{msg::Sg2ExecuteMsg, tests::mock_collection_params};
use sg_multi_test::StargazeApp;
use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};
use vending_factory::{
    msg::{VendingMinterCreateMsg, VendingMinterInitMsgExtension},
    state::{ParamsExtension, VendingMinterParams},
};
use crate::{setup_contracts::{contract_factory, contract_minter,contract_sg721, custom_mock_app}, setup_accounts_and_block::{setup_block_time, setup_accounts}};

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

pub fn mock_create_minter(splits_addr: Option<String>) -> VendingMinterCreateMsg {
    VendingMinterCreateMsg {
        init_msg: mock_init_extension(splits_addr),
        collection_params: mock_collection_params(),
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

// Upload contract code and instantiate minter contract
fn setup_minter_contract(
    router: &mut StargazeApp,
    creator: &Addr,
    num_tokens: u32,
    splits_addr: Option<String>,
) -> (Addr, vending_minter::msg::ConfigResponse) {
    let minter_code_id = router.store_code(contract_minter());
    println!("minter_code_id: {}", minter_code_id);
    let creation_fee = coins(CREATION_FEE, NATIVE_DENOM);

    let factory_code_id = router.store_code(contract_factory());
    println!("factory_code_id: {}", factory_code_id);

    let mut params = mock_params();
    params.code_id = minter_code_id;

    let factory_addr = router
        .instantiate_contract(
            factory_code_id,
            creator.clone(),
            &vending_factory::msg::InstantiateMsg { params },
            &[],
            "factory",
            None,
        )
        .unwrap();

    let sg721_code_id = router.store_code(contract_sg721());
    println!("sg721_code_id: {}", sg721_code_id);

    let mut msg = mock_create_minter(splits_addr);
    msg.init_msg.mint_price = coin(MINT_PRICE, NATIVE_DENOM);
    msg.init_msg.num_tokens = num_tokens;
    msg.collection_params.code_id = sg721_code_id;
    msg.collection_params.info.creator = creator.to_string();

    let msg = Sg2ExecuteMsg::CreateMinter(msg);

    let res = router.execute_contract(creator.clone(), factory_addr, &msg, &creation_fee);

    assert!(res.is_ok());

    // could get the minter address from the response above, but we know its contract1
    let minter_addr = Addr::unchecked("contract1");

    let config: vending_minter::msg::ConfigResponse = router
        .wrap()
        .query_wasm_smart(
            minter_addr.clone(),
            &vending_minter::msg::QueryMsg::Config {},
        )
        .unwrap();

    (minter_addr, config)
}

pub fn configure_minter(
    app: &mut StargazeApp,
) -> (Addr, Addr, Addr, Addr) {
    let (owner, bidder, creator) = setup_accounts(app).unwrap();
    let num_tokens = 1;
    let (minter_addr, config) = setup_minter_contract(app, &creator, num_tokens, None);

    // let whitelist_addr =
    //     configure_collection_whitelist(app, creator.clone(), buyer.clone(), minter_addr.clone());

    // setup_block_time(app, GENESIS_MINT_START_TIME);
    (minter_addr, owner, bidder, creator)
}