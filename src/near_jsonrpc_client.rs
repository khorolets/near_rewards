use serde_json::json;

use crate::primitives::{
    AccountInPoolResponse, AccountInPoolResult, Block, BlockResponse, Response, Status,
    StatusReponse, Validators, ValidatorsResponse, ViewAccountResponse,
};

pub(crate) async fn get_locked_amount(
    account_id: String,
    block_height: u64,
) -> Result<u128, reqwest::Error> {
    let params = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": json!({
            "request_type": "call_function",
            "block_id": block_height,
            "account_id": account_id,
            "method_name": "get_locked_amount",
            "args_base64": ""
        })
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://rpc.mainnet.internal.near.org")
        .json(&params)
        .send()
        .await?;

    let body: Response = res.json().await?;

    Ok(body.result.get_amount())
}

pub(crate) async fn get_liquid_owners_balance(
    account_id: String,
    block_height: u64,
) -> Result<u128, reqwest::Error> {
    let params = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": json!({
            "request_type": "call_function",
            "block_id": block_height,
            "account_id": account_id,
            "method_name": "get_liquid_owners_balance",
            "args_base64": ""
        })
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://rpc.mainnet.internal.near.org")
        .json(&params)
        .send()
        .await?;

    let body: Response = res.json().await?;

    Ok(body.result.get_amount())
}

pub(crate) async fn get_account_in_pool(
    account_id: String,
    pool_account_id: String,
    block_height: u64,
) -> Result<AccountInPoolResult, reqwest::Error> {
    let params = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": json!({
            "request_type": "call_function",
            "block_id": block_height,
            "account_id": pool_account_id,
            "method_name": "get_account",
            "args_base64": base64::encode(json!({"account_id": account_id}).to_string()),
        })
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://rpc.mainnet.internal.near.org")
        .json(&params)
        .send()
        .await?;

    let body: AccountInPoolResponse = res.json().await?;

    let account_in_pool: AccountInPoolResult =
        serde_json::from_slice(&body.result.result[..]).unwrap();

    Ok(account_in_pool)
}

pub(crate) async fn get_native_balance(
    account_id: String,
    block_height: u64,
) -> Result<u128, reqwest::Error> {
    let params = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": json!({
            "request_type": "view_account",
            "block_id": block_height,
            "account_id": account_id,
        })
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://rpc.mainnet.internal.near.org")
        .json(&params)
        .send()
        .await?;

    let body: ViewAccountResponse = res.json().await?;

    Ok(body.result.get_amount())
}

pub(crate) async fn get_status() -> Result<Status, reqwest::Error> {
    let params = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "status",
        "params": json!({}),
    });

    let client = reqwest::Client::new();

    let res = client
        .post("https://rpc.mainnet.internal.near.org")
        .json(&params)
        .send()
        .await?;

    let body: StatusReponse = res.json().await?;

    Ok(body.result)
}

pub(crate) async fn get_validators() -> Result<Validators, reqwest::Error> {
    let params = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "validators",
        "params": json!({"latest": null}),
    });

    let client = reqwest::Client::new();

    let res = client
        .post("https://rpc.mainnet.internal.near.org")
        .json(&params)
        .send()
        .await?;

    let body: ValidatorsResponse = res.json().await?;

    Ok(body.result)
}

pub(crate) async fn get_block(block_height: u64) -> Result<Block, reqwest::Error> {
    let params = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "block",
        "params": json!({"block_id": block_height}),
    });

    let client = reqwest::Client::new();

    let res = client
        .post("https://rpc.mainnet.internal.near.org")
        .json(&params)
        .send()
        .await?;

    let body: BlockResponse = res.json().await?;

    Ok(body.result)
}
