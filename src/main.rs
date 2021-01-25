use std::fs::File;
use std::io::prelude::*;

use clap::Clap;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, Attr, color};

use near_jsonrpc_client::{get_account_in_pool, get_native_balance, get_liquid_owners_balance, get_locked_amount};
use primitives::Account;

mod near_jsonrpc_client;
mod configs;
mod primitives;
mod utils;

fn read_accounts(home_dir: std::path::PathBuf) -> Result<String, std::io::Error> {
    let accounts_list_path = home_dir.join("accounts.json");
    if !accounts_list_path.exists() {
        panic!("You must create ~/near_rewards/accounts.json with list of accounts to check. Example:\n\
        [\n  \
          {\n    \
            \"account_id\": \"accountid.near\",\n    \
            \"pool_account_id\": \"nameofpool.poolv1.near\"\n  \
          }\n\
        ]\n");
    }
    println!("Reading accounts from {}...", &accounts_list_path.to_string_lossy());
    let mut file = File::open(accounts_list_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: configs::Opts = configs::Opts::parse();

    let home_dir = opts
        .home_dir
        .unwrap_or_else(|| match dirs::home_dir() {
            Some(path) => path.join("near_rewards"),
            None => panic!("Unavailable to use default path ~/near_rewards/. Try to run `near_rewards --home-dir ~/near_rewards`"),
        });

    let accounts_file: Vec<Account> = match read_accounts(home_dir) {
        Ok(s) => serde_json::from_str(&s).unwrap(),
        Err(err) => {
            panic!("File read error: {}", err);
        }
    };
    let mut reward_sum= 0_u128;
    let mut liquid_balance_sum= 0_u128;
    let mut table = Table::new();
    table.add_row(row!["LOCKUP ACCOUNT", "REWARD", "LIQUID", "UNSTAKED", "NATIVE"]);
    println!("Fetching accounts data...");
    for account in accounts_file {
        let account_in_pool = match get_account_in_pool(account.clone().account_id, account.clone().pool_account_id).await {
            Ok(account) => account,
            Err(err) => {
                panic!("Error: {}", err);
            }
        };
        let locked_amount = match get_locked_amount(account.clone().account_id).await {
            Ok(amount) => amount,
            Err(err) => {
                panic!("Reqwest Error: {}", err);
            }
        };
        let native_balance = match get_native_balance(account.clone().account_id).await {
            Ok(amount) => amount,
            Err(err) => {
                panic!("Reqwest Error: {}", err);
            }
        };
        let liquid_balance = match get_liquid_owners_balance(account.clone().account_id).await {
            Ok(amount) => amount,
            Err(err) => {
                panic!("Reqwest Error: {}", err);
            }
        };
        let reward = account_in_pool.get_staked_balance() + account_in_pool.get_unstaked_balance() + native_balance - locked_amount;
        reward_sum += utils::human(reward);
        liquid_balance_sum += utils::human(liquid_balance);

        table.add_row(Row::new(vec![
            Cell::new(account.account_id.chars().take(14).collect::<String>().as_str())
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::WHITE)),
            Cell::new(utils::human(reward).to_string().as_str())
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new(utils::human(liquid_balance).to_string().as_str())
                .with_style(Attr::ForegroundColor(color::CYAN)),
            Cell::new(utils::human(account_in_pool.get_unstaked_balance())
                .to_string()
                .as_str()
            )
                .style_spec(if account_in_pool.can_withdraw { "Fy" } else { "Fr" }),
            Cell::new(utils::human(native_balance).to_string().as_str()),
        ]));
    }
    table.add_row(Row::new(vec![
            Cell::new(reward_sum.to_string().as_str())
                .with_hspan(2)
                .style_spec("br"),
            Cell::new(liquid_balance_sum.to_string().as_str())
                .with_hspan(3)
                .style_spec("b"),
    ]));
    let price = match utils::binance_price().await {
        Ok(v) => v,
        Err(_) => 0.0,
    };
    table.add_row(Row::new(vec![
        Cell::new(format!("${}", price * (reward_sum as f32)).as_str())
            .with_hspan(2)
            .style_spec("brFg"),
        Cell::new(format!("${}", price * (liquid_balance_sum as f32)).as_str())
            .with_hspan(3)
            .style_spec("bFc"),
    ]));
    table.printstd();
    Ok(())
}
