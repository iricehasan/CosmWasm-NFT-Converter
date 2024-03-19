import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";
import fs from "fs";

// Define the sender's private key
// const privateKey = "your_private_key_here";

// Create a signer object using the private key
// const wallet = await DirectSecp256k1Wallet.fromKey(privateKey);
// const mnemonic = 

// Create a wallet from the mnemonic
const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: "neutron",
});

// Initialize a CosmWasm client with the signer
const client = await SigningCosmWasmClient.connectWithSigner("https://rpc-palvus.pion-1.ntrn.tech", wallet, {
    gasPrice: GasPrice.fromString("0.025untrn"),
});

// Define the sender's address and the contract address
const [account] = await wallet.getAccounts();
const senderAddress = account.address;
console.log(senderAddress)

// deploy

const wasm= fs.readFileSync("/Users/macbookpro/Downloads/excalidraw/nft_converter/artifacts/nft_converter.wasm")
const result = await client.upload(senderAddress, wasm, "auto")
console.log(result)

// instantiate

const codeId = result.codeId; // 
const nftCodeId = 3471

//Define the instantiate message
const instantiateMsg = { "cw721_code_id": nftCodeId,"name": "Token", "symbol": "TOKEN", "admin": senderAddress}; // for the Converter contract
//Instantiate the contract
const instantiateResponse = await client.instantiate(senderAddress, codeId, instantiateMsg, "NFT Converter", "auto")
console.log(instantiateResponse)

const contractAddress = instantiateResponse.contractAddress // "neutron10y79hdq9mf74kgckduhe5nfvrhka49ftgutmny25htykkrxapyss9khgzs"
const queryNftAddress = client.queryContractSmart(contractAddress, {config: {}})
console.log(queryNftAddress)
// const contracts = await client.getContracts(nftCodeId2)
const nftcontractAddress = "neutron19rjgxw9ukccnets88few3df93jm2m7tqxv86k36l2668p9m7unns0mf6z5";

// Define the token ID and owner
const tokenId = "send_try"; 
const owner = senderAddress; // set the owner to the sender's address

const metadata = {
    name: "Token Name",
    description: "Token Description",
};

// use converter contractAddress to mint since it is the admin (minter)
const mintResult = await client.execute(senderAddress, contractAddress, {mint: {token_id: tokenId, extension: metadata, recipient: senderAddress}}, "auto")

const sendNft = client.execute(senderAddress, nftcontractAddress, {send_nft: {contract: contractAddress, msg: "" , token_id: tokenId}}, "auto" );

// confirm Converter contract has the ownership of the Nft after send
const TokensResponse = await client.queryContractSmart(nftcontractAddress, { tokens: { owner: contractAddress }});
console.log(TokensResponse)

// convert NFT
const new_metadata = {
    name: "New Token Name",
    description: "New Token Description",
};

const convertResult = client.execute(senderAddress, contractAddress, {convert: {token_id: tokenId, extension: new_metadata}}, "auto")
// confirm Nft is converted with new metadata and minted to the recipient which originally owned the NFT
const TokensResponse2 = await client.queryContractSmart(nftcontractAddress, { tokens: { owner: senderAddress }});
console.log(TokensResponse2)

// confirm the minted NFT has the updated Metadata
const nftInfo = await client.queryContractSmart(nftcontractAddress, {nft_info: {token_id: tokenId}});
console.log(nftInfo)

// query operations
const operationsResponse = client.queryContractSmart(contractAddress, {operations: {}})
console.log(operationsResponse)