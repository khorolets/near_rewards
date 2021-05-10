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
    pub account_id: String,
    pub pool_account_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StatusReponse {
    pub result: Status,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Status {
    pub sync_info: SyncInfo,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SyncInfo {
    pub latest_block_height: u64,
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

#[derive(Debug, Deserialize)]
pub(crate) struct Block {
    pub header: BlockHeader,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BlockHeader {
    pub height: u64,
    pub hash: String,
    pub epoch_id: String,
}
