use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw721::Cw721ReceiveMsg;

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub cw721_code_id: u64,
    pub name: String,
    pub symbol: String,
    pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    Convert { 
        token_id: String,
        extension: Option<Metadata>,
        token_uri: Option<String>,
     },
     Mint {
        token_id: String,
        recipient: String,
        extension: Option<Metadata>,
        token_uri: Option<String>,
     }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OperationsResponse)]
    Operations {},
    #[returns(ConfigResponse)]
    Config {},
    #[returns(TokenInfoResponse)]
    TokenInfo { token_id: String }
}

#[cw_serde]
pub struct OperationsResponse {
    pub n_burns: Uint128,
    pub n_mints: Uint128,
}

#[cw_serde]
pub struct ConfigResponse {
    pub nft_addr: String,
}

#[cw_serde]
pub struct TokenInfoResponse {
    pub token_id: String,
    pub nft_addr: String,
    pub sender: String,
}