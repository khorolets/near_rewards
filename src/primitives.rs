use crate::near_jsonrpc_client::get_staking_pool_account_id;
use borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{self, Deserialize};

#[derive(Debug, Deserialize, BorshDeserialize, Clone)]
pub(crate) struct Response {
    pub result: ResponseResult,
}

#[derive(Debug, Deserialize, BorshDeserialize, Clone)]
pub(crate) struct ResponseResult {
    pub block_hash: String,
    pub block_height: u64,
    pub result: Vec<u8>,
}

impl ResponseResult {
    pub fn get_amount(self) -> u128 {
        String::from_utf8(self.result)
            .unwrap()
            .trim_matches('"')
            .parse::<u128>()
            .unwrap()
    }
}

#[derive(Debug, Deserialize, BorshDeserialize)]
pub(crate) struct ViewAccountResponse {
    pub result: ViewAccountResult,
}

#[derive(Debug, Deserialize, BorshDeserialize)]
pub(crate) struct ViewAccountResult {
    pub amount: String,
}

impl ViewAccountResult {
    pub fn get_amount(self) -> u128 {
        self.amount.trim_matches('"').parse::<u128>().unwrap()
    }
}

#[derive(Debug, Deserialize, BorshDeserialize)]
pub(crate) struct AccountInPoolResponse {
    pub result: ResponseResult,
}

#[derive(Debug, Deserialize, BorshSerialize, BorshDeserialize)]
pub(crate) struct AccountInPoolResult {
    pub account_id: String,
    pub unstaked_balance: String,
    pub staked_balance: String,
    pub can_withdraw: bool,
}

impl AccountInPoolResult {
    pub fn get_staked_balance(&self) -> u128 {
        self.staked_balance
            .trim_matches('"')
            .parse::<u128>()
            .unwrap()
    }

    pub fn get_unstaked_balance(&self) -> u128 {
        self.unstaked_balance
            .trim_matches('"')
            .parse::<u128>()
            .unwrap()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Account {
    /// Value used to identify the account.
    pub key: Option<String>,
    pub account_id: String,
    pub pool_account_id: Option<String>,
    pub locked_amount: Option<String>,
}

impl Account {
    pub async fn get_pool_account_id(&mut self) -> Option<String> {
        if self.pool_account_id.is_none() {
            self.pool_account_id = get_staking_pool_account_id(self.account_id.clone())
                .await
                .ok();
        }
        self.pool_account_id.clone()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ValidatorsResponse {
    pub result: Validators,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Validators {
    pub epoch_start_height: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BlockResponse {
    pub result: Block,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Block {
    pub header: BlockHeader,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct BlockHeader {
    pub height: u64,
    pub hash: String,
    pub epoch_id: String,
}

#[derive(Debug)]
pub(crate) struct AccountBalancesAtBlock {
    pub block: Block,
    pub account: Account,
    pub account_in_pool: AccountInPoolResult,
    pub native_balance: u128,
    pub liquid_balance: u128,
    pub reward: u128,
}
