use colored::*;

use crate::near_jsonrpc_client::{
    get_account_in_pool, get_liquid_owners_balance, get_locked_amount, get_native_balance,
};
use crate::primitives::{Account, AccountBalancesAtBlock, Block};
use crate::utils;

pub(crate) async fn collect_account_data(
    account: &mut Account,
    block: Block,
) -> AccountBalancesAtBlock {
    let account_in_pool = match get_account_in_pool(
        account.clone().account_id,
        account.get_pool_account_id().await.unwrap(),
        block.header.height,
    )
    .await
    {
        Ok(account) => account,
        Err(err) => {
            panic!("Error: {}", err);
        }
    };
    let locked_amount: u128 =
        match get_locked_amount(account.clone().account_id, block.header.height).await {
            Ok(amount) => amount,
            Err(_err) => 0,
        };
    let native_balance =
        match get_native_balance(account.clone().account_id, block.header.height).await {
            Ok(amount) => amount,
            Err(err) => {
                panic!("Reqwest Error: {}", err);
            }
        };
    let liquid_balance =
        match get_liquid_owners_balance(account.clone().account_id, block.header.height).await {
            Ok(amount) => amount,
            Err(_err) => native_balance,
        };
    let reward = account_in_pool.get_staked_balance()
        + account_in_pool.get_unstaked_balance()
        + if locked_amount > 0 { native_balance } else { 0 }
        - locked_amount;

    AccountBalancesAtBlock {
        block,
        account: account.clone(),
        account_in_pool,
        native_balance,
        liquid_balance,
        reward,
    }
}

pub(crate) fn reward_diff(current_reward: u128, prev_reward: u128) -> String {
    if current_reward > prev_reward {
        return format!("+{:.2}", utils::human(current_reward - prev_reward))
            .blue()
            .to_string();
    } else {
        return format!("-{:.2}", utils::human(prev_reward - current_reward))
            .red()
            .to_string();
    }
}

pub(crate) fn current_reward(current_reward: u128) -> String {
    return format!("{:.2}", utils::human(current_reward))
        .green()
        .to_string();
}
