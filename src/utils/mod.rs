use std::fs::File;
use std::io::prelude::*;

pub(crate) use accounts::{collect_account_data, reward_diff};
pub(crate) use binance::binance_price;
pub(crate) use human::{current_position_in_epoch, human};

mod accounts;
mod binance;
mod human;

pub(crate) fn read_accounts(home_dir: std::path::PathBuf) -> Result<String, std::io::Error> {
    let accounts_list_path = home_dir.join("accounts.json");
    if !accounts_list_path.exists() {
        panic!("{}", "You must create ~/near_rewards/accounts.json with list of accounts to check. Example:\n\
        [\n  \
          {\n    \
            \"account_id\": \"accountid.near\",\n    \
            \"pool_account_id\": \"nameofpool.poolv1.near\"\n  \
          }\n\
        ]\n");
    }
    println!(
        "Reading accounts from {}...",
        &accounts_list_path.to_string_lossy()
    );
    let mut file = File::open(accounts_list_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
