# near_rewards

Small console application to check the staking rewards for lockup accounts on NEAR protocol.

## Example result:

![near_reward result example](docs/near_rewards.png)

## Prerequisites

This utility requires Rust. To install, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

([Official documentation](https://www.rust-lang.org/tools/install))

Follow the directions which includes running:

```bash
source $HOME/.cargo/env
```

## Usage

1. Create a `near_rewards` folder inside your home directory.

2. In `near_rewards` create a file `accounts.json` with the following structure:

```json
[
  {
    "account_id": "account.lockup.near",
    "pool_account_id": "pool.poolv1.near"
  }
]
```

_**Note:** This tool only works for lockup accounts._

3. Run `cargo run` in your terminal.
