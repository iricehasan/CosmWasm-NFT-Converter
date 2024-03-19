# NFT Converter Functionality

1. While the converter contract is instantiated by setting an admin and with a code_id of the cw721-metadata-onchain NFT contract, a new NFT contract is also instantiated with this code_id so that the converter contract is the minter of the newly instantiated NFT contract.
2. Name, Symbol and Minter information are entered in the newly instantiated contract while instantiating the converter contract.
3. The admin of the converter contract can mint NFTs to recipients since it is the minter, so the converter contract address should be used when calling the mint function. Using the NFT contract’s address results in caller not the owner error.
4. The user sends the NFT he/she owns from the old contract to the converter contract with send Nft function.
5. Converter contract receives the NFT and records NFT-related information such as token_id, sender, and the nft contract address and saves them to a state (TOKEN_INFO).
6. The convert function in the converter contract burns the NFT in the old contract, and within the same function mints a new NFT with the desired metadata to the sender who sent the NFT by the same token_id in the newly instantiated NFT contract.

# Notes

- Only the admin of the converter contract can mint NFTs to recipients.
- The converter contract address should be used when calling the mint function.
- When instantiating the converter contract, the cw721_code_id should belong to a cw721-metadata-onchain contract to allow onchain metadata functionality.


# Messages

```rust
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


```