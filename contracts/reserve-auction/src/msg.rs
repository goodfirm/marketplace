use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

use crate::state::{Auction, Config};

#[cw_serde]
pub struct InstantiateMsg {
    pub fair_burn: String,
    pub marketplace: String,
    pub min_bid_increment_bps: u64,
    pub min_duration: u64,
    pub max_duration: u64,
    pub extend_duration: u64,
    pub create_auction_fee: Coin,
    pub max_auctions_to_settle_per_block: u64,
    pub min_reserve_prices: Vec<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateAuction {
        collection: String,
        token_id: String,
        reserve_price: Coin,
        duration: u64,
        seller_funds_recipient: Option<String>,
    },
    UpdateReservePrice {
        collection: String,
        token_id: String,
        reserve_price: Coin,
    },
    CancelAuction {
        collection: String,
        token_id: String,
    },
    PlaceBid {
        collection: String,
        token_id: String,
    },
    SettleAuction {
        collection: String,
        token_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(CoinsResponse)]
    MinReservePrices {
        query_options: Option<QueryOptions<String>>,
    },
    #[returns(AuctionResponse)]
    Auction {
        collection: String,
        token_id: String,
    },
    #[returns(AuctionsResponse)]
    AuctionsBySeller {
        seller: String,
        query_options: Option<QueryOptions<(String, String)>>,
    },
    #[returns(AuctionsResponse)]
    AuctionsByEndTime {
        end_time: u64,
        query_options: Option<QueryOptions<(String, String)>>,
    },
}

/// QueryOptions are used to paginate contract queries
#[cw_serde]
#[derive(Default)]
pub struct QueryOptions<T> {
    /// Whether to sort items in ascending or descending order
    pub descending: Option<bool>,
    /// The key to start the query after
    pub start_after: Option<T>,
    // The number of items that will be returned
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct CoinsResponse {
    pub coins: Vec<Coin>,
}

#[cw_serde]
pub struct AuctionResponse {
    pub auction: Auction,
}

#[cw_serde]
pub struct AuctionsResponse {
    pub auctions: Vec<Auction>,
}

#[cw_serde]
pub enum SudoMsg {
    BeginBlock {}, // Is called by x/cron module BeginBlocker
    EndBlock {},   // Is called by x/cron module EndBlocker
    UpdateParams {
        fair_burn: Option<String>,
        marketplace: Option<String>,
        min_duration: Option<u64>,
        min_bid_increment_bps: Option<u64>,
        extend_duration: Option<u64>,
        create_auction_fee: Option<Coin>,
        max_auctions_to_settle_per_block: Option<u64>,
    },
    SetMinReservePrices {
        min_reserve_prices: Vec<Coin>,
    },
    UnsetMinReservePrices {
        denoms: Vec<String>,
    },
}
