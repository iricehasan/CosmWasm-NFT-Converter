use std::vec;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_json, to_json_binary, wasm_instantiate, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response, StdResult, SubMsg, Uint128, WasmMsg
};
use cw721::{ApprovalResponse, Cw721ExecuteMsg, Cw721QueryMsg, Cw721ReceiveMsg};
use cw721_base::InstantiateMsg as Cw721InstantaiteMsg;

use cw_utils::parse_reply_instantiate_data;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InnerMsg, InstantiateMsg, QueryMsg, OperationsResponse, ConfigResponse, TokenInfoResponse, Metadata};
use crate::state::{Config, CONFIG, TokenInfo, TOKEN_INFO, OPERATIONS};

pub const INSTANTIATE_REPLY: u64 = 1;
pub const BURN_REPLY: u64 = 2;
pub const MINT_REPLY: u64 = 3;

pub type Extension = Option<Metadata>;
pub type Cw721BaseExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // the contract instantiated should be a metadata-onchain contract
    let cw721_init_msg = Cw721InstantaiteMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: env.contract.address.to_string(),
    };

    let submsg = SubMsg::reply_on_success(
        wasm_instantiate(
            msg.cw721_code_id,
            &cw721_init_msg,
            vec![],
            "Nft Contract".to_owned(),
        )
        .unwrap(),
        INSTANTIATE_REPLY,
    );

    let config = Config {
        nft_addr: Addr::unchecked(""),
        admin:  deps.api.addr_validate(&msg.admin)?,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_submessage(submsg)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::ReceiveNft(receive_msg) => execute_receive(deps, env, info, receive_msg),
        ExecuteMsg::Convert {token_id, extension, token_uri} => execute_convert(deps, env, info, token_id, extension, token_uri),
        ExecuteMsg::Mint { token_id, recipient, extension, token_uri} => execute_mint(deps, env, info, token_id, recipient, extension, token_uri),
    }
}

pub fn execute_mint(
    deps: DepsMut, 
    _env: Env,
    info: MessageInfo,
    token_id: String,
    recipient: String,
    extension: Option<Metadata>,
    token_uri: Option<String>,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    };

    let submsg_mint = SubMsg::reply_on_success(WasmMsg::Execute {
        contract_addr: config.nft_addr.clone().to_string(),
        msg: to_json_binary(&Cw721BaseExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: recipient,
            token_uri: token_uri,
            extension: extension,
        })?,
        funds: vec![],
        },
        MINT_REPLY,
    );

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_submessage(submsg_mint))
}

pub fn execute_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receive_msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {

    // info.sender is the NFT contract Address
    let sender = receive_msg.sender.clone();
    let nft_addr = info.sender.clone().to_string();

    
    let inner_msg: InnerMsg = from_json(&receive_msg.msg)?;

    // ensure contract have approval
    let _: ApprovalResponse = deps
        .querier
        .query_wasm_smart(
            nft_addr.clone(),
            &Cw721QueryMsg::Approval {
                token_id: receive_msg.token_id.clone().to_string(),
                spender: env.contract.address.to_string(),
                include_expired: None,
            },
        )
        .unwrap();

    match inner_msg {
        InnerMsg::Succeed => {
            let new_nft_info = TokenInfo {
                token_id: receive_msg.token_id.clone(),
                nft_addr,
                sender,
            };
        
            TOKEN_INFO.save(deps.storage, &receive_msg.token_id.clone().as_bytes(), &new_nft_info)?;

            Ok(Response::new()
                .add_attributes([
                    ("action", "receive_nft"),
                    ("token_id", receive_msg.token_id.as_str()),
                    ("sender", receive_msg.sender.as_str()),
                    ("msg", receive_msg.msg.to_base64().as_str()),
                ])
                .set_data(
                    [
                        receive_msg.token_id,
                        receive_msg.sender,
                        receive_msg.msg.to_base64(),
                    ]
                    .concat()
                    .as_bytes(),
                ))},
        InnerMsg::Fail => Err(ContractError::Failed {}),
    }
}

pub fn execute_convert(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    token_id: String,
    extension: Option<Metadata>,
    token_uri: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let token_info = TOKEN_INFO.load(deps.storage, &token_id.clone().as_bytes())?;

    let submsg_burn = SubMsg::reply_on_success(
        WasmMsg::Execute {
            contract_addr: token_info.nft_addr.clone(),
            msg: to_json_binary(&Cw721ExecuteMsg::Burn {
                token_id: token_info.token_id.clone().to_string(),
            })?,
            funds: vec![],
        },
        BURN_REPLY,
    );

    let submsg_mint = SubMsg::reply_on_success(WasmMsg::Execute {
        contract_addr: config.nft_addr.clone().to_string(),
        msg: to_json_binary(&Cw721BaseExecuteMsg::Mint {
            token_id: token_info.token_id.to_string(),
            owner: token_info.sender.to_string(),
            token_uri: token_uri,
            extension: extension,
        })?,
        funds: vec![],
        },
        MINT_REPLY,
    );

    TOKEN_INFO.remove(deps.storage, &token_id.as_bytes());

    Ok(Response::new()
        .add_attribute("action","convert")
        .add_submessages(vec![submsg_burn, submsg_mint]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    let mut ops = OPERATIONS.load(deps.storage).unwrap_or_default();
    match reply.id {
        INSTANTIATE_REPLY => {
            let res = parse_reply_instantiate_data(reply).unwrap();
            let mut config = CONFIG.load(deps.storage)?;
            let nft_contract = deps.api.addr_validate(&res.contract_address).unwrap();
            config.nft_addr = nft_contract;
            CONFIG.save(deps.storage, &config)?;
            Ok(Response::default())
        }
        BURN_REPLY => {
            ops.n_burns += Uint128::one();
            OPERATIONS.save(deps.storage, &ops)?;

            Ok(Response::new().add_attribute("Operation", "burn"))
        }
        MINT_REPLY => {
            ops.n_mints += Uint128::one();
            OPERATIONS.save(deps.storage, &ops)?;

            Ok(Response::new().add_attribute("Operation", "mint"))
        }
        _ => Err(ContractError::UnrecognizedReply {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::Operations {} => to_json_binary(&query_operations(deps)?),
        QueryMsg::TokenInfo { token_id } => to_json_binary(&query_nft_info(deps, token_id)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        nft_addr: config.nft_addr.to_string(),
    })
}
fn query_operations(deps: Deps) -> StdResult<OperationsResponse> {
    let ops = OPERATIONS.load(deps.storage)?;
    Ok(OperationsResponse {
        n_burns: ops.n_burns,
        n_mints: ops.n_mints,
    })
}

fn query_nft_info(deps: Deps, token_id: String) -> StdResult<TokenInfoResponse> {
    let token_info = TOKEN_INFO.load(deps.storage, &token_id.clone().as_bytes())?;
    Ok( TokenInfoResponse {
        token_id,
        nft_addr: token_info.nft_addr,
        sender: token_info.sender,
    })
}