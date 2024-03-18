use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Uint128, Addr};

#[cw_serde]
pub struct Config {
    pub nft_addr: Addr,
    pub admin: Addr,
}

#[cw_serde]
#[derive(Default)]
pub struct Operations {
    pub n_burns: Uint128,
    pub n_mints: Uint128,
}

#[cw_serde]
pub struct TokenInfo {
    pub token_id: String,
    pub nft_addr: String,
    pub sender: String,
}

pub const OPERATIONS: Item<Operations> = Item::new("operations");
pub const CONFIG: Item<Config> = Item::new("config");
pub const TOKEN_INFO: Map<&[u8],TokenInfo> = Map::new("nft_info");