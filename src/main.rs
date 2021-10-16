use std::collections::HashSet;

use clap::Clap;

#[macro_use]
extern crate prettytable;
use prettytable::{color, Attr, Cell, Row, Table};

use near_jsonrpc_client::{get_block, get_final_block, get_validators};
use primitives::Account;
use utils::{collect_account_data, reward_diff};

mod configs;
mod near_jsonrpc_client;
mod primitives;
mod utils;

const EPOCH_LENGTH: u64 = 43200;

fn print_table(table: &Table, reward_sum: f64, liquid_balance_sum: f64, price: f32) {
    let mut table = table.clone();

    table.add_row(Row::new(vec![
        Cell::new(format!("{:.2}", (reward_sum as f32)).as_str())
            .with_hspan(2)
            .style_spec("brFg"),
        Cell::new(format!("{:.2}", (liquid_balance_sum as f32)).as_str())
            .with_hspan(4)
            .style_spec("bFc"),
    ]));

    table.add_row(Row::new(vec![
        Cell::new(format!("${:.2}", price * (reward_sum as f32)).as_str())
            .with_hspan(2)
            .style_spec("brFg"),
        Cell::new(format!("${:.2}", price * (liquid_balance_sum as f32)).as_str())
            .with_hspan(4)
            .style_spec("bFc"),
    ]));

    table.printstd();
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

    let mut accounts_file: Vec<Account> = match utils::read_accounts(home_dir) {
        Ok(s) => serde_json::from_str(&s).unwrap(),
        Err(err) => {
            panic!("File read error: {}", err);
        }
    };

    let current_block = match get_final_block().await {
        Ok(block) => block,
        Err(err) => panic!("Error: {}", err),
    };

    let epoch_start_height = match get_validators().await {
        Ok(validators) => validators.epoch_start_height,
        Err(err) => panic!("Error: {}", err),
    };

    // TODO: Improve this, we may end up in missing block so we want
    // somehow to increment the amount of block we subtract from epoch_start_height
    let prev_epoch_block = match get_block(epoch_start_height - 6).await {
        Ok(block) => block,
        Err(err) => panic!("Error: {}", err),
    };

    let current_position_in_epoch =
        utils::current_position_in_epoch(epoch_start_height, current_block.header.height);

    let mut reward_sum = 0_f64;
    let mut liquid_balance_sum = 0_f64;

    let price = match utils::binance_price().await {
        Ok(v) => v,
        Err(_) => 0.0,
    };

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new(format!("Epoch progress: {}%", current_position_in_epoch).as_str()).with_hspan(2),
        Cell::new("NEAR-USDT (Binance)").with_hspan(2),
        Cell::new(format!("${}", price).as_str())
            .with_hspan(1)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));

    table.add_row(row![
        "LOCKUP ACCOUNT",
        "REWARD",
        "LIQUID",
        "UNSTAKED",
        "NATIVE",
        "POOL",
    ]);

    println!("Fetching accounts data...");

    let mut alredy_fetched_liquid_balance_accounts: HashSet<String> = HashSet::new();

    accounts_file.sort_by(|a, b| a.key.cmp(&b.key));

    for mut account in accounts_file {
        let account_at_current_block =
            collect_account_data(&mut account, current_block.clone()).await;

        let account_at_prev_epoch =
            collect_account_data(&mut account, prev_epoch_block.clone()).await;

        reward_sum += utils::human(account_at_current_block.reward);

        let liquid_balance = if alredy_fetched_liquid_balance_accounts
            .get(&account.account_id)
            .is_none()
        {
            alredy_fetched_liquid_balance_accounts.insert(account.account_id.clone());
            account_at_current_block.liquid_balance
        } else {
            0
        };

        liquid_balance_sum += utils::human(liquid_balance);

        table.add_row(Row::new(vec![
            Cell::new(
                account
                    .account_id
                    .chars()
                    .take(14)
                    .collect::<String>()
                    .as_str(),
            )
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::WHITE)),
            Cell::new(&format!(
                "{} {}",
                utils::current_reward(account_at_current_block.reward),
                &reward_diff(
                    account_at_current_block.reward,
                    account_at_prev_epoch.reward,
                ),
            )),
            Cell::new(&format!(
                "{:.2}",
                utils::human(account_at_current_block.liquid_balance)
            ))
            .with_style(Attr::ForegroundColor(color::CYAN)),
            Cell::new(&format!(
                "{:.2}",
                utils::human(
                    account_at_current_block
                        .account_in_pool
                        .get_unstaked_balance(),
                )
            ))
            .style_spec(if account_at_current_block.account_in_pool.can_withdraw {
                "Fy"
            } else {
                "Fr"
            }),
            Cell::new(&format!(
                "{:.2}",
                utils::human(account_at_current_block.native_balance)
            )),
            Cell::new(account.get_pool_account_id().await.unwrap().as_str()),
        ]));
        if opts.verbose {
            print_table(&table, reward_sum, liquid_balance_sum, price);
        }
    }
    if !opts.verbose {
        print_table(&table, reward_sum, liquid_balance_sum, price);
    }
    Ok(())
}
