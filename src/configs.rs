use clap::Clap;

/// NEAR Rewards
/// Checks the rewards of lockup accounts
#[derive(Clap, Debug)]
#[clap(version = "0.6.0", author = "Bohdan Khorolets <b@khorolets.com>")]
pub(crate) struct Opts {
    /// Sets a custom near_rewards dir. Defaults to ~/near_rewards
    #[clap(short, long)]
    pub home_dir: Option<std::path::PathBuf>,
    #[clap(short, long)]
    pub verbose: bool,
    /// Provide a custom RPC server URL
    #[clap(long, default_value = "https://rpc.mainnet.near.org")]
    pub rpc_url: String,
}
