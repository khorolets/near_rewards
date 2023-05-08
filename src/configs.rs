use clap::Parser;

/// NEAR Rewards
/// Checks the rewards of lockup accounts
#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
pub(crate) struct Opts {
    /// Sets a custom near_rewards dir. Defaults to ~/near_rewards
    #[arg(long)]
    pub home_dir: Option<std::path::PathBuf>,
    #[arg(short, long)]
    pub verbose: bool,
    /// Provide a custom RPC server URL
    #[arg(long, default_value = "https://rpc.mainnet.near.org")]
    pub rpc_url: String,
}
